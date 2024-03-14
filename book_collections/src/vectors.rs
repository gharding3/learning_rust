
pub fn demo() {

    {
        let mut v: Vec<i32> = Vec::new();
        v.push(5);
        v.push(6);
        v.push(7);
        let last = v.pop(); // Option<T>
        if let Some(last_elem) = last {
            println!("popped = {}", last_elem);
        }
        println!("v = {:?}", v);
    }

    {
        let mut v = vec![1, 2, 3];
        v.push(4);
        let first = &v[0];
        println!("first = {:?}", first);
    }

    {
        let v = vec![1,2,3,4,5];
        // borrow ref to 3rd element
        // [] operator will panic if out-of-range
        let third: &i32 = &v[2];
        println!("The 3rd element is {}", third);
        // .get() returns an Option<T>
        match v.get(5) {
            Some(elem) => println!("The 6th element is {}", elem),
            None => println!("There is no 6th element.")
        }
    }

    {
        let mut v = vec![0, 32, 57];
        for elem in &mut v {
            *elem += 100;
        }
        for elem in &v {
            println!("{}", elem);
        }
    }

    {
        // enum is a lot like a union
        // each enum object has a fixed size (max size of all variants?)
        // since vector of enums knows how much memory is needed for each element
        #[derive(Debug)]
        enum SpreadSheetCell {
            Int(i32),
            Float(f32),
            Text(String)
        }
        let row1 = vec![
            SpreadSheetCell::Int(901),
            SpreadSheetCell::Text(String::from("blue")),
            SpreadSheetCell::Float(8.12)
        ];
        println!("v = {:?}", row1);
    }

}