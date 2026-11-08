//! Independent raw framing client: actual worker refuses foreign or extra commands.
use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::{
    io::{Read, Write},
    path::Path,
    process::{Child, Command, Stdio},
    sync::mpsc::{self, Receiver},
    thread::{self, JoinHandle},
    time::Duration,
};
fn frame(writer: &mut impl Write, value: &Value) {
    let bytes = serde_json::to_vec(value).unwrap();
    writer
        .write_all(&(bytes.len() as u32).to_le_bytes())
        .unwrap();
    writer.write_all(&bytes).unwrap();
    writer.flush().unwrap();
}
struct Raw {
    child: Child,
    packets: Receiver<Option<Value>>,
    reader: Option<JoinHandle<()>>,
}
impl Raw {
    fn new() -> Self {
        let mut child = Command::new(Path::new(env!("CARGO_BIN_EXE_onenil-desktop")))
            .args(["--live-campaign-worker", "21"])
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::null())
            .spawn()
            .unwrap();
        let mut stdout = child.stdout.take().unwrap();
        let (tx, packets) = mpsc::channel();
        let reader = thread::spawn(move || {
            for _ in 0..3 {
                let mut header = [0; 4];
                if stdout.read_exact(&mut header).is_err() {
                    let _ = tx.send(None);
                    return;
                }
                let length = u32::from_le_bytes(header) as usize;
                assert!(length <= 32 * 1024 * 1024);
                let mut bytes = vec![0; length];
                stdout.read_exact(&mut bytes).unwrap();
                if tx
                    .send(Some(serde_json::from_slice(&bytes).unwrap()))
                    .is_err()
                {
                    return;
                }
            }
        });
        Self {
            child,
            packets,
            reader: Some(reader),
        }
    }
    fn packet(&self) -> Value {
        self.packets
            .recv_timeout(Duration::from_secs(180))
            .expect("worker response timeout")
            .expect("missing response")
    }
    fn finished(&mut self) {
        assert!(self
            .packets
            .recv_timeout(Duration::from_secs(10))
            .unwrap()
            .is_none());
        assert!(self.child.wait().unwrap().success());
    }
}
impl Drop for Raw {
    fn drop(&mut self) {
        let _ = self.child.kill();
        let _ = self.child.wait();
        if let Some(reader) = self.reader.take() {
            let _ = reader.join();
        }
    }
}
#[test]
fn actual_worker_refuses_bad_initial_frames() {
    for oversized in [false, true] {
        let mut raw = Raw::new();
        let mut input = raw.child.stdin.take().unwrap();
        if oversized {
            input.write_all(&u32::MAX.to_le_bytes()).unwrap();
        } else {
            frame(&mut input, &json!({"version":1,"id":22,"campaign":{}}));
        }
        drop(input);
        assert_eq!(raw.packet()["body"]["kind"], "failed");
        raw.finished();
    }
}
#[test]
fn actual_worker_refuses_foreign_identity_phase_and_duplicate_choices() {
    let campaign = onenil_campaign::Campaign::initial().unwrap();
    let bytes = campaign.encode().unwrap();
    let identity = format!("{:x}", Sha256::digest(&bytes));
    for damage in 0..4 {
        let mut raw = Raw::new();
        let mut input = raw.child.stdin.take().unwrap();
        frame(
            &mut input,
            &json!({"version":1,"id":21,"campaign":serde_json::from_slice::<Value>(&bytes).unwrap()}),
        );
        assert_eq!(raw.packet()["body"]["kind"], "interval");
        assert!(raw.child.try_wait().unwrap().is_none());
        let mut choice =
            json!({"version":1,"id":21,"identity":identity,"sequence":1,"decision":"continue"});
        if damage == 0 {
            choice["id"] = json!(22);
        }
        if damage == 1 {
            choice["identity"] = json!("foreign");
        }
        if damage == 2 {
            choice["sequence"] = json!(2);
        }
        frame(&mut input, &choice);
        if damage == 3 {
            frame(&mut input, &choice);
        }
        drop(input);
        assert_eq!(raw.packet()["body"]["kind"], "failed");
        raw.finished();
    }
}
