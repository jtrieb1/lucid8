use std::{thread::{self, JoinHandle}, sync::{Arc, Mutex}};
use std::time::Duration;

#[derive(Debug)]
pub struct TimedRegister {
    value: Arc<Mutex<u8>>,
    timer: Arc<Mutex<Option<JoinHandle<()>>>>
}

impl Default for TimedRegister {
    fn default() -> Self {
        Self {
            value: Arc::new(Mutex::new(0)),
            timer: Arc::new(Mutex::new(None))
        }
    }
}

impl TimedRegister {

    pub fn get(&self) -> u8 {
        *self.value.lock().unwrap()
    }

    pub fn set(&mut self, value: u8) {
        *self.value.lock().unwrap() = value;
        let dur = Duration::from_secs(1) * (1 / 60);
        let v = self.value.clone();
        let t = self.timer.clone();
        let handle = thread::spawn(move || {
            loop {
                thread::sleep(dur);
                if *v.lock().unwrap() <= 0 { // Setting this from <= to == leads to odd behavior sometimes.
                    if let Some(_) = &*t.lock().unwrap() {
                        *t.lock().unwrap() = None;
                    }
                }
                *v.lock().unwrap() -= 1;
            }
        });
        *self.timer.lock().unwrap() = Some(handle);
    }
}