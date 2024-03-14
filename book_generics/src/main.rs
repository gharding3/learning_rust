/*
Generics are similar to C++ templates (as opposed to Java generics)
Concrete types are substituted at compile time (as opposed to Java where types are erased),
 this expands the code into as many concrete versions as necessary.
This process is called "Monomorphization" if you want to impress the boss.
There is no runtime cost but there is a code size and compile-time cost (exactly like C++)
*/

mod generics;
mod traits;
mod lifetimes;

// traits have to be brought into scope to use their methods
use generics::geometry;
use traits::NewsArticle;
use lifetimes::Excerpt;

fn main() {

    {
        let txt = String::from("Call be Ishmael. Some years ago...");
        let first_sentence = lifetimes::first_word(&txt);
        // Since Excerpt holds a ref, it cannot outlive the lifetime of first_sentence, which cannot outlive txt
        let excerpt = Excerpt::from(first_sentence); 
        println!("{}", excerpt.text());
    }

    {
        let s1 = String::from("hum and sausage");
        {
            let s2 = "hamu and egg";
            let longest = lifetimes::longest(&s1, &s2); // will have same lifetime as s2 (shorter of s1,s2), even if ref to s1 is returned
            println!("longest = {}", longest);
        }
    }

    {
        let article = NewsArticle {
            headline : String::from("Hamster Shortage Reported!"),
            author : String::from("edgeworth")
        };
        traits::printSummary(&article);

        let article2 = traits::returns_summary();
        traits::printSummary2(&article2);
        //traits::printSummary3(&article);
    }

    {
        let vec_i32 = vec![2, 4, 5, 6, 3, 1];
        let max_i32 = generics::largest(&vec_i32);
        println!("max value is {} from {:?}", max_i32, vec_i32);

        let vec_ch = ['y','y','z','x'];
        let max_ch = generics::largest(&vec_ch);
        println!("max value is {} from {:?}", max_ch, vec_ch);
    }

    {
        let vec_i32 = vec![2, 4, 5, 6, 3, 1];
        let max_i32 = generics::copy_of_largest(&vec_i32);
        println!("max value is {} from {:?}", max_i32, vec_i32);

        let vec_ch = ['y','y','z','x'];
        let max_ch = generics::copy_of_largest(&vec_ch);
        println!("max value is {} from {:?}", max_ch, vec_ch);
    }

    let point_f32 = geometry::Point::of(1.2f32, 0.50f32, -5.2f32);
    println!("point = {} {} {}", point_f32.x(), point_f32.y(), point_f32.z());
    println!("d = {}", point_f32.distance_from_origin());
    println!("largest coord = {}", point_f32.largest_coord());
}
