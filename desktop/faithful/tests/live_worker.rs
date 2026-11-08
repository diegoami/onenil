//! [designed:T76] The interval must arrive before child exit, with no completed receipt.
use onenil_desktop::live_worker::{Decision, LiveJob, Poll};
use std::{path::Path,time::{Duration,Instant}};
fn exe()->&'static Path {Path::new(env!("CARGO_BIN_EXE_onenil-desktop"))}
#[test]
fn interval_is_delivered_while_the_owned_child_waits_for_a_decision(){
    let campaign=onenil_campaign::Campaign::initial().unwrap();
    let mut job=LiveJob::spawn_campaign(exe(),7,&campaign).unwrap();
    assert!(job.decide(Decision::Continue).is_err());
    let start=Instant::now();
    let prefix=loop {
        match job.poll(){
            Poll::Pending=>std::thread::sleep(Duration::from_millis(5)),
            Poll::Interval(id,prefix)=>{assert_eq!(id,7);break prefix;},
            Poll::Ready(..)=>panic!("completed before decision"),
            Poll::Failed(_,error)=>panic!("{error}"),
        }
        assert!(start.elapsed()<Duration::from_secs(180));
    };
    assert!(!job.reaped());
    assert_eq!(prefix.boundary,"0AB9:7E4A");
    assert!(!prefix.frames.is_empty());
    for _ in 0..3 {assert!(matches!(job.poll(),Poll::Pending));std::thread::sleep(Duration::from_millis(10));}
    job.cancel().unwrap();assert!(job.reaped());
}
