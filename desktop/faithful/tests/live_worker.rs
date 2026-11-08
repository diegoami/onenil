//! [designed:T76] Real children retain one owned image; no window or campaign posting.
use onenil_desktop::{
    live_worker::{Decision, LiveJob, Poll},
    timeline::{self, Completion, IntervalTimeline},
};
use std::{
    path::Path,
    time::{Duration, Instant},
};
fn exe() -> &'static Path {
    Path::new(env!("CARGO_BIN_EXE_onenil-desktop"))
}
fn interval(job: &mut LiveJob) -> IntervalTimeline {
    let start = Instant::now();
    loop {
        let before = Instant::now();
        let poll = job.poll();
        assert!(before.elapsed() < Duration::from_millis(100));
        match poll {
            Poll::Pending => std::thread::sleep(Duration::from_millis(5)),
            Poll::Interval(_, prefix) => return prefix,
            Poll::Ready(..) => panic!("completed before decision"),
            Poll::Failed(_, error) => panic!("{error}"),
        }
        assert!(start.elapsed() < Duration::from_secs(180));
    }
}
fn complete(job: &mut LiveJob) -> Completion {
    let start = Instant::now();
    loop {
        let before = Instant::now();
        let poll = job.poll();
        assert!(before.elapsed() < Duration::from_millis(100));
        match poll {
            Poll::Pending => std::thread::sleep(Duration::from_millis(5)),
            Poll::Ready(_, result) => return result,
            Poll::Interval(..) => panic!("duplicate interval"),
            Poll::Failed(_, error) => panic!("{error}"),
        }
        assert!(start.elapsed() < Duration::from_secs(180));
    }
}
#[test]
fn original_and_edited_intervals_arrive_alive_then_continue_without_changing_results() {
    for edited in [false, true] {
        let mut campaign = onenil_campaign::Campaign::initial().unwrap();
        if edited {
            campaign
                .exchange(
                    onenil_campaign::Location::First(1),
                    onenil_campaign::Location::First(2),
                )
                .unwrap();
        }
        let before = campaign.encode().unwrap();
        let expected = timeline::compute_campaign(&campaign).unwrap();
        let mut job = LiveJob::spawn_campaign(exe(), 7, &campaign).unwrap();
        assert!(job.decide(Decision::Continue).is_err());
        let prefix = interval(&mut job);
        prefix.validate().unwrap();
        assert!(!job.reaped());
        assert_eq!(prefix.boundary, "0AB9:7E4A");
        let value = serde_json::to_value(&prefix).unwrap();
        assert!(value.get("native_return").is_none());
        let mut false_completion = value;
        false_completion["native_return"] = serde_json::json!("0AB9:EBE2");
        assert!(serde_json::from_value::<IntervalTimeline>(false_completion).is_err());
        assert!(prefix.is_prefix_of(&expected));
        for _ in 0..3 {
            assert!(matches!(job.poll(), Poll::Pending));
            std::thread::sleep(Duration::from_millis(10));
        }
        assert!(campaign.played_fixture.is_none());
        assert_eq!(before, campaign.encode().unwrap());
        if edited {
            assert!(job.decide(Decision::ReplaceVerified).is_err());
            assert!(matches!(job.poll(), Poll::Pending));
        }
        job.decide(Decision::Continue).unwrap();
        assert!(job.decide(Decision::Continue).is_err());
        let result = complete(&mut job);
        assert_eq!(result.timeline, expected);
        assert!(prefix.is_prefix_of(&result.timeline));
        assert!(job.reaped());
        assert!(matches!(job.poll(), Poll::Pending));
        assert!(job.decide(Decision::Continue).is_err());
        assert_eq!(before, campaign.encode().unwrap());
        let mut foreign = result.timeline;
        foreign.initial.clock += 1;
        assert!(!prefix.is_prefix_of(&foreign));
    }
}
#[test]
fn verified_replacement_uses_t75_result_and_retains_the_original_first_half() {
    let campaign = onenil_campaign::Campaign::initial().unwrap();
    let before = campaign.encode().unwrap();
    let mut job = LiveJob::spawn_campaign(exe(), 8, &campaign).unwrap();
    let prefix = interval(&mut job);
    job.decide(Decision::ReplaceVerified).unwrap();
    let result = complete(&mut job);
    let source: serde_json::Value =
        serde_json::from_str(include_str!("../../../records/T75/parity-r0002.json")).unwrap();
    assert_eq!(result.native_sha256, source["results"][0]["match_sha256"]);
    assert_eq!(result.timeline.frames.len(), 3672);
    assert_eq!(result.timeline.draws, 156115);
    assert_eq!(
        result
            .timeline
            .frames
            .iter()
            .map(|f| f.events.len())
            .sum::<usize>(),
        743
    );
    assert!(prefix.is_prefix_of(&result.timeline));
    let first_new = &result.timeline.frames[prefix.frames.len()].view;
    assert!(first_new.players.iter().any(|p| p.player == 2592));
    assert!(!first_new.players.iter().any(|p| p.player == 2582));
    assert_eq!(before, campaign.encode().unwrap());
    assert!(job.reaped());
}
#[test]
fn cancel_and_drop_reap_children_even_while_waiting_at_interval() {
    let campaign = onenil_campaign::Campaign::initial().unwrap();
    let before = campaign.encode().unwrap();
    let mut job = LiveJob::spawn_campaign(exe(), 9, &campaign).unwrap();
    let pid = job.pid();
    interval(&mut job);
    let start = Instant::now();
    job.cancel().unwrap();
    assert!(start.elapsed() < Duration::from_secs(2));
    assert!(job.reaped());
    assert!(matches!(job.poll(), Poll::Pending));
    assert!(job.decide(Decision::Continue).is_err());
    job.cancel().unwrap();
    let mut dropped = LiveJob::spawn_campaign(exe(), 10, &campaign).unwrap();
    let dropped_pid = dropped.pid();
    interval(&mut dropped);
    drop(dropped);
    let early = LiveJob::spawn_campaign(exe(), 11, &campaign).unwrap();
    let early_pid = early.pid();
    drop(early);
    #[cfg(target_os = "linux")]
    for pid in [pid, dropped_pid, early_pid] {
        assert!(!Path::new(&format!("/proc/{pid}")).exists());
    }
    assert_eq!(before, campaign.encode().unwrap());
}
