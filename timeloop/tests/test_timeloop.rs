use std::sync::mpsc;
use std::thread;
use std::time::Duration;

use s2_codegen::job;
use s2_timeloop::TimeLoop;

#[job(5)]
fn foo(rx: mpsc::Receiver<()>) {
    println!("tick 5");
}

#[job(2)]
fn bar(rx: mpsc::Receiver<()>) {
    println!("tick 2");
}

#[test]
pub fn test_job() {
    let mut tl = TimeLoop::new(vec![foo, bar]);
    tl.start();

    loop {
        println!("waiting for 10 secs ...");
        thread::sleep(Duration::from_millis(10000));
        break;
    }

    tl.stop();
}
