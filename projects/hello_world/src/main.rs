/*
    run in command line
    mkdir: mkdir target
    compile: rustc -o target/main main.rs
    execute: ./target/main
    output: Hello, world!
*/

/*
    main function is always the first code
    that runs in every executable Rust program.
*/
fn main() {
    /*
        Prints text to the screen.

        println! calls a rust macro rather than a
        function, indicated by the ending !
    */
    println!("Hello, world!");
}