const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;

fn main() {
    // invalid_assignment();
    mutable_assignment();
    print_constant();
    shadow_variables();
    shadowed_type();
}

/*
    error: cannot assign twice
    to immutable variable
*/
// fn invalid_assignment() {
//     let x = 5;
//     print_x(x);
//     x = 6;
//     print_x(x);
// }

fn mutable_assignment() {
    let mut x = 5;
    print_x(x);
    x = 6;
    print_x(x);
}


fn print_constant() {
    println!("The amount of seconds in 3 hours is: {THREE_HOURS_IN_SECONDS}");
}

fn shadow_variables() {
    let x = 5;
    let x = x + 1;

    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");
    }

    print_x(x);
}

fn shadowed_type() {
    let spaces = "    ";

    // shadow spaces to the length of spaces
    let spaces = spaces.len();

    println!("The amount of spaces is: {spaces}");
}

fn print_x(x: i32) {
    println!("The value of x is: {x}");
}