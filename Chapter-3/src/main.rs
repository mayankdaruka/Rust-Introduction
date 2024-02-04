
fn main() {
    // In addition to immutable variables, there are also constants. Differences between them:
    // - Not allowed to use "mut" with constants. Constants are not just immutable by default, they're ALWAYS immutable
    // - You declare constants using the "const" keyword instead of the "let" keyword
    // - The type of the value MUST be annotated for constants
    // - Constants can be declared in any scope including the global scope
    // - Constants can only be set to a constant expression, not the result of a value that can only be computed during runtime
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;

    let x = 5;
    
    // Second variable overshadows the first
    // It takes any uses of the variable to itself until its scope ends or it iself is shadowed
    // Shadowing is different from "mut" because you still have to use "let" when you reassign to the variable
    // Another difference: when we shadow, we are effectively creating a new variable again and can change the type of the value. This cannot be done when reassigning with "mut"
    let x = x + 1;

    {
        // Also shadows x and creates a new variable. Inner shadowing ends when inner scope is over
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");
    }

    println!("The number of seconds in 3 hours is: {THREE_HOURS_IN_SECONDS}");
    println!("The value of x is: {x}");

    data_types();
    functions(5, 'w');
    control_flow();
}

fn data_types() {
    // Rust is a statically typed language, which means it must know the types of all variables at compile time
    // Compiler can usually infer what type we want to use based on the value and how we use it
    // In the following case, many types are possible, so we add a type annotation:
    let guess: u32 = "42".parse().expect("Not a number!");

    // 4 primary scalar types:
    // - Integers
    //      - i8, u8, i16, u16, i32, u32, i64, u64, i128. u128, isize, usize
    //      - isize and usize depend on the architecture of the computer (64 bits if you're on 64-bit architecture and 32 on 32-bit)
    //      - If overflow occurs, Rust performs two's complement wrapping - still considered an error to rely on that though
    //      - Wrap modes in wrapping_* methods, return None if there is overflow with checked_* methods
    //      - OR return value and boolean indicating whether there was overflow with the overflowing_* methods
    //      - OR saturate at the value's minimum or maximum values with the saturating_* methods
    // - Floating-point numbers
    //      - f32, f64 (default type is f64)
    // - Booleans
    //      - 1 byte in size
    // - Characters
    //      - 4 bytes in size unlike other languages, so can represent a lot more than just ASCII (accented letters, chinese/japanese characters, emojis etc.)

    // 2 primitive compound types:
    // - Tuples
    //      - Groups together a number of values with different types into one compound type
    //      - Have a fixed length and cannot be changed once declared
    // - Arrays
    //      - Every element must have the same type
    //      - Also have a fixed length
    //      - Useful when you want data allocated on stack instead of heap (otherwise use vectors)

    let tup: (i32, f64, u8) = (500, 6.4, 1);
    
    let (x, y, z) = tup; // destructures the tuple
    let z_value = tup.2; // access individual element
    println!("The value of z is: {z_value}");

    let a: [i32; 5] = [1, 2, 3, 4, 5];
    let a = [3; 5]; // same thing as [3, 3, 3, 3, 3]
    let first = a[0]; // access individual element
}

fn functions(value: i32, unit_label: char) {
    // MUST declare the type of each parameter
    println!("The measurement is: {value}{unit_label}");

    // Rust is an expression-based language
    // Each piece of code is either an expression (evaluate to a resultant value) or statements (perform some action and do not return a value)
    // let x = (let y = 6); does NOT work because let y = 6 is a statement and doesn't return anything
    // But creating the following scope block works because it is an expression:
    // Note that the x + 1 line doesn't have a semicolon at the end
    //  - if you add a semicolon to the end of an expression, you turn it into a statement, and it doesn't return a value
    let y = {
        let x = 3;
        x + 1
    };

    fn five() -> i32 {
        // You can return early by using the "return" keyword and specifying a value
        // But most functions return the last expression implicitly
        5
    }

    let val = five();
    println!("The value returned from five() is: {val}");
}

fn control_flow() {
    // If else statements can be used as expressions
    let condition = true;
    let number = if condition { 5 } else { 6 };

    // The following statement will fail. Rust compiler expects integer because of the value within if block
    // let number = if condition { 5 } else { "six" };

    println!("The value of number is: {number}");

    // Can also return values from loops
    let mut counter = 0;
    let result = loop {
        counter += 1;
        
        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is {result}");

    // Loop labels to disambiguate between multiple loops
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

    // While loops
    let mut number = 3;

    while number != 0 {
        println!("{number}!");

        number -= 1;
    }

    // For loop to iterate through arrays
    let a = [10, 20, 30, 40, 50];

    for element in a {
        println!("the value is: {element}");
    }

    for number in (1..4).rev() {
        println!("{}", a[number]);
    }
}