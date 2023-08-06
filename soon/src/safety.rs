use std::{
    mem,
    sync::atomic::{AtomicBool, AtomicUsize, Ordering},
    thread,
};

pub struct SoonSafety {
    init_thread: AtomicUsize,
    has_value: AtomicBool,
}

impl SoonSafety {
    pub fn on_replace(&self) {
        if self.init_thread.load(Ordering::Relaxed) != current_thread() {
            panic!("Tried to replace a `Soon` on different thread than it was created.")
        }

        if self.has_value.load(Ordering::Relaxed) {
            panic!("Tried to replace a `Soon` that already had a value.");
        }

        self.has_value.store(true, Ordering::Relaxed);
    }

    pub fn on_deref(&self) {
        if !self.has_value.load(Ordering::Relaxed) {
            panic!("A `Soon` was dereferenced before being givin a value.");
        }
    }
}

impl Default for SoonSafety {
    fn default() -> Self {
        Self {
            init_thread: AtomicUsize::new(current_thread()),
            has_value: Default::default(),
        }
    }
}

fn current_thread() -> usize {
    unsafe { mem::transmute(thread::current().id()) }
}
