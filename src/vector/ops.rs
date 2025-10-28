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
        self.components = self.components.map(|&mut x| *x += rhs);
    }
}

impl<T: Float, const N: usize> Mul for Vector<T, N> {
    type Output = Self;

    fn mul(self, rhs: T) -> Self::Output {
        let output = Self;
        for (e1, e2) in &self.components.iter().zip(output.components.iter_mut()) {
            *e2 = e1 * rhs;
        }

        output
    }
}

impl<T: Float, const N: usize> MulAssign for Vector<T, N> {
    fn mul_assign(&mut self, rhs: T) {
        self.components = self.components.map(|&mut x| x = x * rhs);
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

impl<T: Float, const N: usize> Div for Vector<T, N> {
    type Output = Self;

    fn div(self, rhs: T) -> Self::Output {
        let mut output = Self::zero();

        for (qoutient, e1) in output.iter_mut().zip(&self.components.iter()) {
            *qoutient = e1 / rhs;
        }
    }
}

impl<T: Float, const N: usize> DivAssign for Vector<T, N> {
    fn div_assign(&mut self, rhs: T) {
        self.components = self.components.map(|&mut x| *x = x / rhs);
    }
}
