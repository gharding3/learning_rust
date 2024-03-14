

struct Counter
{
    count: u32
}

impl Counter
{
    fn new() -> Counter {
        Counter { count: 0 }
    }
}

impl Iterator for Counter 
{
    // Item is the element type
    type Item = u32;

    // Calling next on the iterator will return a new value for the Counter,
    // until it reaches 5, when the iterator will stop returning new values (None)
    fn next(&mut self) -> Option<Self::Item> {
        if self.count < 5 {
            self.count += 1;
            Some(self.count)
        } else {
            None
        }
    }
}

fn main() {
    
    // iterators are lazy (like streams)
    let v1 = vec![1, 2, 3];
    let v1_iter = v1.iter();
    for val in v1_iter {
        println!("Got: {}", val);
    }
}

// all iterators implement a trait named Iterator in the standard library
// this trait has 1 required method:
//      next(&mut self) which returns one time at a time.
//          we can call the next method on iterators directly
//          when the iteration is over, it returns None.  Othewise, it returns a Some(&value)
#[test]
fn iterator_demo() {

    let mut v1 = vec![1, 2, 3];
    {
        let mut v1_iter = v1.iter(); // must be mutable in order to call next

        // next() returns immutable references to values in the vector
        assert_eq!(v1_iter.next(), Some(&v1[0]));
        assert_eq!(v1_iter.next(), Some(&v1[1]));
        assert_eq!(v1_iter.next(), Some(&3)); // ???
        assert_eq!(v1_iter.next(), None);
    }
    {
        // iter_mut() can iterate over mutable references in the vector,
        // if the vector itself is mutable
        let mut v1_iter = v1.iter_mut();
        assert_eq!(v1_iter.next(), Some(&mut 1)); // ???
        assert_eq!(v1_iter.next(), Some(&mut 2)); // we can't do assert_eq(&mut v[1]) here since this would require 2 mutable references to the same element :)
        assert_eq!(v1_iter.next(), Some(&mut 3))
    }
}

// .sum() method 
// consumes the iterator and produces a value
#[test]
fn sum_demo() {

    let v1 = vec![1, 2, 3];
    let v1_iter = v1.iter();

    let total: i32 = v1_iter.sum(); // needs a type here?
    // iterator is now consumed, cannot be used after this point

    assert_eq!(total, 6);
}

// "adapter" methods convert the iterator into a different iterator
// but the iterator is still lazy - it won't be evaluated until consumed
#[test]
fn map_demo() {

    let v1 = vec![4, 5, 6];
    let it = v1.iter().map(|x| { x + 1 });
    let v2:Vec<_> = it.collect();
    assert_eq!(vec![5, 6, 7], v2);
}

// the iter_into method takes ownership of the vector (moves)
// the filter() runs on an immutable reference to each element
// if filter() returns true, the element is included in the modified iterator
#[test]
fn filter_demo() {

    let v1 = vec![4, 5, 6];
    let it = v1.into_iter().filter(|x:&i32| *x >= 5);
    let v2:Vec<_> = it.collect();
    assert_eq!(vec![5, 6], v2);
}

#[test]
fn test_counter() {

    let mut c = Counter::new();

    assert_eq!(Some(1), c.next());
    assert_eq!(Some(2), c.next());
    assert_eq!(Some(3), c.next());
    assert_eq!(Some(4), c.next());
    assert_eq!(Some(5), c.next());
    assert_eq!(None, c.next());
}

#[test]
fn test_zip_counter() {

    let c1 = Counter::new();
    let c2 = Counter::new();
    // zipped: returns a new iter of tuples (x, y) where x is sourced from left, y is sourced from right
    // if iterators are not the same length, the zip operation stops when the shortest iter returns None
    let zipped = c1.zip(c2);
    let mapped = zipped.map(|(x, y)| { x * y });
    let summed: u32 = mapped.sum();
    // 1 + 4 + 9 + 16 + 25
    assert_eq!(55, summed);
}