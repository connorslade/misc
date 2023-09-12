//! On the off chance that somebody is actually looking at this.
//! DO NOT USE THIS. THERE IS NO GOOD REASON TO DO SOMETHING LIKE THIS.
//! i only wrote this because my reasons are bad.
//! also its been too long since ive used unsafe for no reason.
//! This is originally from my [radio-data project](https://github.com/Basicprogrammer10/radio-data/blob/master/src/misc/soon.rs).

use std::{cell::UnsafeCell, mem::MaybeUninit, ops::Deref};

#[cfg(debug_assertions)]
mod safety;
#[cfg(test)]
mod test;

/// A *VERY UNSAFE* way to set values after creating a struct.
/// Like a RefCell without the borrow checking.
/// You are expected to use it properly.
pub struct Soon<T> {
    inner: MaybeUninit<UnsafeCell<T>>,
    #[cfg(debug_assertions)]
    safety: safety::SoonSafety,
}

impl<T> Soon<T> {
    /// Create a new `Soon` with out its value.
    /// If it is dereferenced at this point, in debug mode it will panic
    /// and in release mode you will get some sorta segfault.
    /// **(very unsafe)**
    pub fn empty() -> Self {
        Self {
            inner: MaybeUninit::zeroed(),
            #[cfg(debug_assertions)]
            safety: safety::SoonSafety::default(),
        }
    }

    /// Replace whatever is in the `Soon` with a specified value.
    /// Only call this once per soon object.
    pub fn replace(&self, val: T) {
        #[cfg(debug_assertions)]
        self.safety.on_replace();

        // SAFETY: nobody cares >:)
        let cell = self.inner.as_ptr() as *mut T;
        unsafe {
            cell.write(val);
        }
    }
}

impl<T> Deref for Soon<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        #[cfg(debug_assertions)]
        self.safety.on_deref();

        let cell = UnsafeCell::raw_get(self.inner.as_ptr());
        unsafe { &*cell }
    }
}

impl<T> Drop for Soon<T> {
    fn drop(&mut self) {
        let cell = UnsafeCell::raw_get(self.inner.as_ptr());
        unsafe { cell.drop_in_place() }
    }
}

// shhhhh. its not really thread safe.
unsafe impl<T: Send> Send for Soon<T> {}
unsafe impl<T: Sync> Sync for Soon<T> {}
