use num_traits::Float;

mod iterator;
mod ops;

#[derive(Clone, PartialOrd, PartialEq, Debug)]
pub struct Vector<T, const N: usize> {
    pub components: [T; N],
    dimensions: usize,
}

impl<T: Float, const N: usize> Vector<T, N> {
    pub fn new(slice: [T; N]) -> Self {
        Vector {
            components: slice,
            dimensions: N,
        }
    }

    pub fn zero() -> Vector<T, N> {
        Vector {
            components: [T::from(0.0).expect("REASON"); N],
            dimensions: N,
        }
    }

    pub fn magnitude(&self) -> T {
        let mut result: T = T::from(0.0).expect("REASON");

        for x in self.components {
            let square = x * x;
            result = result + square;
        }

        let result = result.powf(T::from(1.0 / 2.0).expect("REASON"));

        result
    }

    pub fn dot(&self, rhs: &Self) -> T {
        let mut result: T = T::from(0).expect("REASON");

        for (ele1, ele2) in self.components.iter().zip(rhs.components.iter()) {
            let product = *ele1 * *ele2;
            result = result + product;
        }

        result
    }

    pub fn unit_vector(&self) -> Vector<T, N> {
        let mut normalized_components: [T; N] = self.components;
        let magnitude: T = T::from(self.magnitude()).expect("magnitude could not be calculated");

        for x in &mut normalized_components {
            *x = *x / magnitude;
        }

        Vector {
            components: normalized_components,
            dimensions: N,
        }
    }

    pub fn angle_rad(&self, rhs: &Self) -> T {
        T::acos(self.dot(rhs) / self.magnitude() * rhs.magnitude())
    }

    pub fn iter_mut(&mut self) -> std::slice::IterMut<'_, T> {
        self.components.iter_mut()
    }

    pub fn iter(&self) -> std::slice::Iter<'_, T> {
        self.components.iter()
    }
}
