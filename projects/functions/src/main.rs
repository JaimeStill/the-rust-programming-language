fn main() {
    print_x(64);
    print_measurement(3, "tbsp.");
    print_expression(6);
    print_x(five());
    print_x(square(32));
}

// function with a parameter
fn print_x(x: i32) {
    println!("The value of x is: {x}");
}

// function with multiple parameters
fn print_measurement(value: i32, unit_label: &str) {
    println!("The measurement is: {value} {unit_label}");
}

// function with expression assignment
fn print_expression(increment: i32) {
    let y = {
        let x = 6;
        x + increment
    };

    println!("The value of the expression is: {y}");
}

// function with return value (expression)
fn five() -> i32 {
    5
}

// more meaningful expression function
fn square(x: i32) -> i32 {
    x * x
}