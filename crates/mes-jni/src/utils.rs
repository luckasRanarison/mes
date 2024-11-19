pub trait RefUnwrap<T> {
    fn unwrap_ref(&self) -> &T;
}

#[allow(clippy::mut_from_ref)]
pub trait MutUnwrap<T> {
    fn unwrap_mut(&self) -> &mut T;
}

impl<T> RefUnwrap<T> for *const T {
    fn unwrap_ref(&self) -> &T {
        unsafe { self.as_ref().expect("null pointer exception") }
    }
}

impl<T> MutUnwrap<T> for *mut T {
    fn unwrap_mut(&self) -> &mut T {
        unsafe { self.as_mut().expect("null pointer exception") }
    }
}
