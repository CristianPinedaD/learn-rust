fn main() {
    // In general, the {} will be automatically replaced with any arguments
    // These will be stringified

    println!("{} days", 31); // Prints 31 days

    // Positional arguments can be used. Specifying an integer inside '{}'
    // determines which additional argument will be replaced.

    println!("{0}, this is {1}. {1}, this is {0}", "Alice", "Bob");
    // Alice, this is Bob. Bob, this is Alice

    // As can be named arguments

    println!("{subject} {verb} {object}",
                object="the lazy dog",
                subject="the quick brown fox",
                verb="jumps over");
    
    // Different formatting can be invoked by specifying the format character
    // after a ':'

    println!("Base 10:              {}", 69420); //69420
    println!("Base 2 (binary):      {:b}", 69420); // 10000111100101100
    println!("Base 8 (octal):       {:o}", 69420); // 207454
    println!("Base 16 (hex)         {:x}", 69420); // 10f2c

    // You can right-justify text with a specified width.
    // This will output "   1" (4 whitespaces and a 1, for a total width of 5)

    println!("{number:>5}", number=1);

    // You can pad numbers with extra zeros,
    println!("{number:0>5}", number=1); // 00001
    // Left adjust-them by flipping the sign
    println!("{number:0<5}", number=1);

    // You can use named arguments in the format specifier by appending a $
    println!("{number:0>width$}", number=1, width=5); 

    // Rust will even check to make sure the correct number of args is used

    println!("My name is {0}, {1}, {0}", "Bond", "James");

    // Only types that implement fmt::Display can be formatted with {}
    // User-defined types do not implement fmt::Display by default.

    #[allow(dead_code)] // disable 'dead_code', which warn against unused module

    

    use std::fmt; 

    struct Structure(i32);

    impl fmt::Display for Structure {
        // this trait requires 'fmt' with this exact signature.
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            // Write strictly the first element into the supplied output
            // stream: 'f'. Returens 'fmt::Result' which indicates
            // whether the operation is succeeded or failed. Note that
            // write! uses syntax which is very similar to println!
            write!(f, "{}", self.0)
        }
    }

    // This will not compile because 'Structure' does not implement
    // fmt::Display.

    println!("This struct '{}' will not print...", Structure(3));

    // For rust 1.58 and above, you can directly capture the argument
    // from a surrounding variable. Just like the above, this will 
    // output "    1", 4 whitespaces and a 1.

    let number: f64 = 1.0; 
    let width: usize = 5;
    println!("{number:>width$}");

}
