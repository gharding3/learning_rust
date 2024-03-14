use rand::Rng;
use std::io;
use std::thread;
use std::process;
use std::time::Duration;

fn roll(n : u32) -> u32 {
    rand::thread_rng().gen_range(1..(n+1))
}

struct LazyU32<T,V:Copy>
where T: Fn() -> V
{
    init: T,
    result: Option<V>
}

impl<T,V:Copy> LazyU32<T,V>
where T: Fn() -> V
{
    fn new(init: T) -> LazyU32<T,V> {
        LazyU32 {
            init: init,
            result: None
        }
    }

    fn get(&mut self) -> V {
        match self.result {
            Some(value) => value,
            None => {
                let value = (self.init)();
                self.result = Some(value);
                value
            }
        }
    }
}

fn generate_workout(intensity: u32, random_number: u32) -> () {

    // unlike functions, closures do not have an explicit return type
    // the compiler will infer the argument and return types
    // however you can add explicit types if you wish
    
    // Rust infers which variables are captured (unlike C++ where you must specify explicitly)
    // they can be captured by reference, or by "move" (stealing)

    // Closures can be stored in variables with traits: Fn, FnMut, or FnOnce
    //  FnOnce : takes ownership of everything it captures
    //  Fn : borrows captured variables immutably
    //  FnMut : borrows captured varialbes mutably
    // most of the time, start with Fn. The compiler will tell you if you need to use FnMut or FnOnce instead.
    let mut lazy = LazyU32::new(|| {
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        random_number
    });

    if intensity < 25 {
        println!(
            "Today, do {} pushups!",
            lazy.get()
        );
        println!(
            "Next, do {} situps!",
            lazy.get()
        );
    } else {
        if random_number == 3 {
            println!("Take a break today! Remember to stay hydrated!");
        } else {
            println!(
                "Today, run for {} minutes!",
                lazy.get()
            );
        }
    }
}

fn main() {

    // example of closure which "moves" captured variables
    {
        let x = vec![1, 2, 3];
        let equal_to_x = move |z| z == x; // x is now "stolen"
        let y = vec![1, 2, 3];
        println!("y equals x : {}", equal_to_x(y));
    }

    println!("Please enter the value");
    let mut intensity = String::new();

    io::stdin()
        .read_line(&mut intensity)
        .expect("Failed to read line");

    let intensity: u32 = intensity.trim().parse().unwrap_or_else(|_err| {
        println!("Value must be a number.");
        process::exit(1);
    });

    let random_number = roll(7);

    // just for fun :)
    let identity_closure = |x| x;
    let random_number = identity_closure(random_number);

    generate_workout(intensity, random_number);
}

#[test]
fn call_with_different_values() {

    let x = 1;
    let mut c = LazyU32::new(|| x + 1 );

    let v1 = c.get();
    let v2 = c.get();

    assert_eq!(v1, 2);
    assert_eq!(v2, 2);
}
