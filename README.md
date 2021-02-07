# s2
Named after the star [s2](https://en.wikipedia.org/wiki/S2_(star)) at the center of our galaxy, it losely mimics the [Python version](https://github.com/sankalpjonn/timeloop).

# Example
```Rust
use std::sync::mpsc;
use std::thread;
use std::time::Duration;

use s2_codegen::job;
use s2_timeloop::TimeLoop;

#[job(5)]
fn foo(rx: mpsc::Receiver<()>) {
    println!("tick after 5 secs");
}

#[job(2)]
fn bar(rx: mpsc::Receiver<()>) {
    println!("tick after 2 secs");
}

...
let mut tl = TimeLoop::new(vec![foo, bar]);
tl.start();

// Do something in between on this thread

tl.stop();
...
```
