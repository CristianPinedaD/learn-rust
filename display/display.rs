use std::fmt; 

struct Structure(i32); 

//To use the '{}' marker, the trait for fmt::Display must be implemented
// Manually for the type 

impl fmt::Display for Structure {
    // This trait requires 'fmt' with this exact signature.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Write strictly the first element into the supplied output
        // stream : 'f'. Returns 'fmt::Result' which indicates whether it succeeded or failed.
        // Note that 'write!' uses syntax which is very simiar to 'println!'
        write!(f, "{}", self.0)
    }
}

// A structure holding two numbers. 'Debug' will be derived so the results 
// can be contrasted with 'Display'. 

#[derive(Debug)]
struct MinMax(i64, i64);

// Implement 'Display' for 'MinMax'
impl fmt::Display for MinMax {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Use 'self.number' to refer to each positional data point. 
        write!(f, "({}, {})", self.0, self.1)
    }
}

// Define a structure where the fields are nameable for comparison.
#[derive(Debug)]
struct Point2D {
    x: f64, 
    y: f64,
}
// Similarly, implement 'Display' for 'Point2D' 
impl fmt::Display for Point2D {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Customize so only 'x' and 'y' are denoted.
        write!(f, "x: {}, y: {}", self.x, self.y)
    }
}

// Activity: 

    /*
        After checking the output of the above example,
        use the Point2D struct as a guide to add a 
        Complex struct to the example. When printed in the 
        same way, the output should be:

        Display: 3.3 + 7.2i 
        Debug: Complex { real: 3.3, imag: 7.2}
     */

#[derive(Debug)]
struct Complex {
    real: f64, 
    imag: f64, 
}

impl fmt::Display for Complex {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} + {}i", self.real, self.imag)
    }
}

fn main() {
    let minmax = MinMax(0, 14);

    println!("Compare structures:");
    println!("Display: {}", minmax);
    println!("Debug: {:?}", minmax);

    let big_range = MinMax(-300, 300); 
    let small_range = MinMax(-3, 3); 

    println!("The big range is {big}, and the small is {small}",
                small = small_range, 
                big = big_range);
    
    let point = Point2D {x: 3.3, y: 7.2}; 

    println!("Compare points:");
    println!("Display: {}", point);
    println!("Debug: {:?}", point);

    let complex = Complex{real: 3.3, imag: 7.2}; 

    println!("Compare complexes:");
    println!("Display: {}", complex);
    println!("Debug: {:?}", complex); 
}
