fn main() {
    // Strings

    // string slices (&str)
    // Are fixed in size and immutable by default
    let some_string = "Fixed length string slice";

    // String type
    // Can grow and shrink, used on a myriad of string operations
    // Stored on the heap
    let mut growable_string = String::from("This String can grow in size!");
    println!("The value of growable_string is: '{}'", growable_string);

    // Appending chars to the end of the String
    growable_string.push('Q');
    println!("Growable string after pushing Q: '{}'", growable_string);

    // Removing from the end of the String
    growable_string.pop();
    println!(
        "Growable string after popping the last value: '{}'",
        growable_string
    );

    // Pushing &str to the end of the String
    growable_string.push_str(" And you can too!");
    println!("Growable string after push_str: '{}'", growable_string);

    // Common String operations
    println!("Basic string methods");
    println!("is_empty(): {}", growable_string.is_empty());
    // Length in bytes, for length in chars use chars().count()
    println!("len(): {}", growable_string.len());
    println!("bytes(): {}", growable_string.capacity());
    println!("contains(): {}", growable_string.contains("string"));

    // Chaining String methods
    growable_string.push_str("      ");
    let string_len = growable_string.trim().len();
    println!("The growable string len in bytes is: {}", string_len);

    // Converting other data types to Strings
    let number = 5;
    let num_to_str = number.to_string();

    let charr = 'A';
    let char_to_str = charr.to_string();

    let string_slice = "Slice";
    let slice_to_str = string_slice.to_string();

    // Creating empty Strings
    let empty_string = String::new();

    // Concatenating Strings using format!() macro
    let first_name = "Lucin";
    let last_name = "Santiago Santana";
    let full_name = format!("My full name is {} {}", first_name, last_name);
    println!("Full name: {}", full_name);
}
