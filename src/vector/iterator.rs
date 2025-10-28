use crate::vector::Vector;
use std::marker::PhantomData;

pub struct Iter<'a, T, const N: usize> {
    inner: &'a Vector<T, N>,
    front_index: usize,
    back_index: usize,
    size: Option<usize>,
}

pub struct IntoIter<T, const N: usize> {
    inner: Vector<T, N>,
    front_index: usize,
    back_index: usize,
    size: Option<usize>,
}

pub struct IterMut<'a, T, const N: usize> {
    inner: *mut Vector<T, N>,
    front_index: usize,
    back_index: usize,
    size: Option<usize>,
    phantom_data: PhantomData<&'a mut Vector<T, N>>,
}


