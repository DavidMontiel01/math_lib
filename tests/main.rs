#[cfg(test)]
mod tests {
    use math_lib::vector::Vector;

    #[test]
    fn test_iter() {
        let vec = Vector::new(1.0, 2.0, 3.0);
        let iterator = vec.iter();
        for x in  iterator {
            println!("{}",x);
        }

        println!("{}",vec.i);
        println!("{}", vec.j);
    }
    #[test]
    fn test_into_iter () {
        let vec = Vector::new(1.0, 2.0, 3f32);
        let into = vec.into_iter();

        for x in into {
             println!("{}", x);
        }

        println!("{}", vec);
    }

}