use std::{thread, time::{self, Duration}};

const TIME: Duration = time::Duration::from_millis(600);

fn main() {
    let number: i32 = 3;

    println!("if_condition:");
    if_condition(number);
    println!();

    println!("not_zero:");
    let result: bool = not_zero(number);

    if result {
        println!("{number} is not zero");
    }
    println!();
    
    let number: i32 = 15;

    println!("is_divisible:");
    is_divisible(number);
    println!();

    println!("is_even:");
    let even: bool = is_even(number);
    println!("{number} is even: {even}");
    println!();

    println!("run_counter:");
    run_counter();
    println!();

    println!("run_labeled_loops:");
    run_labeled_loops();
    println!();

    println!("run_while:");
    run_while();
    println!();

    println!("run_iterative_while:");
    run_iterative_while();
    println!();

    println!("run_for:");
    run_for();
    println!();

    println!("run_countdown:");
    run_countdown();
    println!();

    println!("run_loop:");
    run_loop();
    println!();
}

fn if_condition(value: i32) {
    if value < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }
}

fn not_zero(value: i32) -> bool {
    value != 0
}

fn is_divisible(x: i32) {
    if x % 4 == 0 {
        println!("{x} is divisible by 4");
    } else if x % 3 == 0 {
        println!("{x} is divisible by 3");
    } else if x % 2 == 0 {
        println!("{x} is divisible by 2");
    } else {
        println!("{x} is not divisible by 4, 3, or 2");
    }
}

fn is_even(x: i32) -> bool {
    if x % 2 == 0 { true } else { false }
}

fn run_counter() {
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is {result}");
}

fn run_labeled_loops() {
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
            thread::sleep(TIME);
        }

        count += 1;
        thread::sleep(TIME);
    }

    println!("End count = {count}");
}

fn run_while() {
    let mut number = 3;

    while number != 0 {
        println!("{number}!");

        number -= 1;
        thread::sleep(TIME);
    }

    println!("LIFTOFF!!!");
    thread::sleep(TIME);
}

fn run_iterative_while() {
    let a = [10, 20, 30, 40 ,50];
    let mut index = 0;

    while index < a.len() {
        println!("The value is: {}", a[index]);

        index += 1;
        thread::sleep(TIME);
    }
}

fn run_for() {
    let a = [10, 20, 30, 40, 50];

    for element in a {
        println!("The value is: {element}");
        thread::sleep(TIME);
    }
}

fn run_countdown() {
    for number in (1..4).rev() {
        println!("{number}!");
        thread::sleep(TIME);
    }

    println!("LIFTOFF!!!");
    thread::sleep(TIME);
}

fn run_loop() {
    println!("THIS WILL ONLY STOP WITH CTRL-C");

    let mut it: i32 = 0;

    /*
        Will only stop when you halt the
        program with ctrl + c
    */
    loop {
        it += 1;
        println!("{it}");
        thread::sleep(TIME);
    }
}