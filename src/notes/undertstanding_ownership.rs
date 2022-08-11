// Ownership is a set of rules that governs how a Rust program manages memory
// string vs heap
// there are three ownership rules in rust:
//  (1) Each value in Rust has an owner.
//  (2) There can only be one owner at a time.
//  (3) When the owner goes out of scope, the value will be dropped.

// scope is the range within a program for which an item is valid
// a variable is valid from the point at which it is declared until the end of the current scope
fn main() {
    {                      // s is not valid here, it’s not yet declared
        let s = "hello";   // s is valid from this point forward

        // do stuff with s
    }                      // this scope is now over, and s is no longer valid
}

// String is a datatype that allows a variable length string to be stored (unlike string literals)
let mut s = String::from("hello");
s.push_str(", world!"); // push_str() appends a literal to a String
println!("{}", s); // This will print `hello, world!`

// rust handles memory allocation differently on the heap than other languages:
// - Older C style languages without garbage collectors (GC) require manual allocate and free to the
// heap (annoying AF imo)
// - languages with GC automatically allocate and free memory when it is used/no longer needed
// - in rust on the other hand, memory is automatically freed when the variable that owns it goes
// out of scope
{
    let s = String::from("hello"); // s is valid from this point forward
    // do stuff with s
}                                  // this scope is now over, and s is no longer valid

// MOVE
// double referencing is valid in rust as long as the value is wholly contained in the stack
// if the value is not wholly contained in the stack, like a String type who only contains the
// pointer and metadata in the stack and the actual values in the heap, an error will be thrown
// since there is the possibility of double de-allocation of the memory in the heap.
// instead, what happens is that rust invalidates the original memory reference in the stack
// and creates a new stack item with those values, i.e. it moves the original stack reference item
// into a second stack reference item without affecting the actual data in the heap
let s1 = String::from("hello");
let s2 = s1;

println!("{}, world!", s1); // will throw a compile error since the original s1 stack pointer has
// been invalidated when it was moved into s2

// CLONE
// if you do choose to instead fully copy the heap memory in addition to the stack reference, use clone
// this is resource intensive
let s1 = String::from("hello");
let s2 = s1.clone();
println!("s1 = {}, s2 = {}", s1, s2);

// COPY
// when data with all its info in the stack is copied
// the data is not moved, only certain datatypes can be copied, and you cant copy dropped variables
// - All the integer types, such as u32.
// - The Boolean type, bool, with values true and false.
// - All the floating point types, such as f64.
// - The character type, char.
// - Tuples, if they only contain types that also implement Copy. For example, (i32, i32) implements
// Copy, but (i32, String) does not.
let x = 5;
let y = x;

println!("x = {}, y = {}", x, y);

// passing a value to a function has a similar effect as when assigning a value to a variable
// i.e. passing a variable to a function will move or copy, just as assignment does
fn main() {
    let s = String::from("hello");  // s comes into scope

    takes_ownership(s);             // s's value moves into the function...
    // ... and so is no longer valid here

    let x = 5;                      // x comes into scope

    makes_copy(x);                  // x would move into the function,
    // but i32 is Copy, so it's okay to still
    // use x afterward

} // Here, x goes out of scope, then s. But because s's value was moved, nothing
// special happens.

fn takes_ownership(some_string: String) { // some_string comes into scope
    println!("{}", some_string);
} // Here, some_string goes out of scope and `drop` is called. The backing
// memory is freed.

fn makes_copy(some_integer: i32) { // some_integer comes into scope
    println!("{}", some_integer);
} // Here, some_integer goes out of scope. Nothing special happens.

// returning values from functions also transfers ownership
fn main() {
    let s1 = gives_ownership();         // gives_ownership moves its return
    // value into s1

    let s2 = String::from("hello");     // s2 comes into scope

    let s3 = takes_and_gives_back(s2);  // s2 is moved into
    // takes_and_gives_back, which also
    // moves its return value into s3
} // Here, s3 goes out of scope and is dropped. s2 was moved, so nothing
// happens. s1 goes out of scope and is dropped.

fn gives_ownership() -> String {             // gives_ownership will move its
    // return value into the function
    // that calls it

    let some_string = String::from("yours"); // some_string comes into scope

    some_string                              // some_string is returned and
    // moves out to the calling
    // function
}

// This function takes a String and returns one
fn takes_and_gives_back(a_string: String) -> String { // a_string comes into
    // scope

    a_string  // a_string is returned and moves out to the calling function
}

// rust also allows the passing of data using tuples
fn main() {
    let s1 = String::from("hello");

    let (s2, len) = calculate_length(s1);

    println!("The length of '{}' is {}.", s2, len);
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len(); // len() returns the length of a String

    (s, length)
}
// constantly transferring ownership back and forth, especially to/from functions can be tedious,
// which is why rust has implemented references
//A reference is like a pointer in that it’s an address we can follow to access the data stored at
// that address; that data is owned by some other variable. Unlike a pointer, a reference is
// guaranteed to point to a valid value of a particular type for the life of that reference.
// reference is denoted by &
// dereferencing is denoted by *
fn main() {
    let s1 = String::from("hello");

    let len = calculate_length(&s1);

    println!("The length of '{}' is {}.", s1, len);
}

fn calculate_length(s: &String) -> usize {
    s.len()
}

// references are immutable by default
// references can be mutable using &mut
// the only drawback to mutable references is that you can only have one mutable referencer per
// value at a time, which prevents data races at compile time
// if you have a mutable reference, you also cannot have mutable references, however you can have
// multiple immutable references at the same time
fn main() {
    let mut s = String::from("hello");
    change(&mut s);
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}

// you can always use curly brackets to create an artificial scope
let mut s = String::from("hello");
{
let r1 = &mut s;
} // r1 goes out of scope here, so we can make a new reference with no problems.
let r2 = &mut s;

// Dangling references are when a reference is made to a location in memory that has already been
// freed
// Dangling references are impossibly in rust and are caught by the compiler
fn dangle() -> &String { // dangle returns a reference to a String

    let s = String::from("hello"); // s is a new String

    &s // we return a reference to the String, s
} // Here, s goes out of scope, and is dropped. Its memory goes away.
// Danger!

// Slices let you reference a contiguous sequence of elements in a collection rather than the whole
// collection. A slice is a kind of reference, so it does not have ownership.
// a string slice looks like this
let s = String::from("hello world");
let hello = &s[0..5];
let world = &s[6..11];
let slice = &s[..2]; // from he beginning
let slice = &s[3..]; // to the end
let slice = &s[..];  // the whole thing

// &String is a string reference and is mutable
// &str is a string literal reference and is immutable

// slices can be used in other collections as well, such as in arrays shown below:
let a = [1, 2, 3, 4, 5];
let slice = &a[1..3];

assert_eq!(slice, &[2, 3]);
