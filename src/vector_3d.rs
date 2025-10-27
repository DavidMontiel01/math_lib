use std::fmt::{Display, Formatter};
use crate::vector_3d::iterator::{Iter, IterMut};
use num_traits::Float;
use std::marker::PhantomData;
use std::ops::{Index, IndexMut};

pub mod iterator;
pub mod ops;

/// A generic 3-dimensional vector_3d struct with components specified in the i, j, and k directions.
/// This struct represents a mathematical vector_3d in a 3D space, where each component's type must
/// implement the `Float` trait.
///
/// # Type Parameters
/// - `T`: A generic type for the vector_3d's components that must implement the `Float` trait,
///   allowing operations typically associated with floating-point numbers (e.g., addition,
///   subtraction, square roots).
///
/// # Fields
/// - `i` (`T`): The magnitude of the vector_3d in the i-hat (x-axis) direction.
/// - `j` (`T`): The magnitude of the vector_3d in the j-hat (y-axis) direction.
/// - `k` (`T`): The magnitude of the vector_3d in the k-hat (z-axis) direction.
///
/// # Example
/// ```
/// use my_crate::Vector; // Replace `my_crate` with the actual module path.
///
/// let v = Vector { i: 1.0, j: 2.0, k: 3.0 };
/// println!("Vector components: i={}, j={}, k={}", v.i, v.j, v.k);
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Vector3d<T> {
    pub i: T, // magnitude in the i-hat direction
    pub j: T, // magnitude in the j-hat direction
    pub k: T, // magnitude in teh j-hat direction
}

/// A function representing a zero vector_3d of type `Vector<f32>`.
///
/// The `ZERO` vector_3d is a pre-defined, immutable instance of a `Vector`
/// where all components (i, j, k) are initialized to zero (0.0).
///
/// # Type
/// `Vector<f32>`
///
/// # Usage
/// This constant can be used whenever a default or neutral `Vector<f32>`
/// with zero magnitude is required in calculations or as a placeholder.
///
/// # Example
/// ```rust
/// let zero_vector = Vector::ZERO;
/// assert_eq!(zero_vector.i, 0.0);
/// assert_eq!(zero_vector.j, 0.0);
/// assert_eq!(zero_vector.k, 0.0);
/// ```
///
/// # Fields
/// - `i`: f32, initialized to `0.0`
/// - `j`: f32, initialized to `0.0`
/// - `k`: f32, initialized to `0.0`
pub fn zero<T: Float>() -> Vector3d<T> {
    let zero = T::zero();
    Vector3d {
        i: zero,
        j: zero,
        k: zero,
    }
}

pub fn i_hat<T: Float>() -> Vector3d<T> {
    Vector3d {
        i: T::from(1.0).unwrap(),
        j: T::from(0.0).unwrap(),
        k: T::from(0.0).unwrap(),
    }
}

pub fn j_hat<T: Float>() -> Vector3d<T> {
    Vector3d {
        i: T::from(0.0).unwrap(),
        j: T::from(1.0).unwrap(),
        k: T::from(0.0).unwrap(),
    }
}

pub fn k_hat<T: Float>() -> Vector3d<T> {
    Vector3d {
        i: T::from(0.0).unwrap(),
        j: T::from(0.0).unwrap(),
        k: T::from(1.0).unwrap(),
    }
}

impl<T: Float + Copy + Clone> Vector3d<T> {
    pub fn new(i: T, j: T, k: T) -> Self {
        Vector3d { i, j, k }
    }

    /// Returns the magnitude (length) of the vector_3d calculated as sqrt(i² + j² + k²)
    pub fn magnitude(&self) -> f32 {
        (self.i * self.i + self.j * self.j + self.k * self.k)
            .to_f32()
            .unwrap()
            .sqrt()
    }

