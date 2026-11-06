use onenil_desktop::{
    timeline::Phase,
    worker::{Job, Poll},
};
use std::{
    path::Path,
    time::{Duration, Instant},
};
fn exe() -> &'static Path {
    Path::new(env!("CARGO_BIN_EXE_onenil-desktop"))
}
fn root() -> &'static Path {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .parent()
        .unwrap()
}
#[test]
fn owned_worker_completes_cancels_fails_and_reaps() {
    let start = Instant::now();
    let mut job = Job::spawn(exe(), root(), 7).unwrap();
    let pid = job.pid();
    let timeline = loop {
        let poll = Instant::now();
        let state = job.poll();
        assert!(poll.elapsed() < Duration::from_millis(100));
        match state {
            Poll::Pending => {
                assert!(start.elapsed() < Duration::from_secs(180));
                std::thread::sleep(Duration::from_millis(5));
            }
            Poll::Ready(id, t) => {
                assert_eq!(id, 7);
                break t;
            }
            Poll::Failed(_, e) => panic!("{e}"),
        }
    };
    assert!(job.reaped());
    assert_eq!(timeline.seed, 64);
    assert_eq!(timeline.frames.len(), 4199);
    assert_eq!(timeline.draws, 182811);
    assert_eq!(timeline.clock_rows, 4101);
    assert_eq!(timeline.frames.last().unwrap().view.score, [2, 1]);
    assert_eq!(timeline.native_return, "0AB9:EBE2");
    assert_eq!(
        timeline
            .frames
            .iter()
            .map(|f| f.events.len())
            .sum::<usize>(),
        831
    );
    assert_eq!(
        timeline
            .initial
            .players
            .iter()
            .map(|p| p.player)
            .collect::<Vec<_>>(),
        vec![
            2661, 2662, 2675, 2664, 2665, 2666, 2667, 2673, 2669, 2670, 2676, 2581, 2582, 2583,
            2584, 2585, 2586, 2587, 2588, 2589, 2590, 2591
        ]
    );
    let mut playback = onenil_desktop::timeline::Playback::new(timeline);
    playback.start_or_continue().unwrap();
    while playback.phase == Phase::FirstHalf {
        playback.step(8).unwrap();
    }
    assert_eq!(playback.phase, Phase::HalfTime);
    assert!(playback.step(1).is_err());
    playback.start_or_continue().unwrap();
    while playback.phase == Phase::SecondHalf {
        playback.step(8).unwrap();
    }
    assert_eq!(playback.phase, Phase::FullTime);
    let events = playback.events.len();
    assert!(playback.start_or_continue().is_err());
    assert_eq!(playback.events.len(), events);
    playback.restart();
    assert_eq!(playback.phase, Phase::Ready);
    assert!(playback.events.is_empty());
    let mut cancel = Job::spawn(exe(), root(), 8).unwrap();
    let before = Instant::now();
    cancel.cancel().unwrap();
    assert!(before.elapsed() < Duration::from_secs(2));
    assert!(cancel.reaped());
    let mut failure = Job::spawn(exe(), &root().join("missing-fixture-root"), 9).unwrap();
    loop {
        match failure.poll() {
            Poll::Pending => std::thread::sleep(Duration::from_millis(5)),
            Poll::Failed(id, e) => {
                assert_eq!(id, 9);
                assert!(!e.is_empty());
                break;
            }
            Poll::Ready(..) => panic!("invalid input accepted"),
        };
        assert!(start.elapsed() < Duration::from_secs(180));
    }
    assert!(failure.reaped());
    let close = Job::spawn(exe(), root(), 10).unwrap();
    let close_pid = close.pid();
    drop(close);
    #[cfg(target_os = "linux")]
    {
        assert!(!Path::new(&format!("/proc/{pid}")).exists());
        assert!(!Path::new(&format!("/proc/{close_pid}")).exists());
    }
}
