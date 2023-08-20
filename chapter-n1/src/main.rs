fn main() {
    // Declare (and initialize, maybe) variables.
    let a: i32;
    let b = 3;
    let c: () = ();

    let d;
    a = 42;
    println!("a = {}, b = {}, c = {:?}", a, b, c);
    d = 4;
    println!("d = {}", d);

    // Mutable variables.
    let mut this_is_mutable = 3;
    println!("before mutating: {}", this_is_mutable);
    this_is_mutable = 114514;
    println!("after mutating: {}", this_is_mutable);

    // Tuples
    let test_tuple: (i32, bool, f64) = (1, true, 5.3f64);
    println!("test_tuple = {:?}", test_tuple);

    // Accessing tuples
    println!("the first element of test_tuple: {}", test_tuple.0);
    println!("test_tuple.2 = {}", test_tuple.2);
    let (t0, t1, t2) = test_tuple;
    println!("t0 = {}, t1 = {}, t2 = {}", t0, t1, t2);

    // Arrays
    let arr: [i32; 4] = [1, 14, 5, 14];
    println!("{:?}", arr);
    
    // Strings
    let string_literal = "Hello, World! ";
    println!("{}", string_literal);

    let sref = "I'm ref";
    let sobj = String::from(sref);
    let sref2 = sobj.as_str();
    println!("{} {} {}", sref, sobj, sref2);

    // Ownership
    let a = String::from("Hello, World");
    let b = a;
    println!("b = {}", b);
    // println!("a = {}", a);
    // ^ This code doesn't compile because a is moved. 
    
    let a = String::from("Hello, World");
    {
        let b = a;
        println!("b = {}", b);
    }
    // println!("a = {}", a);
    // ^ This code doesn't compile because a is moved.

    // References! 
    #[allow(unused)]
    {
        let c = &mut String::from("HI");
        let d = &(1+1);
        let e = &mut (1 + 1);
        let f = 6;
        let g = &mut ( f + 1 );
    }

    let a = String::from("Hello, World");
    {
        let b = &a;
        println!("b = {}", b);
    }
    println!("a = {}", a);

    let a;
    {
        let local = String::from("HI");
        a = &local;
        println!("local = {}, a = {}", local, a);
    }
    // println!("a = {}", a);
    // ^ this line doesn't compiles.

}
