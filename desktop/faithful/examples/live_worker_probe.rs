//! [designed:T76] Real-process receipt transport, no expected match state in requests.
use onenil_desktop::{
    live_worker::{Decision, LiveJob, Poll},
    timeline::{self, Completion, IntervalTimeline},
};
use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::{
    path::Path,
    time::{Duration, Instant},
};
fn sha<T: serde::Serialize>(v: &T) -> String {
    format!("{:x}", Sha256::digest(serde_json::to_vec(v).unwrap()))
}
fn interval(job: &mut LiveJob) -> Result<IntervalTimeline, String> {
    let start = Instant::now();
    loop {
        match job.poll() {
            Poll::Pending => std::thread::sleep(Duration::from_millis(5)),
            Poll::Interval(_, p) => return Ok(p),
            Poll::Failed(_, e) => return Err(e),
            Poll::Ready(..) => return Err("premature completion".into()),
        }
        if start.elapsed() > Duration::from_secs(180) {
            return Err("interval timeout".into());
        }
    }
}
fn complete(job: &mut LiveJob) -> Result<Completion, String> {
    let start = Instant::now();
    loop {
        match job.poll() {
            Poll::Pending => std::thread::sleep(Duration::from_millis(5)),
            Poll::Ready(_, p) => return Ok(p),
            Poll::Failed(_, e) => return Err(e),
            Poll::Interval(..) => return Err("duplicate interval".into()),
        }
        if start.elapsed() > Duration::from_secs(180) {
            return Err("completion timeout".into());
        }
    }
}
fn process_gone(pid: u32) -> bool {
    #[cfg(target_os = "linux")]
    {
        !Path::new(&format!("/proc/{pid}")).exists()
    }
    #[cfg(not(target_os = "linux"))]
    {
        let _ = pid;
        true
    }
}
fn run(executable: &Path) -> Result<Value, String> {
    let mut cases = vec![];
    for (number, edited, decision) in [
        (1, false, Decision::Continue),
        (2, true, Decision::Continue),
        (3, false, Decision::ReplaceVerified),
    ] {
        let mut campaign = onenil_campaign::Campaign::initial()?;
        if edited {
            campaign.exchange(
                onenil_campaign::Location::First(1),
                onenil_campaign::Location::First(2),
            )?;
        }
        let campaign_sha256 = sha(&campaign);
        let original = campaign.encode()?;
        let expected = if decision == Decision::Continue {
            Some(timeline::compute_campaign(&campaign)?)
        } else {
            None
        };
        let started = Instant::now();
        let mut job = LiveJob::spawn_campaign(executable, number, &campaign)?;
        if job.decide(decision).is_ok() {
            return Err("early decision accepted".into());
        }
        let prefix = interval(&mut job)?;
        let pid = job.pid();
        if job.reaped() || campaign.played_fixture.is_some() || !matches!(job.poll(), Poll::Pending)
        {
            return Err("interval is not pending/live".into());
        }
        let prefix_sha256 = sha(&prefix);
        let frames = prefix.frames.len();
        if edited && job.decide(Decision::ReplaceVerified).is_ok() {
            return Err("edited replacement admitted".into());
        }
        job.decide(decision)?;
        if job.decide(decision).is_ok() {
            return Err("duplicate decision admitted".into());
        }
        let result = complete(&mut job)?;
        if !prefix.is_prefix_of(&result.timeline)
            || !job.reaped()
            || !process_gone(pid)
            || campaign.encode()? != original
            || !matches!(job.poll(), Poll::Pending)
        {
            return Err("owned result/reaping/campaign invariant differs".into());
        }
        if expected.as_ref().is_some_and(|v| *v != result.timeline) {
            return Err("old complete timeline changed".into());
        }
        if decision == Decision::ReplaceVerified {
            let proof: Value =
                serde_json::from_str(include_str!("../../../records/T75/parity-r0002.json"))
                    .map_err(|e| e.to_string())?;
            if result.native_sha256 != proof["results"][0]["match_sha256"] {
                return Err("T75 replacement result differs".into());
            }
        }
        cases.push(json!({"profile":if edited{"edited"}else{"original"},"decision":decision,"campaign_sha256":campaign_sha256,
            "pid":pid,"interval_delivered_alive":true,"interval_boundary":prefix.boundary,"interval_frames":frames,"interval_draws":prefix.draws,
            "prefix_sha256":prefix_sha256,"complete_frames":result.timeline.frames.len(),"complete_draws":result.timeline.draws,
            "actual_return":result.timeline.native_return,"native_sha256":result.native_sha256,"timeline_sha256":sha(&result.timeline),
            "prefix_preserved":true,"campaign_unchanged":true,"completion_once":true,"child_reaped":true,"elapsed_seconds":started.elapsed().as_secs_f64()}));
    }
    let campaign = onenil_campaign::Campaign::initial()?;
    let mut cancellations = vec![];
    for (id, drop_job) in [(4, false), (5, true)] {
        let mut job = LiveJob::spawn_campaign(executable, id, &campaign)?;
        let prefix = interval(&mut job)?;
        let pid = job.pid();
        if drop_job {
            drop(job);
        } else {
            job.cancel()?;
            if !job.reaped() {
                return Err("cancel failed to reap".into());
            }
            drop(job);
        }
        if !process_gone(pid) {
            return Err("cancel/drop left child alive".into());
        }
        cancellations.push(json!({"drop":drop_job,"pid":pid,"interval_frames":prefix.frames.len(),"child_reaped":true}));
    }
    Ok(
        json!({"cases":cases,"cancellations":cancellations,"platform":std::env::consts::OS,"status":"passed"}),
    )
}
fn main() {
    let result = std::env::args()
        .nth(1)
        .ok_or("worker executable required".into())
        .and_then(|p| run(Path::new(&p)));
    match result {
        Ok(packet) => println!("{}", packet),
        Err(error) => {
            eprintln!("{error}");
            std::process::exit(1);
        }
    }
}
