use std::sync::mpsc;
use std::thread::JoinHandle;
pub struct TimeLoop {
    jobs: Vec<fn(rx: mpsc::Receiver<()>)>,
    handles: Vec<JoinHandle<()>>,
    senders: Vec<mpsc::Sender<()>>,
}

impl TimeLoop {
    pub fn new(jobs: Vec<fn(rx: mpsc::Receiver<()>)>) -> Self {
        TimeLoop {
            jobs,
            handles: Vec::new(),
            senders: Vec::new(),
        }
    }

    pub fn start(&mut self) {
        for j in &self.jobs {
            let (tx, rx) = mpsc::channel();
            self.senders.push(tx);
            let j = j.clone();
            let handle = std::thread::spawn(move || j(rx));
            self.handles.push(handle);
        }
    }

    pub fn stop(self) {
        for s in self.senders.iter() {
            let _ = s.send(());
        }
        for handle in self.handles {
            let _ = handle.join();
        }
    }
}
