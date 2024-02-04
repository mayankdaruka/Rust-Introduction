fn main() {
    // println! calls a Rust macro. If it had called a regular function instead, it would
    // be entered as println without the '!'. Basically using a '!' means you're calling a
    // macro instead of a function and macros don't always follow the same rules as functions
    println!("Hello, world!");
}