use core::cell::RefCell;
use core::ops::Deref;

/// A tiny wrapper around [`RefCell`] that can be stored in static immutable variables.
/// # Safety
/// [`LocalCell`] is supposed to be used within single-threaded context. Using it in multi-threaded applications
/// may cause undefined behavior. For such environments consider using [`std::sync::RwLock`] instead.
pub struct LocalCell<T> {
    inner: RefCell<T>,
}
/// SAFETY: The code is run within single-threaded context.
unsafe impl<T> Sync for LocalCell<T> {}

impl<T> LocalCell<T> {
    #[inline(always)]
    pub const fn new(value: T) -> Self {
        LocalCell {
            inner: RefCell::new(value),
        }
    }
}

impl<T> From<RefCell<T>> for LocalCell<T> {
    #[inline(always)]
    fn from(value: RefCell<T>) -> Self {
        LocalCell { inner: value }
    }
}

impl<T> Into<RefCell<T>> for LocalCell<T> {
    #[inline(always)]
    fn into(self) -> RefCell<T> {
        self.inner
    }
}

impl<T> Deref for LocalCell<T> {
    type Target = RefCell<T>;

    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
