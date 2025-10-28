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
            components: [0; N],
            dimensions: N,
        }
    }

    pub fn magnitude(&self) -> impl Float {
        let mut result: T = Float::from(0);

        for x in self.components {
            let square = x * x;
            result = result + square;
        }

        let result = result.powf(1 / 2);

        result
    }

    pub fn dot(&self, rhs: &Self) -> impl Float {
        let mut result: T = Float::from(0);

        for (ele1, ele2) in self.components.iter().zip(rhs.components.iter()) {
            let product = *ele1 * *ele2;
            result = result + product;
        }

        result
    }

    pub fn unit_vector(&self) -> Vector<T, N> {
        let mut normalized_components: [T; N] = self.components;
        let magnitude = self.magnitude();

        for x in &mut normalized_components {
            *x = x * (1 / magnitude);
        }

        Vector {
            components: normalized_components,
            dimensions: N,
        }
    }

    pub fn angle_rad(&self, rhs: &Self) -> impl Float {
        Float::acos(self.dot(rhs) / self.magnitude() * rhs.magnitude())
    }

    pub fn iter_mut(&mut self) -> std::slice::IterMut<T> {
       self.components.iter_mut()
    }

    pub fn iter(&self) -> std::slice::Iter<T> {
        self.components.iter()
    }

}