    /// Computes the dot product of two 3D vectors.
    ///
    /// The dot product is calculated using the formula:
    /// `dot_product = self.i * other.i + self.j * other.j + self.k * other.k`
    ///
    /// # Parameters
    /// - `&self`: The first vector_3d in the operation.
    /// - `other: &Self`: The second vector_3d in the operation.
    ///
    /// # Returns
    /// A `f32` value representing the dot product of the two vectors.
    ///
    /// # Panics
    /// This function will panic if the resulting value cannot be converted to `f32`
    /// using `.to_f32().unwrap()`.
    ///
    /// # Example
    /// ```
    /// let vector1 = Vector { i: 2.0, j: 3.0, k: 4.0 };
    /// let vector2 = Vector { i: 1.0, j: 0.0, k: 5.0 };
    /// let result = vector1.dot(&vector2);
    /// assert_eq!(result, 22.0);
    /// ```
    pub fn dot(&self, other: &Self) -> f32 {
        (self.i * other.i + self.j * other.j + self.k * other.k)
            .to_f32()
            .unwrap()
    }

    /// Computes the cross-product of two 3D vectors and returns the resulting vector_3d.
    ///
    /// The cross-product of two vectors in three-dimensional space results in a vector_3d
    /// that is perpendicular to both of the original vectors and follows the right-hand rule.
    ///
    /// # Parameters
    /// - `self`: The first vector_3d (of type `Vector`) involved in the cross product computation.
    /// - `other`: A reference to the second vector_3d (of type `Vector`) to compute the cross product with.
    ///
    /// # Returns
    /// A new instance of `Vector` representing the resulting vector_3d from the cross product computation.
    ///
    /// # Formula
    /// For vectors `self = (i₁, j₁, k₁)` and `other = (i₂, j₂, k₂)`, the resulting vector_3d
    /// `(i, j, k)` is calculated as:
    /// - `i = j₁ * k₂ - j₂ * k₁`
    /// - `j = -(i₁ * k₂ - i₂ * k₁)`
    /// - `k = i₁ * j₂ - i₂ * j₁`
    ///
    /// # Example
    /// ```rust
    /// let v1 = Vector { i: 1.0, j: 2.0, k: 3.0 };
    /// let v2 = Vector { i: 4.0, j: 5.0, k: 6.0 };
    /// let result = v1.cross(&v2);
    ///
    /// assert_eq!(result.i, -3.0);
    /// assert_eq!(result.j, 6.0);
    /// assert_eq!(result.k, -3.0);
    /// ```
    ///
    /// # Notes
    /// - The current implementation assumes that `Vector` is a struct with `i`, `j`, and `k` components.
    /// - The returned vector_3d follows the right-hand rule for 3D cross products.
    /// - This operation is not commutative: `v1.cross(v2)` is not equal to `v2.cross(v1)`.
    pub fn cross(&self, other: &Self) -> Self {
        let i_cross = self.j * other.k - other.j * self.k;
        let j_cross = self.i * other.k - other.i * self.k;
        let k_cross = self.i * other.j - other.i * self.j;
        Vector3d {
            i: i_cross,
            j: -j_cross,
            k: k_cross,
        }
    }

    /// Calculates the angle in radians between two vectors.
    ///
    /// # Parameters
    /// - `&self`: A reference to the current vector_3d.
    /// - `other`: A reference to another vector_3d to compute the angle with.
    ///
    /// # Returns
    /// - `f32`: The angle between the two vectors in radians.
    ///
    /// # Panics
    /// This function may panic if the magnitudes of either vector_3d are zero,
    /// as it would result in a division by zero when normalizing the dot product.
    ///
    /// # Example
    /// ```
    /// let vector1 = Vector { x: 1.0, y: 0.0 };
    /// let vector2 = Vector { x: 0.0, y: 1.0 };
    ///
    /// let angle = vector1.angle_rad(&vector2);
    /// assert_eq!(angle, std::f32::consts::FRAC_PI_2); // π/2 radians
    /// ```
    pub fn angle_rad(&self, other: &Self) -> f32 {
        f32::acos(self.dot(other) / (self.magnitude() * other.magnitude()))
    }

