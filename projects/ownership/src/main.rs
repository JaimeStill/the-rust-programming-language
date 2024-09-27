fn main() {
    println!("string_mutation()");
    string_mutation("Jaime");    
    println!();

    println!("string_clone()");
    string_clone();
    println!();

    println!("fn_ownership()");
    fn_ownership();
    println!();

    println!("return_ownership()");
    return_ownership();
    println!();

    println!("tuple_return()");
    tuple_return();
    println!();
}

fn string_mutation(word: &str) {
    let mut s = String::from("hello");

    s.push_str(format!(", {word}!").as_str());

    println!("{s}");
}

fn string_clone() {
    let s1 = String::from("hello");
    let s2 = s1.clone();

    println!("s1 = {s1}, s2 = {s2}");
}

fn fn_ownership() {
    // s comes into scope
    let s = String::from("hello");

    // s's value moves into the function
    takes_ownership(s);

    // s is no longer valid

    // x comes into scope
    let x = 5;

    /*
        x would move into the function,
        but i32 is Copy, so its okay
        to still use x afterward
    */
    makes_copy(x);
}
/*
    Here, x goes out of scope, then s. But because s's
    value was moved, nothing special happens.
*/

fn takes_ownership(some_string: String) {
    // some_string comes into scope
    println!("{some_string}");
}
/*
    some_string goes out of scope and `drop` is called.
    The backing memory is freed.
*/

fn makes_copy(some_integer: i32) {
    // some_integer comes into scope
    println!("{some_integer}");
}
// some_integer goes out of scope.

fn return_ownership() {
    // moves its return value into s1
    let s1 = gives_ownership();
    println!("s1: {s1}");

    // s2 comes into scope
    let s2 = String::from("hello");
    println!("s2: {s2}");

    let s3 = takes_and_gives_back(s2);
    println!("s3: {s3}");
}
/*
    s3 goes out of scope and is dropped.
    s2 was moved, so nothing happens.
    s1 goes out of scope and is dropped.
*/

/*
    moves its return value into
    the function that calls it
*/
fn gives_ownership() -> String {
    // some_string comes into scope
    let some_string = String::from("yours");

    /*
        some_string is returned and moves
        out to the calling function.
    */
    some_string
}

// takes a String and returns one
fn takes_and_gives_back(a_string: String) -> String {
    // a_string comes into scope.

    /*
        a_string is returned and moves
        out to the calling function
    */
    a_string
}

/*
    this is a lot of work just to avoid
    s1 from going out of scope.
*/
fn tuple_return() {
    let s1 = String::from("hello");

    let (s2, len) = calculate_length(s1);

    println!("The length of '{s2}' is {len}.");
}

fn calculate_length(s: String) -> (String, usize) {
    // len() returns the length of the string
    let length = s.len();

    (s, length)
}