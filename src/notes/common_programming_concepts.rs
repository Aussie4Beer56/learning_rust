// variables are immutable by default
let x = 5;

// to let variables be mutable, use mut
let mut x = 6

// constants are never mutable and must use an all uppercase snake case naming convention
const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;

// shadowing is when you create a second variable with the same name but give it a different value
// in this case the second variable will "overshadow" the original until it itself is shadowed
// or ultimately when it's scope ends
//
// shadowing is different than mut because instead of reassigning the value of a variable,
// it is instead creating a second variable with the same name of a different value using the "let"
// keyword
//
// with shadowing, we can change the datatype of the variable which can't be done with mut
let x = 5;
let x = x + 1;
{
    let x = x * 2;
}

// rust is statically typed but with built in inference capabilities
// where inference can not be used, you must annotate the datatype
let guess: u32 = "42".parse().expect("Not a number!");

// scalar types represent a single value and have four primary types:
// integers, floating point numbers, booleans, characters

// integers are a number without a fractional component and can be signed (include negatives: i)
// or unsigned (only positive numbers: u)
//
// the data length of the integer has 6 options (default is i32):
// 8bit, 16bit, 32bit, 64bit, 128bit, and arch (dependent on pc architecture i.e. 32bit/64bit)
//
// integer overflow will occur if value is outside datatype bounds, causing wrapping to the value

let num: i8 = 42; // signed 8 bit
let num: u128 = 42; // unsigned 128 bit

// integer literals can be written in 5 ways:
// decimal, hex, octal, binary, byte(u8 only)
let dec: i32 = 98_222;
let hex: i32 = 0xff;
let oct: i32 = 0o77;
let bin: i32 = 0b1111_0000;
let byt: u8 = b'A';

// floating points are numbers with decimal points, and all fp numbers are signed in rust
// there are two types in rust, f32 and f64 (f64 is the default type)
let x = 2.0; // f64
let y: f32 = 3.0; // f32

// rust has all the basic mathematical operators built in
// addition
let sum = 5 + 10;
// subtraction
let difference = 95.5 - 4.3;
// multiplication
let product = 4 * 30;
// division -- integer division rounds down to the nearest integer
let quotient = 56.7 / 32.2;
let floored = 2 / 3; // Results in 0
// remainder
let remainder = 43 % 5;

// rust's boolean values can be implemented as seen below:
let t = true;
let f: bool = false; // with explicit type annotation

// char values are the most primitive alphabetic type and use single quotes (whereas string literals
// use double quotes)
// chars are represented by Unicode Scalar Value and can therefore use more than just ASCII
// i.e. accented letters, asian language characters, emojis, and zero width characters
let c = 'z';
let z: char = 'ℤ'; // with explicit type annotation
let rustaceans = '🦀';

// tuples group together a number of values with a variety of types into one compound type
// to define:
let tup: (i32, f64, u8) = (500, 6.4, 1);
// to easily destructure:
let (x, y, z) = tup;
// to directly access an element:
let five_hundred = tup.0;
let six_point_four = tup.1;
let one = tup.2;

// in rust, arrays are like tuples, however all elements must have the same type and a fixed length
// arrays have their data allocated on the stack instead of the heap
// rust will panic at runtime if you try to access an out of bounds element of an array
// to define:
let a: [i32; 5] = [1, 2, 3, 4, 5];
let b = [3; 5]; //creates an array where all elements have the value 3
// accessing elements
let first = a[0];
let second = a[1];

// rust starts and runs in the main() function
// function names should be lower snake case and can be defined like the following:
fn main() {
    println!("Hello, world!");
    another_function(); // calling a second function
}
fn another_function() {
    println!("Another function.");
}

// parameters can be passed to functions, however all parameters must have a defined type and be
// defined in the function declaration
// multiple parameters can be defined using commas
fn main() {
    print_labeled_measurement(5, 'h');
}
fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {value}{unit_label}");
}

// statements are instructions that perform some action and do not return a value
let y = 6;

// Expressions evaluate to a resulting value
// expressions do not have a semicolon at the end of the line, if it does, it is actually a statement
let y = {
    let x = 3;
    x + 1
};

// return values allow function to send data back to the code that calls them
// return values are not named, but must be declared with ->
// statements cannot be returned, but expressions can
fn five() -> i32 {
    5
}

// if expressions allow you to run snippets of code based on certain conditions
// the condition logic must always result in a boolean value
// only the first block with logic that evaluates to true will be run
// elif can be used for multiple conditions
// else can be used as an endcap for coverage if none of the other conditions evaluate to true
let number = 6;

if number % 4 == 0 {
    println!("number is divisible by 4");
} else if number % 3 == 0 {
    println!("number is divisible by 3");
} else if number % 2 == 0 {
    println!("number is divisible by 2");
} else {
    println!("number is not divisible by 4, 3, or 2");
}

// since if is an expressions, it can be used to assign values in a let statement
let condition = true;
let number = if condition { 5 } else { 6 };

println!("The value of number is: {number}");

// rust has 3 types of loops: loop, while, for

// loop will iterate over a section of code until you tell it to stop with break
// continue will skip over the rest of the code for that loop and restart at the top of the loop
// to return a value from a loop, just add the value you want returned after the break expression
let mut counter = 0;
let result = loop {
    counter += 1;

    if counter == 10 {
        break counter * 2;
    }
};

// if you have multiple loops together, you can add a label to a loop as well as its control code
let mut count = 0;
'counting_up: loop {
    println!("count = {count}");
    let mut remaining = 10;
        loop {
        println!("remaining = {remaining}");
        if remaining == 9 {
            break;
        }
        if count == 2 {
            break 'counting_up;
        }
        remaining -= 1;
    }

    count += 1;
}
println!("End count = {count}");

// a while loop will continue to loop as long as a condition is met
let mut number = 3;

while number != 0 {
    println!("{number}!");

    number -= 1;
}

println!("LIFTOFF!!!");

// for iterative loops where conditional logic isn't needed, for loops should be used
let a = [10, 20, 30, 40, 50];

for element in a {
    println!("the value is: {element}");
}