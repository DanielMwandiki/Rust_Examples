use std::fmt::{self, Formatter, Display};
#[derive(Debug)]

struct Matrix(f32, f32, f32, f32);

//Recap: Add the fmt::Display trait to the Matrix struct in the above example,
// so that if you switch from printing the debug format {:?} to the display format {}

impl Display for Matrix{
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {

        write!(f, "({}, {}, {}, {})", self.0, self.1, self.2, self.3)
        
    }
}



// Tuples can be used as function arguments and as return values.
fn reverse(pair: (i32, bool)) -> (bool, i32) {
    // `let` can be used to bind the members of a tuple to variables.
    let (int_param, bool_param) = pair;

    (bool_param, int_param)
}

fn transpose(matrix:Matrix) -> Matrix {

    Matrix(matrix.0, matrix.2, matrix.1, matrix.3)

}

fn main() {
    // A tuple with a bunch of different types.
    let long_tuple = (1u8, 2u16, 3u32, 4u64,
                    -1i8, -2i16, -3i32, -4i64,
                    0.1f32, 0.2f64,
                    'a', true);
    // Values can be extracted from the tuple using tuple indexing.
    println!("Long tuple first value is {}", long_tuple.0);
    println!("Long tuple second value is {}", long_tuple.1);

    // Tuples can be tuple members.

    let tuple_of_tuples = ((1u8, 2u16, 2u32), (4u64, -1i8), -2i16);
     // Tuples are printable.   
    println!("tuple of tuples: {:?}", tuple_of_tuples);

    //let too_long_tuple = (1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13);

    //println!("Too long tuple: {:?}", too_long_tuple);

    let pair = (1,true);
    println!("Pair is {:?}", pair);

    println!("The reversed pair is {:?}", reverse(pair));

    // To create one element tuples, the comma is required to tell them apart
    // from a literal surrounded by parentheses.
    println!("One element tuple: {:?}", (5u32,));
    println!("Just an integer: {:?}", (5u32));

    // Tuples can be destructured to create bindings.

    let tuple = (1, "hello", 4.5, true);

    let (a,b,c,d) = tuple;
    println!("{:?}, {:?}, {:?}, {:?}", a,b,c,d);

    let matrix = Matrix(1.1, 1.2, 2.1, 2.2);

    println!("Matrix:");
    println!("({}, {})", matrix.0, matrix.1);
    println!("({}, {})", matrix.2, matrix.3);

    let transposed_matrix = transpose(matrix);

    println!("Transpose:");
    println!("({}, {})", transposed_matrix.0, transposed_matrix.1);
    println!("({}, {})", transposed_matrix.2, transposed_matrix.3);
}