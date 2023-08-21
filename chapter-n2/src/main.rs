fn main() {
    let something = {
        let mut a = 3;
        println!("Before mutating a: {}", a);
        a += 1;
        println!("After mutating a: {}", a);
        a
    };
    println!("something = {}", something);

    // If
    let a = 3;
    if a < 2 {
        println!("{}", a);
    } else {
        println!("TOOOO BIG! ");
    }

    let res0 = if a == 3 {
        "a is 3"
    } else {
        "a is not 3"
    };
    println!("{}", res0);

    // While
    let mut counter = 0;
    let expr_result = while counter < 3 {
        counter += 1;
    };
    println!("expr_result = {:?}", expr_result);

    // Loop
    let mut counter = 0;
    let arr = [10, 2048, 194, 20, 401];
    let result = loop {
        if counter >= arr.len() {
            break -1;
        }
        if arr[counter] == 194 {
            break counter as isize;
        }
        counter += 1;
    };
    println!("result = {}", result);

    // For
    let arr = [11, 45, 1, 4, 1919, 810];
    for i in arr {
        println!("i = {i}");
    }

    let arr = [(14, "Hello"), (25, "World"), (19, "Yuhang"), (84, "Hu"), (10, "is my"), (114, "bestie! ")];
    for (i, j) in arr {
        println!("(i, j) = ({}, {})", i, j);
    }

    // Match
    let a = 3;
    match a {
        0 => {
            println!("a is zero");
        }
        1 => {
            println!("a is one");
        }
        2 => {
            println!("a is two");
        }
        other => {
            println!("a is another value: {}", other);
        }
    }

    let num = 3;
    match num {
        value if value < 0 => {
            println!("Negative number");
        }
        value if value % 2 == 0 => {
            println!("Even number");
        }
        _ => {
            println!("Odd number");
        }
    }
}

fn homework(a: i32, b: f32, c: &str) {
    let tuple = (a, b, c);
    match tuple {
        (3, _, "Yuhang Hu") => {
            println!("A");
        }
        (42, _, "27Onion Nebell") => {
            println!("B");
        }
        (a, b, "ab") | (a, b, "cd") if a % 2 == 0 && b < 0.35f32 => {
            println!("C");
        }
        (53, _, s) => {
            println!("D: {}", s);
        }
        _ => {
            println!("E");
        }
    }
}

