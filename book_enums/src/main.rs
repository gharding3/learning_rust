
// enums are different than java/C#/C++
// although they can work like that.
// each label ("variant" in Rust) can be extended with its own data
// in this case each variant behaves like a struct, and has its own constructor
#[derive(Debug)]
enum IpAddrKind {
    V4(u8,u8,u8,u8),
    V6(String)
}

// just like structs, we can define "associated methods" on enums
impl IpAddrKind {
    fn loopback(v:u8) -> IpAddrKind {
        if v == 4 { 
            return IpAddrKind::V4(127,0,0,1);
        }
        if v == 6 { 
            return IpAddrKind::V6(String::from("::1"));
        }
        panic!("Bad IP address version: {}", v);
    }
    fn print(&self) {
        println!("{:?}", self);
    }
}

fn main() {

    let ipv4 = IpAddrKind::loopback(4);
    let ipv6 = IpAddrKind::loopback(6);

    route(ipv4);
    route(ipv6);

    //let ipv5 = IpAddrKind::loopback(5);
    //route(ipv5);

    {
        println!("---- Option refs ----");
        let x = String::from("trigger");
        let y = String::from("cross");
        let mut opt = Some(&x);
        opt = Some(&y);
        
        match opt {
            Some(s) => println!("s = {}", s),
            None => println!("empty")
        }
    }

    println!("---- Fun with Option ----");
    {
        // the "Option" enum is included in the prelude (of std library),
        // its variants can be used without the "Option::" prefix
        let something:Option<i32> = Some(5);
        let nothing:Option<i32> = None;
        
        assert!(something.is_some());
        assert!(!nothing.is_some());

        assert!(!something.is_none());
        assert!(nothing.is_none());

        let custom_unwrap = match something {
            Some(value) => value, // If the enum variant has value(s), the match expr needs ot include bindings
            None => 0
        };
        println!("something: {}", custom_unwrap);
        println!("nothing: {}", nothing.unwrap_or_default());

        let something = plus_one(something);
        let nothing = plus_one(nothing);
        println!("something: {}", something.unwrap_or_default());
        println!("nothing: {}", nothing.unwrap_or_default());

        // if let syntax sugar
        // useful for when you want to match a single variant
        // equivalent to:
        /*
        match o {
            Some(value) => {} // do something (with 'value')
            _ => ()
        }
        */
        fn print_if_has_value(name:&str, o:&Option<i32>) {
            if let Some(value) = o {
                println!("{} has value: {}" , name, value);
            } else{
                println!("{} has no value" , name);
            }
        }
        print_if_has_value("something", &something);
        print_if_has_value("nothing", &nothing);
    }

    println!("---- Fun with match ----");
    {
        // match is like a java switch with pattern matching
        #[derive(Debug)]
        enum Coin {
            Penny,
            Nickel,
            Dime,
            Quarter,
        }

        let coin = Coin::Quarter;

        let cents = match coin {
            // note: this won't compile unless we have cases for all enum variants
            Coin::Penny => 1,
            Coin::Nickel => {
                 5
            },
            Coin::Dime => {10},
            Coin::Quarter => {100/4},
        };
        println!("coin value in cents = {}", cents);

        let is_lucky = match coin {
            Coin::Penny => true,
            _ => false
        };
        println!("is it lucky? = {}", is_lucky);
        
        fn deposit_for_later(c:&Coin) {
            println!("Saving {:?} for later...", c);
        }
        match coin {
            Coin::Quarter => deposit_for_later(&coin),
            _ => (),
        }
    }
}

// map the Option<T>
fn plus_one(o: Option<i32>) -> Option<i32> {
    match o {
        Some(i) => Some(i + 1),
        None => None
    }
}

fn route(ip_kind: IpAddrKind) {
    ip_kind.print();
}