    /// Calculates and returns the unit vector_3d (a vector_3d with a magnitude of 1) in the same direction
    /// as the current vector_3d.
    ///
    /// The method first computes the magnitude of the vector_3d, then normalizes the vector_3d by dividing
    /// each of its components (`i`, `j`, `k`) by the magnitude. This ensures the resulting vector_3d
    /// maintains the same direction as the original but has a magnitude of 1.
    ///
    /// # Returns
    ///
    /// A new vector_3d instance representing the unit vector_3d of the current vector_3d.
    ///
    /// # Panics
    ///
    /// This method may panic if:
    /// - The vector_3d's magnitude is `0`, as dividing by zero is undefined.
    /// - The conversion using `T::from(mag).unwrap()` fails, depending on the implementation of
    ///   the type `T`.
    ///
    /// # Examples
    ///
    /// ```
    /// let vector_3d = Vector { i: 3.0, j: 4.0, k: 0.0 };
    /// let unit = vector_3d.unit_vector();
    /// assert_eq!(unit.i, 0.6);
    /// assert_eq!(unit.j, 0.8);
    /// assert_eq!(unit.k, 0.0);
    /// ```
    pub fn unit_vector(&self) -> Self {
        let mag = self.magnitude();
        Vector3d {
            i: self.i / T::from(mag).unwrap(),
            j: self.j / T::from(mag).unwrap(),
            k: self.k / T::from(mag).unwrap(),
        }
    }

    /// Calculates and returns a vector_3d which is the projection of u onto v.
    /// # PARAMETERS
    /// - `u`: A reference to the vector_3d being projected (the vector_3d to project)
    /// - `v`: A reference to the vector_3d onto which `u` is projected (the target direction)
    /// # RETURNS
    /// A new vector_3d instance which is the projection of `u` onto `v`. The returned vector_3d
    /// will be parallel to v.
    /// # PANICS
    /// This function will panic if:
    /// - The magnitude of `v` is 0 (zero vector_3d).
    /// - Type conversion fails during the computation.
    /// # EXAMPLE
    /// ```
    ///
    /// ```
    pub fn project(u: &Self, v: &Self) -> Self {
        let u_dot_v = u.dot(v);
        let v_magnitude = v.dot(v);
        let scalar = T::from(u_dot_v / v_magnitude).unwrap();
        Vector3d {
            i: v.i * scalar,
            j: v.j * scalar,
            k: v.k * scalar,
        }
    }

    pub fn iter(&self) -> Iter<'_, T> {
        Iter {
            inner: self,
            front_index: 0,
            end_index: 2,
            size: Some(3),
        }
    }

    pub fn iter_mut(&mut self) -> IterMut<'_, T> {
        IterMut {
            inner: self,
            front_index: 0,
            back_index: 2,
            _phantom: PhantomData,
            size: Some(3),
        }
    }
}

impl<T> Index<u8> for Vector3d<T> {
    type Output = T;

    fn index(&self, index: u8) -> &Self::Output {
        match index {
            0 => &self.i,
            1 => &self.j,
            2 => &self.k,
            _ => panic!("Index out of bounds for Vector: {}", index),
        }
    }
}

impl<T> IndexMut<u8> for Vector3d<T> {
    fn index_mut(&mut self, index: u8) -> &mut Self::Output {
        match index {
            0 => &mut self.i,
            1 => &mut self.j,
            2 => &mut self.k,
            _ => panic!("Index out of bounds for Vector: {}", index),
        }
    }
}

impl<T: Display> Display for Vector3d<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:}i\u{0302}", self.i)?;

        write!(f, " {:+}j\u{0302}", self.j)?;

        write!(f, " {:+}k\u{0302}", self.k)
    }
}