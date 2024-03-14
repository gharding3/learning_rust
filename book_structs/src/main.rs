/*
---- Data types ----
i8  u8
i16	u16
i32	u32
i64	u64
128 u128
isize	usize
char    // 4 bytes and represents a Unicode scalar value (ex. U+10FFFF)
f32
f64
bool
*/

use rand::Rng::{self};

// Struct
#[derive(Debug)]
struct DiceRoll {
    left: u8,
    right: u8,
}

// Impl block : "Associated functions"
impl DiceRoll {
    fn total(self: &DiceRoll) -> u8 {
        self.left + self.right
    }
    fn max(&self) -> u8 {
        if self.left > self.right {self.left} else {self.right}
    }
    // within an impl block, Self is an alias for DiceRoll
    fn min(self: &Self) -> u8 {
        if self.left < self.right {self.left} else {self.right}
    }
    // if self param is missing, the method is static (like String::from)
    /*
    fn bestRoll() -> &[u8] {
        return [6,6]
    }
    fn worstRoll() -> &[u8] {
        return [1,1]
    }
    */
    fn from(l:u8, r:u8) -> DiceRoll {
        DiceRoll { left: l, right: r }
    }
    fn random() -> DiceRoll {
        let l = rand::thread_rng().gen_range(1..7);
        let r = rand::thread_rng().gen_range(1..7);
        DiceRoll { left : l, right :r }
    }
}

// You can extend a struct's "associated methods" with multiple impl blocks
impl DiceRoll {
    fn reroll(&mut self) -> () {
        let roll = DiceRoll::random();
        self.left = roll.left;
        self.right = roll.right;
    }
}

// raw tuple
fn roll2() -> (u8, u8) {
    return (1, 4);
}

// struct
fn roll() -> DiceRoll {
    return DiceRoll {
        left: 1,
        right: 4, 
    }
}

// tuple struct
struct DiceRoll2(u8, u8);

// unit struct (wtf?)
#[derive(Debug)]
struct EmptyRoll;

#[derive(Debug)]
struct VoidRoll;

fn main() {
    println!("-- raw tuple --");
    {
        let roll = roll2();
        let (x, y) = roll;
        println!("You rolled {} {}", x, y);
    }
    println!("-- basic struct --");
    {
        let mut roll = DiceRoll::random();
        let (x, y) = (roll.left, roll.right);
        println!("You rolled {} {}", x, y);
        roll.reroll();
        println!("You re-rolled {:?}", roll);
        println!("Your score: {}", roll.total());
        println!("Your min: {}", roll.min());
        println!("Your max: {}", roll.max());
    }
    println!(" -- mutable struct --");
    {
        let mut roll = roll();
        roll.left = 2;
        roll.right = 6;
        let (x, y) = (roll.left, roll.right);
        println!("You rolled {} {}", x, y);
        println!("Your score: {}", roll.total());
    }
    // struct update syntax
    // note this respects rust move semantics (copy-trait data is deep-copied, other data is "stolen")
    println!("-- struct update syntax --");
    {
        let roll1 = roll();
        let roll2 = DiceRoll {
            left: 6,
            ..roll1
        };
        println!("You rolled {} {}", roll1.left, roll1.right);
        println!("You rolled {} {}", roll2.left, roll2.right);
    }
    println!("-- tuple structs --");
    {
        let roll = DiceRoll2(5,2);
        let roll2 = DiceRoll2 {
            0: 4,
            1: 3
        };
        let roll3 = DiceRoll2 {
            0: 1,
            ..roll
        };
        println!("You rolled {} {}", roll.0, roll.1);
        println!("You rolled {} {}", roll2.0, roll2.1);
        println!("You rolled {} {}", roll3.0, roll3.1);
    }
    println!("-- unit structs --");
    {
        // not sure what the point is yet
        let e = EmptyRoll;
        let v = VoidRoll;
        println!("e = {:?}", e);
        println!("v = {:?}", v);
        // == operator cannot be applied to structs by default
    }
}
