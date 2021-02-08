use std::sync::mpsc;
use std::sync::Arc;
use std::thread::JoinHandle;

pub struct TimeLoop {
    jobs: Vec<Arc<fn(rx: mpsc::Receiver<()>)>>,
    handles: Vec<JoinHandle<()>>,
    senders: Vec<mpsc::Sender<()>>,
}

impl TimeLoop {
    pub fn new(mut jobs: Vec<fn(rx: mpsc::Receiver<()>)>) -> Self {
        let jobs = jobs.drain(..).map(|j| Arc::new(j)).collect();

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
