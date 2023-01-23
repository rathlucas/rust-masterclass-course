fn main() {
    // Tuple, compound type that can store multiple values of different types
    let (first_value, second_value) = ('A', 32.0);

    // Large numbers can be represented with an underscore _
    let large_number = 10_000_000;

    // Integer overflow
    // When the assigned value is larger than the integer space
    // Example: let overflow: u8 = 255; -> Doesn't compile

    // Representing numbers on other formats
    let x = 255;
    println!(
        "X in octal: {:o}, X in hexadecimal: {:x}, X in binary: {:b}",
        x, x, x
    );

    // "Type casting"
    let n1 = 10;
    let n2 = 30.2;

    let n3 = n1 as f64 + n2; // as keyword only changes the value for this operation
    println!("{} + {} = {}", n1, n2, n3);

    // Variable shadowing
    let s = 5;
    let s = 5 * 5; // Shadows the first s variable
    println!("The value of s is {}", s);

    let mut p = 5;
    let p = 5 * 5;
    println!("The value of p is {}, and now it is immutable!", p);
    // Reassigning to p would be invalid

    let mut r = 64;
    {
        let r = 60;
        println!("Value of r inside the block: {}", r);
    }
    println!("Value of r outside of the block: {}", r);
}
