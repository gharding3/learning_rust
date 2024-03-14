
// generic version
// accept a slice of T's, returns a T
pub fn largest<T: PartialOrd>(array: &[T]) -> &T {
    let mut max = &array[0];
    for elem in array {
        if *elem > *max {
            max = elem;
        }
    }
    max
}

// makes a copy of the stack data instead of returning a reference
pub fn copy_of_largest<T: PartialOrd + Copy>(array: &[T]) -> T {
    let mut max = array[0];
    for &elem in array {
        if elem > max {
            max = elem;
        }
    }
    max
}


pub mod geometry {

    pub struct Point<T> {
        x: T,
        y: T,
        z: T
    }
    
    // when declaring a generic impl for a genericized struct,
    // you have to put impl<T> instead of impl
    impl<T> Point<T> {
    
        pub fn of(x:T, y:T, z:T) -> Point<T> {
            Point { x: x, y: y, z: z}
        }
    
        pub fn x(&self) -> &T {
            &self.x
        }
        pub fn y(&self) -> &T {
            &self.y
        }
        pub fn z(&self) -> &T {
            &self.z
        }
    }

    // you can have a type-specific impl of a genericized struct,
    // similar to C++ specialized templates
    impl Point<f32> { 
        pub fn distance_from_origin(&self) -> f32 {
            (self.x.powi(2) + self.y.powi(2) + self.z.powi(2)).sqrt()
        }
    }

    // impls can also be trait-specific with the impl<T: ...> syntax
    impl<T: PartialOrd + Copy> Point<T> {
        pub fn largest_coord(&self) -> T {
            let l1 = super::copy_of_largest(&[self.x, self.y]);
            let l2 = super::copy_of_largest(&[self.z, l1]);
            l2
        }
    }
}
