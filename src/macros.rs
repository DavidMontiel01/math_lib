#![allow(unused)]

macro_rules! vec {
    () => {
        $crate::vector_3d::zero()
    };
    ($i:expr, $j:expr, $k: expr $(,)?) => {
        $crate::vector_3d::Vector3d::new($i, $j, $k)
    }
}
macro_rules! impl_exact_size_iterator {
    ($ty:ident < $ty:lifetime, $T: ident >) => {
        impl<$lt, $T> ExactSizeIterator for $ty<$lt, $T> {
            fn len($self) -> usize {
                self.size.unwrap_or(0) as usize
            }
        }
    };
}