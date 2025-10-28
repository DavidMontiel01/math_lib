use crate::vector::Vector;
use itertools::izip;
use num_traits::Float;
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign};

impl<T: Float, const N: usize> Add for Vector<T, N> {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        let mut sum: Vector<T, N> = Vector::zero();
        for (e1, e2, e3) in izip!(&mut sum, &self, &rhs) {
            *e1 = e2.clone() + e3.clone();
        }

        sum
    }
}

impl<T: Float, const N: usize> AddAssign for Vector<T, N> {
    fn add_assign(&mut self, rhs: Self) {
        for (self_c, rhs_c) in self.iter_mut().zip(rhs.components.iter()) {
            *self_c = *self_c + *rhs_c;
        }
    }
}

impl<T: Float, const N: usize> Mul<T> for Vector<T, N> {
    type Output = Self;

    fn mul(self, rhs: T) -> Self::Output {
        Self {
            components: self.components.map(|x| x * rhs),
            dimensions: N,
        }
    }
}


impl<T: Float, const N: usize> MulAssign<T> for Vector<T, N> {
    fn mul_assign(&mut self, rhs: T) {
        for x in self.components.iter_mut() {
            *x = *x * rhs;
        }
    }
}

impl<T: Float, const N: usize> Sub for Vector<T, N> {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        let mut output = Vector::zero();

        for (diff, e1, e2) in izip!(&mut output, &self, &rhs) {
            *diff = e1.clone() - e2.clone();
        }

        output
    }
}

impl<T: Float, const N: usize> SubAssign for Vector<T, N> {
    fn sub_assign(&mut self, rhs: Self) {
        for (e1, e2) in self.components.iter_mut().zip(rhs.components.iter()) {
            *e1 = *e1 - *e2;
        }
    }
}

impl<T: Float, const N: usize> Div<T> for Vector<T, N> {
    type Output = Self;

    fn div(self, rhs: T) -> Self::Output {
        Self {
            components: self.components.map(|x| x / rhs),
            dimensions: N,
        }
    }
}

impl<T: Float, const N: usize> DivAssign<T> for Vector<T, N> {
    fn div_assign(&mut self, rhs: T) {
        for x in self.components.iter_mut() {
            *x = *x * rhs;
        }
    }
}
