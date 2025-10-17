pub mod vector{
    use std::ops::{Add, AddAssign, Sub};

    #[derive(Debug, Copy, Clone, PartialEq)]
    pub struct Vector {
        i: isize,
        j: isize
    }

    impl Vector {

    }

    impl Add for Vector {
        type Output = Self;
        fn add(self, rhs: Self) -> Self::Output {
            Vector{ i: self.i + rhs.i, j: self.j + rhs.j }
        }
    }
    impl AddAssign for Vector {
        fn add_assign(&mut self, rhs: Self) {
            *self = Self {i: self.i + rhs.i, j: self.j + rhs.j};
        }
    }
   impl Sub for Vector {
        type  Output = Self;

        fn sub(self, rhs: Self) -> Self::Output {
            Vector {i: self.i - rhs.i, j: self.j - rhs.j}
        }
    }

}