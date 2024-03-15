use std::sync::{Mutex, MutexGuard};

static SINGLETON: Mutex<App> = Mutex::new(App {});

pub struct App {}

impl App {
    pub fn new() -> MutexGuard<'static, App> {
        SINGLETON.lock().unwrap()
    }
}
