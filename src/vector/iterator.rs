use crate::vector::Vector;
use num_traits::Float;

impl<'a, T: Float, const N: usize> IntoIterator for &'a Vector<T, N> {
    type Item = &'a T;
    type IntoIter = std::slice::Iter<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.components.iter()
    }
}

impl<T: Float, const N:usize> IntoIterator for Vector<T, N> {
    type Item = T;
    type IntoIter = std::array::IntoIter<T, N>;

    fn into_iter(self) -> Self::IntoIter {
        self.components.into_iter()
    }
}

impl<'a, T: Float, const N: usize> IntoIterator for &'a mut Vector<T, N> {
    type Item = &'a mut T;
    type IntoIter = std::slice::IterMut<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.components.iter_mut()
    }
}


