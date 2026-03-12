fn main() {
    // muitable variable:
    let mut z = 5; // x is mutable and can be changed
                   // immutable variable:
    let w = 5; // w is immutable and cannot be changed

    //integer types;
    let q: i32 = -5; // q is an signed integer 32-bit meaning it can hold both positive and negative values
    let r: u32 = 5; // r is an unsigned integer 32-bit meaning it can only hold positive values
    let s = -100_000; // s is an i32 because it is negative and does not have a suffix

    let index: usize = 0; // usize is an unsigned integer type that is used for indexing and can hold the maximum size of a value in memory
    let length: usize = 10; // length is also a usize because it is used for indexing and does not have a suffix

    // float, boolean, char types:
    let f: f64 = 3.14; // f is a 64-bit floating point number
    let ff = 3.14; // ff is also a f64 because it has a decimal point and does not have a suffix

    let tt: bool = true; // one bit boolean type that can be either true or false
    let ttt = false; // ttt is also a bool because it is a boolean literal

    let char = 'a'; // char is a 4-byte Unicode scalar value that represents a single character
    let crab = '🦀'; // crab is also a char because it is a single Unicode character

    // {:?} for tup, is used for debug formatting, which is useful for printing complex data structures like tuples and arrays
    // {X:?} is used for debug formatting with a specific width, which can be useful for aligning output in a more readable way
    // {} is place holder for variables in the string, and the variables are passed as arguments to println! macro
    println!(
        "z: {}, w: {}, q: {}, r: {}, s: {}, f: {}, ff: {}, tt: {}, ttt: {}, char: {}, crab: {}",
        z, w, q, r, s, f, ff, tt, ttt, char, crab
    );

    // compound types:
    let tup: (i32, f64, u8, char) = (-500, 6.4, 1, 'a'); // tup is a tuple that can hold multiple values of different types
    let (xx, yy, zz, charr) = tup; // destructuring the tuple into individual variables
    println!("tup: {}, {}, {}, {}", xx, yy, zz, charr);
    let one = tup.2; // accessing the third element of the tuple using dot notation
    println!("one: {}", one);

    let arr: [i32; 5] = [1, 2, 3, 4, 5]; // arr can only be same type and has a fixed length
    let first = arr[0]; // accessing the first element of the array using index
    println!("first elem of arr:{}", first);
    let arr2 = [xx; 5]; // arr2 is an array of 5 elements all initialized to 0
    println!("arr2: {}, {}, {}", arr2[0], arr2[1], arr2[2]);

    // functions:
    // always require type annotation
    // implicit return type of () if no return value is specified
    // return specified for explicit return type

    fn add(x: i32, y: i32) -> i32 {
        x + y // expression returns  the value
    }
    fn devide(x: f64, y: f64) -> f64 {
        return x / y; // statement does not return a value, but the return keyword is used to specify the return value of the function
    }
    let dev = devide(10.0, 2.0);
    let sum = add(5, 10);

    println!("dev: {}, sum: {}", dev, sum);

    // experesions:
    // adding a ; turns an expression into a statement, which does not return a value
    let x = {
        let a = 3;
        a + 1 // x  = 4 is the value of the expression, and it is assigned to x
    };
    let y = {
        let b = 3;
        b + 1; // y = () is a statement that does not return a value, and it is assigned to y
    };
    println!("x = {}, y = {:?}", x, y);

    // retu values and unit type
    fn five() -> i32 {
        5 // implicit return of 5, implicitly returns means that the last expression in the function is returned without using the return keyword
    }

    fn print_value(x: i32) {
        println!("the value is: {}", x); // returns () implicitly because it does not have a return statement, and the last expression is a statement that does not return a value
    }

    fn explicit_unit() -> () {
        println!("Helllo"); // returns () explicitly because it has a return type of (), and the last expression is a statement that does not return a value
    }
    let five = five();
    println!("Five: {}", five);
    print_value(10);
    explicit_unit();

    // if else:
    // must have a condition that evaluates to a boolean value
    //(in the condition of an if statement, the expression must evaluate to a boolean value, which is either true or false)
    // must return same type in all else branches and main if branch

    let state = if five > 3 { "Positive" } else { "negative" };
    println!("state: {}", state);

    if five > 3 {
        println!("five is greater than 3");
    } else if five < 3 {
        println!("five is less than 3");
    } else {
        println!("five is equal to 3");
    }

    // loops:
    loop {
        println!("This is an infinite loop");
        break; // break is used to exit the loop, otherwise it will run indefinitely
    }

    let mut n = 3;
    while n > 0 {
        println!("n: {}", n);
        n -= 1; // decrementing n by 1 in each iteration of the loop
    }

    for i in 0..=5 {
        println!("i: {}", i);
    }
    for i in 0..5 {
        println!("i: {}", i);
    }

    // loop cotrols with break labels and continue:
    'outer: for i in 0..5 {
        for j in 0..5 {
            if i == j {
                continue; // skip the rest of the inner loop and move to the next iteration of the inner loop
            }
            if i + j == 5 {
                println!("i+j: {}", i + j);
                break 'outer; // break out of the outer loop when the condition is met
            }
            println!("i: {}, j: {}", i, j);
        }
    }

    let mut counter = 0;
    let resulte = loop{
        counter += 1;
        if counter == 10{
            break counter * 2; // break can also return a value, and it is assigned to resulte
        }
    };
    println!("resulte: {}", resulte);
}
