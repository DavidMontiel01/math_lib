use std::ops::{Add, AddAssign, Mul, MulAssign, Sub, SubAssign};
use num_traits::Float;
use crate::vector::Vector;

impl<T: Float> Add for Vector<T> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Vector {
            i: self.i + rhs.i,
            j: self.j + rhs.j,
            k: self.k + rhs.k,
        }
    }
}

impl<T: Float> AddAssign for Vector<T> {
    fn add_assign(&mut self, rhs: Self) {
        *self = Vector {
            i: self.i + rhs.i,
            j: self.j + rhs.j,
            k: self.k + rhs.k,
        }
    }
}

impl<T: Float> Sub for Vector<T> {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Vector {
            i: self.i - rhs.i,
            j: self.j - rhs.j,
            k: self.k - rhs.k,
        }
    }
}

impl<T: Float> SubAssign for Vector<T> {
    fn sub_assign(&mut self, rhs: Self) {
        *self = Vector {
            i: self.i - rhs.i,
            j: self.j - rhs.j,
            k: self.k - rhs.k,
        }
    }
}

impl<T: Float> Mul<T> for Vector<T> {
    type Output = Self;

    fn mul(self, scalar: T) -> Self::Output {
        Vector {
            i: self.i * scalar,
            j: self.j * scalar,
            k: self.k * scalar,
        }
    }
}

impl<T: Float> MulAssign<T> for Vector<T> {
    fn mul_assign(&mut self, scalar: T) {
        *self = Vector {
            i: self.i * scalar,
            j: self.j * scalar,
            k: self.k * scalar,
        }
    }
}
