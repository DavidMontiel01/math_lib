#[cfg(test)]
mod tests {
    use math_lib::vector_3d::Vector3d;

    #[test]
    fn test_iter() {
        let vec = Vector3d::new(1.0, 2.0, 3.0);
        let iterator = vec.iter();
        for x in iterator {
            println!("{}", x);
        }

        println!("{}", vec.i);
        println!("{}", vec.j);
    }
    #[test]
    fn test_into_iter() {
        let vec = Vector3d::new(1.0, 2.0, 3f32);
        let into = vec.into_iter();

        for x in into {
            println!("{}", x);
        }
    }

    #[test]
    fn test_mut_iter() {
        let mut vec = Vector3d::new(1f32, 2f32, 3f32);

        for x in &mut vec {
            *x = *x + 1.0;
        }
    }
}
