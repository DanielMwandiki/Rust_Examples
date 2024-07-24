use std::fmt; // Import `fmt`
/*
// Import (via `use`) the `fmt` module to make it available.
use std::fmt;

// Define a structure for which `fmt::Display` will be implemented. This is
// a tuple struct named `Structure` that contains an `i32`.
struct Structure(i32);

// To use the `{}` marker, the trait `fmt::Display` must be implemented
// manually for the type.
impl fmt::Display for Structure {
    // This trait requires `fmt` with this exact signature.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Write strictly the first element into the supplied output
        // stream: `f`. Returns `fmt::Result` which indicates whether the
        // operation succeeded or failed. Note that `write!` uses syntax which
        // is very similar to `println!`.
        write!(f, "{}", self.0)
    }
}
*/

// A structure holding two numbers. `Debug` will be derived so the results can
// be contrasted with `Display`.
#[derive(Debug)]
struct MinMax(i64,i64);

// Implement `Display` for `MinMax`.
impl fmt::Display for MinMax {

    fn fmt(&self, f:&mut fmt::Formatter) -> fmt::Result {
        // Use `self.number` to refer to each positional data point.
        write!(f, "({} {})", self.0, self.1)
    }
}

// Similarly, implement `Display` for `Point2D`.

#[derive(Debug)]
struct Point2D {
    x:f64,
    y:f64
}

impl fmt::Display for Point2D {

    fn fmt(&self, f:&mut fmt::Formatter) -> fmt::Result {

        write!(f, "x: {} y: {}", self.x, self.y)
    }
}

// Similarly, implement `Display` for `Complex`.

#[derive(Debug)]

struct Complex{
    real:f64,
    imag:f64
}

impl fmt::Display for Complex{

    fn fmt(&self, f:&mut fmt::Formatter) -> fmt::Result {

        write!(f, " {} + {}i", self.real,self.imag)
    }
}


fn main() {
    //MinMax
    let minmax = MinMax(0, 14);

    println!("Compare Structures:");
    println!("Dispaly: {}", minmax);
    println!("Debug: {:?}", minmax);

    let big_range = MinMax(-300, 300);
    let small_range = MinMax(-3, 3);

    println!("The big range is {big}, the small range is {small}.",
            big = big_range,
            small = small_range);

    //Point2D
    let point = Point2D{x:3.3, y:7.2};

    println!("Diplay: {}", point);
    println!("Debug: {:?}", point);
    
    //Complex

    let complex = Complex{real:3.3,imag:7.2};

    println!("Complex");
    println!{"Display: {}", complex};
    println!("Debug: {:?}", complex);
}