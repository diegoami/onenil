//! [designed] One app owns each child. Cancellation kills and reaps before replacement.
use crate::timeline::{self, Timeline};
use serde::{Deserialize, Serialize};
use std::{
    io::{Read, Write},
    path::Path,
    process::{Child, Command, Stdio},
    sync::mpsc::{self, Receiver, TryRecvError},
    thread::{self, JoinHandle},
};
#[derive(Serialize, Deserialize)]
struct Packet {
    id: u64,
    result: Result<Timeline, String>,
}
pub enum Poll {
    Pending,
    Ready(u64, Timeline),
    Failed(u64, String),
}
pub struct Job {
    id: u64,
    child: Child,
    receiver: Receiver<Result<Packet, String>>,
    reader: Option<JoinHandle<()>>,
    reaped: bool,
    delivered: bool,
}
impl Job {
    pub fn spawn(executable: &Path, root: &Path, id: u64) -> Result<Self, String> {
        Self::spawn_request(executable, root, id, None)
    }
    pub fn spawn_campaign(
        executable: &Path,
        root: &Path,
        id: u64,
        campaign: &onenil_campaign::Campaign,
    ) -> Result<Self, String> {
        crate::campaign_match::validate(campaign)?;
        let data = campaign.encode()?;
        Self::spawn_request(executable, root, id, Some(data))
    }
    fn spawn_request(
        executable: &Path,
        root: &Path,
        id: u64,
        request: Option<Vec<u8>>,
    ) -> Result<Self, String> {
        let mut child = Command::new(executable)
            .arg(if request.is_some() {
                "--campaign-worker"
            } else {
                "--worker"
            })
            .arg(root)
            .arg(id.to_string())
            .stdin(if request.is_some() {
                Stdio::piped()
            } else {
                Stdio::null()
            })
            .stdout(Stdio::piped())
            .stderr(Stdio::null())
            .spawn()
            .map_err(|e| e.to_string())?;
        if let Some(data) = request {
            let sent = child
                .stdin
                .take()
                .ok_or("missing campaign worker input")
                .map_err(str::to_owned)
                .and_then(|mut input| input.write_all(&data).map_err(|error| error.to_string()));
            if let Err(error) = sent {
                let _ = child.kill();
                let _ = child.wait();
                return Err(error);
            }
        }
        let stdout = child.stdout.take().ok_or("missing worker pipe")?;
        let (sender, receiver) = mpsc::channel();
        let reader = thread::spawn(move || {
            let mut bytes = Vec::new();
            // [designed] Bound protocol memory and reject oversized/trailing packets.
            let result = stdout
                .take(32 * 1024 * 1024 + 1)
                .read_to_end(&mut bytes)
                .map_err(|e| e.to_string())
                .and_then(|_| {
                    if bytes.len() > 32 * 1024 * 1024 {
                        Err("worker packet exceeds limit".into())
                    } else {
                        serde_json::from_slice(&bytes).map_err(|e| e.to_string())
                    }
                });
            let _ = sender.send(result);
        });
        Ok(Self {
            id,
            child,
            receiver,
            reader: Some(reader),
            reaped: false,
            delivered: false,
        })
    }
    pub fn pid(&self) -> u32 {
        self.child.id()
    }
    pub fn reaped(&self) -> bool {
        self.reaped
    }
    pub fn poll(&mut self) -> Poll {
        if self.delivered {
            return Poll::Pending;
        }
        match self.child.try_wait() {
            Ok(Some(_)) => self.reaped = true,
            Ok(None) => return Poll::Pending,
            Err(e) => {
                self.delivered = true;
                return Poll::Failed(self.id, e.to_string());
            }
        }
        match self.receiver.try_recv() {
            Err(TryRecvError::Empty) => Poll::Pending,
            Err(TryRecvError::Disconnected) => {
                self.delivered = true;
                Poll::Failed(self.id, "worker reader disconnected".into())
            }
            Ok(result) => {
                self.delivered = true;
                match result {
                    Ok(packet) if packet.id == self.id => match packet.result {
                        Ok(t) => Poll::Ready(self.id, t),
                        Err(e) => Poll::Failed(self.id, e),
                    },
                    Ok(_) => Poll::Failed(self.id, "stale worker identity".into()),
                    Err(e) => Poll::Failed(self.id, e),
                }
            }
        }
    }
    pub fn cancel(&mut self) -> Result<(), String> {
        if !self.reaped {
            match self.child.try_wait().map_err(|e| e.to_string())? {
                Some(_) => self.reaped = true,
                None => {
                    self.child.kill().map_err(|e| e.to_string())?;
                    self.child.wait().map_err(|e| e.to_string())?;
                    self.reaped = true;
                }
            }
        }
        if let Some(reader) = self.reader.take() {
            reader.join().map_err(|_| "worker reader panic")?;
        }
        self.delivered = true;
        Ok(())
    }
}
impl Drop for Job {
    fn drop(&mut self) {
        let _ = self.cancel();
    }
}
pub fn run(root: &Path, id: u64) -> Result<(), String> {
    let packet = Packet {
        id,
        result: timeline::compute(root),
    };
    serde_json::to_writer(std::io::stdout().lock(), &packet).map_err(|e| e.to_string())
}
pub fn run_campaign(id: u64) -> Result<(), String> {
    let result = (|| {
        let mut input = Vec::new();
        std::io::stdin()
            .take(1024 * 1024 + 1)
            .read_to_end(&mut input)
            .map_err(|error| error.to_string())?;
        let campaign = onenil_campaign::Campaign::decode(&input)?;
        timeline::compute_campaign(&campaign)
    })();
    serde_json::to_writer(std::io::stdout().lock(), &Packet { id, result })
        .map_err(|error| error.to_string())
}
