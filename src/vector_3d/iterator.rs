use crate::vector_3d::Vector3d;
use std::iter::FusedIterator;
use std::marker::PhantomData;

pub struct Iter<'a, T> {
    /// reference to original struct
    pub(super) inner: &'a Vector3d<T>,
    /// index for Iterator
    pub(super) front_index: u8,
    /// index for DoubleEndedIterator
    pub(super) end_index: u8,
    /// Keep track of number of items in Iterator so that DoubleEndedIterator don't overlap,
    /// Option so that once front_index > 3 or check_sub() returns None we can return early.
    pub(super) size: Option<usize>,
}

pub struct IntoIter<T> {
    pub(super) inner: Vector3d<T>,
    pub(super) front_index: u8,
    pub(super) end_index: u8,
    pub(super) size: Option<usize>,
}

pub struct IterMut<'a, T> {
    /// raw Pointer to original mut struct
    pub(super) inner: *mut Vector3d<T>,
    /// index for `next()` method
    pub(super) front_index: u8,
    /// index for `next_back()`, in DoubleEndedIterator
    pub(super) back_index: u8,
    /// Tell Compiler we are storing mut reference to Vector<T>
    pub(super) _phantom: PhantomData<&'a mut Vector3d<T>>,
    /// The current size of the Iterator, Option so that we may return early after size < 0.
    pub(super) size: Option<usize>,
}

// <editor-fold desc="Iter Method start, reference to original">
impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        // early return after first underflow occurs
        if self.size.is_none() {
            return None;
        }
        // check for integer underflow for u8
        if let Some(new_size) = self.size?.checked_sub(1) {
            self.size = Some(new_size);
        } else {
            return None;
        }

        // match index to named field
        let to_return = match self.front_index {
            0 => Some(&self.inner.i),
            1 => Some(&self.inner.j),
            2 => Some(&self.inner.k),
            _ => None,
        };

        self.front_index += 1;
        to_return
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        if self.size.is_none() {
            (0, None)
        } else {
            (self.size.unwrap(), self.size)
        }
    }
}

impl<'a, T> IntoIterator for &'a Vector3d<T> {
    type Item = &'a T;
    type IntoIter = Iter<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        Iter {
            inner: self,
            front_index: 0,
            end_index: 2,
            size: Some(3),
        }
    }
}

impl<T> FusedIterator for Iter<'_, T> {}

impl_exact_size_iterator!(Iter<'a, T>);

impl<'a, T> DoubleEndedIterator for Iter<'a, T> {
    fn next_back(&mut self) -> Option<Self::Item> {
        //so we don't have to check for an underflow after first underflow occurs
        if self.size.is_none() {
            return None;
        }

        if let Some(new_size) = self.size?.checked_sub(1) {
            self.size = Some(new_size);
        } else {
            self.size = None;
            return None;
        }

        let dir_to_return = match self.end_index {
            0 => Some(&self.inner.i),
            1 => Some(&self.inner.j),
            2 => Some(&self.inner.k),
            _ => None,
        };

        self.end_index -= 1;
        dir_to_return
    }
}
//</editor-fold>

//<editor-fold desc="IntoIter method start, Takes ownership of original struct">

impl<T: Copy> Iterator for IntoIter<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.size.is_none() {
            return None;
        }

        if let Some(new_value) = self.size?.checked_sub(1) {
            self.size = Some(new_value);
        } else {
            self.size = None;
            return None;
        }

        let to_return = match self.front_index {
            0 => Some(self.inner.i.clone()),
            1 => Some(self.inner.j.clone()),
            2 => Some(self.inner.k.clone()),
            _ => None,
        };

        self.front_index += 1;

        to_return
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        if self.size.is_none() {
            (0, None)
        } else {
            (self.size.unwrap(), self.size)
        }
    }
}

impl<T: Copy> IntoIterator for Vector3d<T> {
    type Item = T;
    type IntoIter = IntoIter<T>;

    fn into_iter(self) -> Self::IntoIter {
        IntoIter {
            inner: self,
            front_index: 0,
            end_index: 2,
            size: Some(3),
        }
    }
}

impl<T: Copy> FusedIterator for IntoIter<T> {}

impl_exact_size_iterator!(IntoIter<T>);

impl<T: Copy> DoubleEndedIterator for IntoIter<T> {
    fn next_back(&mut self) -> Option<Self::Item> {
        //so we don't have to check for an underflow after first underflow occurs
        if self.size.is_none() {
            return None;
        }

        if let Some(new_size) = self.size?.checked_sub(1) {
            self.size = Some(new_size);
        } else {
            self.size = None;
            return None;
        }

        let dir_to_return = match self.end_index {
            0 => Some(self.inner.i),
            1 => Some(self.inner.j),
            2 => Some(self.inner.k),
            _ => None,
        };

        self.end_index -= 1;
        dir_to_return
    }
}
//</editor-fold>

// <editor-fold desc="IterMut method start, mut reference to original">
impl<'a, T> Iterator for IterMut<'a, T> {
    type Item = &'a mut T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.size.is_none() {
            return None;
        }

        //calculate first early return
        if let Some(new_value) = self.size?.checked_sub(1) {
            self.size = Some(new_value);
        } else {
            self.size = None; // early return
        }

        let to_return = match self.front_index {
            0 => Some(unsafe { &mut (*self.inner).i }),
            1 => Some(unsafe { &mut (*self.inner).j }),
            2 => Some(unsafe { &mut (*self.inner).k }),
            _ => None,
        };

        self.front_index += 1;

        to_return
    }
}

impl<'a, T> IntoIterator for &'a mut Vector3d<T> {
    type Item = &'a mut T;
    type IntoIter = IterMut<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        IterMut {
            inner: self,
            front_index: 0,
            back_index: 2,
            _phantom: PhantomData,
            size: Some(3),
        }
    }
}

impl<'a, T> FusedIterator for IterMut<'a, T> {}

impl_exact_size_iterator!(IterMut<'a, T>);

impl<'a, T> DoubleEndedIterator for IterMut<'a, T> {
    fn next_back(&mut self) -> Option<Self::Item> {
        if self.size.is_none() {
            return None;
        }

        if let Some(new_value) = self.size?.checked_sub(1) {
            self.size = Some(new_value);
        } else {
            self.size = None; // early return
        }

        let dir_to_return = match self.back_index {
            0 => Some(unsafe { &mut (*self.inner).i }),
            1 => Some(unsafe { &mut (*self.inner).j }),
            2 => Some(unsafe { &mut (*self.inner).k }),
            _ => None,
        };

        self.back_index -= 1;
        dir_to_return
    }
}
// </editor-fold>
