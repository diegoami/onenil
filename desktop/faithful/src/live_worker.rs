//! [designed:T76] Bounded local transport for one retained native campaign image.
use crate::{
    campaign_match,
    timeline::{Completion, IntervalTimeline, LiveCampaign},
};
use onenil_campaign::Campaign;
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::{
    io::{self, BufReader, Read, Write},
    path::Path,
    process::{Child, ChildStdin, Command, Stdio},
    sync::mpsc::{self, Receiver, TryRecvError},
    thread::{self, JoinHandle},
};
const VERSION: u16 = 1;
const REQUEST_LIMIT: usize = 1024 * 1024;
const PACKET_LIMIT: usize = 32 * 1024 * 1024;
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum Decision {
    Continue,
    ReplaceVerified,
}
#[derive(Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct Start {
    version: u16,
    id: u64,
    campaign: serde_json::Value,
}
#[derive(Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct Choice {
    version: u16,
    id: u64,
    identity: String,
    sequence: u8,
    decision: Decision,
}
#[derive(Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct Packet {
    version: u16,
    id: u64,
    identity: String,
    sequence: u8,
    body: Body,
}
#[derive(Serialize, Deserialize)]
#[serde(
    tag = "kind",
    content = "data",
    rename_all = "snake_case",
    deny_unknown_fields
)]
enum Body {
    Interval(IntervalTimeline),
    Complete(Completion),
    Failed(String),
}
fn identity(campaign: &Campaign) -> Result<String, String> {
    Ok(format!("{:x}", Sha256::digest(campaign.encode()?)))
}
fn write_frame<W: Write, T: Serialize>(
    writer: &mut W,
    value: &T,
    limit: usize,
) -> Result<(), String> {
    let bytes = serde_json::to_vec(value).map_err(|e| e.to_string())?;
    if bytes.is_empty() || bytes.len() > limit {
        return Err("live worker frame exceeds limit".into());
    }
    writer
        .write_all(&(bytes.len() as u32).to_le_bytes())
        .and_then(|_| writer.write_all(&bytes))
        .and_then(|_| writer.flush())
        .map_err(|e| e.to_string())
}
fn read_frame<R: Read, T: DeserializeOwned>(
    reader: &mut R,
    limit: usize,
) -> Result<Option<T>, String> {
    let mut header = [0; 4];
    match reader.read_exact(&mut header[..1]) {
        Err(e) if e.kind() == io::ErrorKind::UnexpectedEof => return Ok(None),
        Err(e) => return Err(e.to_string()),
        Ok(()) => (),
    }
    reader
        .read_exact(&mut header[1..])
        .map_err(|e| format!("truncated live frame header: {e}"))?;
    let length = u32::from_le_bytes(header) as usize;
    if length == 0 || length > limit {
        return Err("live worker frame exceeds limit".into());
    }
    let mut bytes = vec![0; length];
    reader
        .read_exact(&mut bytes)
        .map_err(|e| format!("truncated live frame body: {e}"))?;
    serde_json::from_slice(&bytes)
        .map(Some)
        .map_err(|e| e.to_string())
}
fn require_eof<R: Read>(reader: &mut R) -> Result<(), String> {
    let mut extra = [0];
    if reader.read(&mut extra).map_err(|e| e.to_string())? != 0 {
        return Err("trailing or duplicate live worker frame".into());
    }
    Ok(())
}
/// Child entrypoint. Game inputs come only from the validated owned campaign.
pub fn run(id: u64) -> Result<(), String> {
    let mut reader = BufReader::new(io::stdin().lock());
    let mut writer = io::stdout().lock();
    let mut binding = String::new();
    let mut sequence = 0;
    let result = (|| {
        let start: Start =
            read_frame(&mut reader, REQUEST_LIMIT)?.ok_or("missing live campaign request")?;
        if start.version != VERSION || start.id != id {
            return Err("live request identity/version differs".into());
        }
        let campaign =
            Campaign::decode(&serde_json::to_vec(&start.campaign).map_err(|e| e.to_string())?)?;
        binding = identity(&campaign)?;
        let mut session = LiveCampaign::new(&campaign)?;
        let prefix = session.pause()?;
        write_frame(
            &mut writer,
            &Packet {
                version: VERSION,
                id,
                identity: binding.clone(),
                sequence,
                body: Body::Interval(prefix),
            },
            PACKET_LIMIT,
        )?;
        sequence = 1;
        let choice: Choice =
            read_frame(&mut reader, REQUEST_LIMIT)?.ok_or("missing live interval decision")?;
        if choice.version != VERSION
            || choice.id != id
            || choice.identity != binding
            || choice.sequence != sequence
        {
            return Err("live decision identity/phase differs".into());
        }
        // Parent closes the command pipe after one decision. Refuse queued extras
        // before consuming the engine, rather than executing one and ignoring more.
        require_eof(&mut reader)?;
        let complete = session.finish(choice.decision == Decision::ReplaceVerified)?;
        write_frame(
            &mut writer,
            &Packet {
                version: VERSION,
                id,
                identity: binding.clone(),
                sequence,
                body: Body::Complete(complete),
            },
            PACKET_LIMIT,
        )
    })();
    if let Err(error) = result {
        write_frame(
            &mut writer,
            &Packet {
                version: VERSION,
                id,
                identity: binding,
                sequence,
                body: Body::Failed(error),
            },
            PACKET_LIMIT,
        )?;
    }
    Ok(())
}
enum Wire {
    Packet(Packet),
    End(Result<(), String>),
}
#[derive(Clone, Copy, PartialEq, Eq)]
enum Phase {
    Starting,
    Interval,
    Finishing,
    Terminal,
}
pub enum Poll {
    Pending,
    Interval(u64, IntervalTimeline),
    Ready(u64, Completion),
    Failed(u64, String),
}
pub struct LiveJob {
    id: u64,
    identity: String,
    child: Child,
    input: Option<ChildStdin>,
    receiver: Receiver<Wire>,
    reader: Option<JoinHandle<()>>,
    phase: Phase,
    next_sequence: u8,
    prefix: Option<IntervalTimeline>,
    complete: Option<Completion>,
    failure: Option<String>,
    reader_done: bool,
    reaped: bool,
    replacement_allowed: bool,
    expected_clubs: [u64; 2],
    expected_names: [String; 2],
}
impl LiveJob {
    pub fn spawn_campaign(executable: &Path, id: u64, campaign: &Campaign) -> Result<Self, String> {
        let replacement_allowed =
            campaign_match::profile(campaign)? == campaign_match::Profile::Original;
        let binding = identity(campaign)?;
        let request = Start {
            version: VERSION,
            id,
            campaign: serde_json::from_slice(&campaign.encode()?).map_err(|e| e.to_string())?,
        };
        let mut child = Command::new(executable)
            .arg("--live-campaign-worker")
            .arg(id.to_string())
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::null())
            .spawn()
            .map_err(|e| e.to_string())?;
        let pipes = (|| {
            let mut input = child.stdin.take().ok_or("missing live input pipe")?;
            let stdout = child.stdout.take().ok_or("missing live output pipe")?;
            write_frame(&mut input, &request, REQUEST_LIMIT)?;
            Ok::<_, String>((input, stdout))
        })();
        let (input, stdout) = match pipes {
            Ok(p) => p,
            Err(e) => {
                let _ = child.kill();
                let _ = child.wait();
                return Err(e);
            }
        };
        let (sender, receiver) = mpsc::channel();
        let reader = thread::spawn(move || {
            let mut stdout = BufReader::new(stdout);
            let result = (|| {
                // At most two packet messages plus EOF: channel memory is bounded
                // without a blocking sender that could deadlock cancellation.
                for _ in 0..2 {
                    match read_frame(&mut stdout, PACKET_LIMIT)? {
                        Some(packet) => {
                            if sender.send(Wire::Packet(packet)).is_err() {
                                return Ok(());
                            }
                        }
                        None => return Ok(()),
                    }
                }
                require_eof(&mut stdout)
            })();
            let _ = sender.send(Wire::End(result));
        });
        Ok(Self {
            id,
            identity: binding,
            child,
            input: Some(input),
            receiver,
            reader: Some(reader),
            phase: Phase::Starting,
            next_sequence: 0,
            prefix: None,
            complete: None,
            failure: None,
            reader_done: false,
            reaped: false,
            replacement_allowed,
            expected_clubs: [
                u64::from(campaign.club.id),
                u64::from(campaign.next_fixture.opponent),
            ],
            expected_names: [
                campaign.club.name.clone(),
                campaign.next_fixture.opponent_name.clone(),
            ],
        })
    }
    pub fn pid(&self) -> u32 {
        self.child.id()
    }
    pub fn reaped(&self) -> bool {
        self.reaped
    }
    pub fn decide(&mut self, decision: Decision) -> Result<(), String> {
        if self.phase != Phase::Interval {
            return Err("no pending live interval decision".into());
        }
        if decision == Decision::ReplaceVerified && !self.replacement_allowed {
            return Err("replacement is supported only in the verified original campaign".into());
        }
        let mut input = self.input.take().ok_or("live command pipe is closed")?;
        let result = write_frame(
            &mut input,
            &Choice {
                version: VERSION,
                id: self.id,
                identity: self.identity.clone(),
                sequence: 1,
                decision,
            },
            REQUEST_LIMIT,
        );
        drop(input);
        if let Err(error) = result {
            let _ = self.cancel();
            return Err(error);
        }
        self.phase = Phase::Finishing;
        Ok(())
    }
    fn reject(&mut self, error: String) {
        if self.failure.is_none() {
            self.failure = Some(error);
            self.input.take();
            let _ = self.child.kill();
        }
    }
    fn packet(&mut self, packet: Packet) -> Result<Option<IntervalTimeline>, String> {
        if packet.version != VERSION
            || packet.id != self.id
            || packet.identity != self.identity
            || packet.sequence != self.next_sequence
        {
            return Err("live output identity/sequence differs".into());
        }
        self.next_sequence += 1;
        match packet.body {
            Body::Interval(prefix) if self.phase == Phase::Starting => {
                prefix.validate()?;
                if prefix.club_names != self.expected_names
                    || prefix.clubs.len() != 2
                    || !self.expected_clubs.iter().all(|c| prefix.clubs.contains(c))
                {
                    return Err("live interval fixture differs from request".into());
                }
                if let Some(_) = self.child.try_wait().map_err(|e| e.to_string())? {
                    self.reaped = true;
                    return Err("live worker exited before its decision".into());
                }
                self.prefix = Some(prefix.clone());
                self.phase = Phase::Interval;
                Ok(Some(prefix))
            }
            Body::Complete(complete) if self.phase == Phase::Finishing => {
                complete.timeline.validate_markers()?;
                if !self
                    .prefix
                    .as_ref()
                    .is_some_and(|p| p.is_prefix_of(&complete.timeline))
                    || complete.native_sha256.len() != 64
                    || !complete
                        .native_sha256
                        .bytes()
                        .all(|b| b.is_ascii_hexdigit())
                {
                    return Err("live completion differs from owned prefix".into());
                }
                self.complete = Some(complete);
                Ok(None)
            }
            Body::Failed(error) => Err(error),
            _ => Err("live output outside expected phase".into()),
        }
    }
    pub fn poll(&mut self) -> Poll {
        if self.phase == Phase::Terminal {
            return Poll::Pending;
        }
        loop {
            match self.receiver.try_recv() {
                Ok(Wire::Packet(packet)) => match self.packet(packet) {
                    Ok(Some(prefix)) => return Poll::Interval(self.id, prefix),
                    Ok(None) => (),
                    Err(error) => self.reject(error),
                },
                Ok(Wire::End(result)) => {
                    self.reader_done = true;
                    if let Err(error) = result {
                        self.reject(error);
                    }
                }
                Err(TryRecvError::Empty) => break,
                Err(TryRecvError::Disconnected) => {
                    if !self.reader_done {
                        self.reader_done = true;
                        self.reject("live reader disconnected".into());
                    }
                    break;
                }
            }
        }
        if !self.reaped {
            match self.child.try_wait() {
                Ok(Some(status)) => {
                    self.reaped = true;
                    if !status.success() {
                        self.reject("live worker exited unsuccessfully".into());
                    }
                }
                Ok(None) => (),
                Err(error) => self.reject(error.to_string()),
            }
        }
        if self.reaped && self.reader_done {
            if let Some(reader) = self.reader.take() {
                if reader.join().is_err() {
                    self.reject("live reader panic".into());
                }
            }
            self.phase = Phase::Terminal;
            if let Some(error) = self.failure.take() {
                return Poll::Failed(self.id, error);
            }
            if let Some(complete) = self.complete.take() {
                return Poll::Ready(self.id, complete);
            }
            return Poll::Failed(self.id, "live worker ended without completion".into());
        }
        Poll::Pending
    }
    pub fn cancel(&mut self) -> Result<(), String> {
        self.input.take();
        if !self.reaped {
            if self.child.try_wait().map_err(|e| e.to_string())?.is_none() {
                if let Err(error) = self.child.kill() {
                    if self.child.try_wait().map_err(|e| e.to_string())?.is_none() {
                        return Err(error.to_string());
                    }
                }
                self.child.wait().map_err(|e| e.to_string())?;
            }
            self.reaped = true;
        }
        if let Some(reader) = self.reader.take() {
            reader.join().map_err(|_| "live reader panic")?;
        }
        self.phase = Phase::Terminal;
        Ok(())
    }
}
impl Drop for LiveJob {
    fn drop(&mut self) {
        let _ = self.cancel();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;
    #[test]
    fn framing_rejects_partial_oversized_trailing_and_unknown_payloads() {
        let mut bytes = vec![];
        write_frame(
            &mut bytes,
            &Choice {
                version: 1,
                id: 7,
                identity: "x".into(),
                sequence: 1,
                decision: Decision::Continue,
            },
            REQUEST_LIMIT,
        )
        .unwrap();
        assert!(
            read_frame::<_, Choice>(&mut Cursor::new(&bytes), REQUEST_LIMIT)
                .unwrap()
                .is_some()
        );
        for bad in [
            bytes[..2].to_vec(),
            bytes[..bytes.len() - 1].to_vec(),
            u32::MAX.to_le_bytes().to_vec(),
            0u32.to_le_bytes().to_vec(),
        ] {
            assert!(read_frame::<_, Choice>(&mut Cursor::new(bad), REQUEST_LIMIT).is_err());
        }
        let extra = serde_json::json!({"version":1,"id":7,"identity":"x","sequence":1,"decision":"continue","expected_state":{}});
        let mut bad = vec![];
        write_frame(&mut bad, &extra, REQUEST_LIMIT).unwrap();
        assert!(read_frame::<_, Choice>(&mut Cursor::new(bad), REQUEST_LIMIT).is_err());
        assert!(require_eof(&mut Cursor::new([1])).is_err());
        assert!(read_frame::<_, Choice>(&mut Cursor::new([]), REQUEST_LIMIT)
            .unwrap()
            .is_none());
    }
}
