fn main() {
    // In general, the `{}` will be automatically replaced with any arguments. These will be stringified.
    println!("{} days", 31);

    //Positional arguments can be used. Specifying an integer inside `{}` determines which additional argument will be replaced. Arguments start at 0 immediately after the format string.
    println!("{0} this is {1}. {1} this is {0}", "Daniel","Eve");

    // As can named arguments.
    println!("{subject} {verb} {object}",
        subject="the quick brown fox",
        object="the lazy dog",
        verb="jumps over");

    // Different formatting can be invoked by specifying the format character after a `:`.
    println!("Base 10: {}",69420); //69420
    println!("Base 2 (binary): {:b}",69420); //10000111100101100
    println!("Base 8 (octal): {:o}",69420); //207454
    println!("Base 16 (hexadecimal): {:x}",69420); //10f2c

    // You can right-justify text with a specified width. This will output "    1". (Four white spaces and a "1", for a total width of 5.)
    println!("{number:>5}", number = 1);

    // You can pad numbers with extra zeroes,
    println!("{number:0>5}", number = 1);//00001
    // and left-adjust by flipping the sign.
    println!("{number:0<5}", number = 1);//10000

    // You can use named arguments in the format specifier by appending a `$`.
    println!("{number:0>width$}", number = 1, width = 5);

    // Rust even checks to make sure the correct number of arguments are used.
    println!("My name is {0} {1} {0}", "James","Bond");

    #[allow(dead_code)]
    struct Structure (i32);
    // This will not compile because `Structure` does not implement
    // fmt::Display.
    // println!("This struct `{}` won't print...", Structure(3));
    // TODO ^ Try uncommenting this line

    // For Rust 1.58 and above, you can directly capture the argument from a
    // surrounding variable. Just like the above, this will output
    //00001
    let number:f64 = 1.0;
    let width:usize = 5;
    println!("{number:0>width$}"); 

    //Add a println! macro call that prints: Pi is roughly 3.142 by controlling the number of decimal places shown. 
    //For the purposes of this exercise, use let pi = 3.141592 as an estimate for pi. 
    //(Hint: you may need to check the std::fmt documentation for setting the number of decimals to display)

    let pi:f64 = 3.141592;
    println!("pi is roughly {:.3}",pi);
}
