use std::{
    mem,
    ops::{Deref, DerefMut},
};

pub trait Owned {
    fn delete(&mut self);
}

#[derive(Debug)]
#[repr(transparent)]
pub struct Own<T>
where
    T: Owned,
{
    inner: Box<T>,
}

impl<T> Own<T>
where
    T: Owned,
{
    pub fn new(value: T) -> Self {
        Self {
            inner: Box::new(value),
        }
    }

    pub fn drop_value(mut own: Own<T>) {
        own.inner.as_mut().delete();
        mem::forget(own);
    }
}

impl<T> Drop for Own<T>
where
    T: Owned,
{
    fn drop(&mut self) {
        self.inner.as_mut().delete();
    }
}

impl<T> AsRef<T> for Own<T>
where
    T: Owned,
{
    fn as_ref(&self) -> &T {
        self.inner.as_ref()
    }
}

impl<T> AsMut<T> for Own<T>
where
    T: Owned,
{
    fn as_mut(&mut self) -> &mut T {
        self.inner.as_mut()
    }
}

impl<T> Deref for Own<T>
where
    T: Owned,
{
    type Target = T;

    fn deref(&self) -> &T {
        self.inner.deref()
    }
}

impl<T> DerefMut for Own<T>
where
    T: Owned,
{
    fn deref_mut(&mut self) -> &mut T {
        self.inner.deref_mut()
    }
}
