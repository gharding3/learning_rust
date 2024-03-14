
//! # Restaurant Crate
//! Use this style of comment to document the item that contains the comment.
//! 
//! In this case, the top-level crate.

// "forward declaring" the front_of_house module,
// whose body will be defined in another file (src/front_of_house.rs)
// semi-colon tells Rust to load the module contents from this file
mod front_of_house;

mod back_of_house {

    // Declaring a struct public does NOT make all its fields public
    // need to add "pub" in front of each field we want public
    #[derive(Debug)]
    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String
    }

    // In contrast, declaring an enum public makes all its fields public
    pub enum Appetizer {
        Soup,
        Salad
    }

    impl Breakfast {
        pub fn summer() -> Breakfast {
            Breakfast {
                toast: String::from("rye"),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }

    fn fix_incorrect_order() { 
        cook_order();
        //super::front_of_house::serving::serve_order();
    }

    fn cook_order() {

    }
}

// we can "re-export" modules, types, and functions wih different names
// use reexports to design a public interface for your crate
// this allows you to move stuff around internally without breaking your interface
pub use self::front_of_house::hosting as reservations;
pub use self::front_of_house::hosting::Table;

/// Call this function to get some delicious food
///
/// # Examples
/// examples will run as test cases if you run cargo test
/// ```
/// // need to prefix example code with crate name
/// restaurant::eat_at_restaurant();
/// let x = 5;
/// assert_eq!(x as f64, x as f64)
/// ```
/// # Panics
/// panics if you order something off the menu
/// # Errors
/// Might return an error if you order too many fries
/// # Safety
/// This function is safe to call for most people
pub fn eat_at_restaurant() {

    // absolute path to module in the same crate
    crate::front_of_house::hosting::add_to_waitlist();

    // relative path to module at the same level
    front_of_house::hosting::add_to_waitlist();

    reservations::add_to_waitlist();
    
    // bring the back_of_house structs "into scope"
    // use =~ using =~ import
    // when bringing in structs, it's idiomatic to use the full path
    // when bringing in functions, it's idiomatic to use the path of the parent module
    use crate::back_of_house::Breakfast;
    use back_of_house::Appetizer as Appy;

    // Breakfast cannot be constructed from here because it has private fields
    /*
    let mut meal = Breakfast {
        toast("rye"),
        seasonal_fruit("peaches")
    };
    */
    let mut meal = Breakfast::summer();

    // because toast field is public, and meal variable is mutable,
    // we can modify the field
    meal.toast = String::from("white");

    let appetizer = Appy::Soup;

    println!("---- Meal ----");
    println!("{:?}", meal);
}
