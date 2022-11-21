
use std::num::ParseIntError;

// We can explicitly handled the errors using combinators. 

// Another way to deal with this case analysis is to use a combination of match statements and early returns.

// That is, we can simply stop executing the function and return the error if one occurs. 

fn multiply(first_number_str: &str, second_number_str: &str) -> Result<i32, ParseIntError> {
    let first_number = match first_number_str.parse::<i32>() {
        Ok(first_number)  => first_number,
        Err(e) => return Err(e), // early return Error
    };

    let second_number = match second_number_str.parse::<i32>() {
        Ok(second_number)  => second_number,
        Err(e) => return Err(e), // // early return Error
    };

    Ok(first_number * second_number)
}

// simplify the processing by ? to get variable out, or else return Error Result:
fn multiply2(first_number_str: &str, second_number_str: &str) -> Result<i32, ParseIntError> {
    let first_number = first_number_str.parse::<i32>()?;
    let second_number = second_number_str.parse::<i32>()?;

    Ok(first_number * second_number)
}


fn print(result: Result<i32, ParseIntError>) {
    match result {
        Ok(n)  => println!("n is {}", n),
        Err(e) => println!("Error: {}", e),
    }
}

fn main() {
    print(multiply("10", "2"));
    print(multiply("t", "2"))
    print(multiply2("10", "2"));
    print(multiply2("t", "2"));
}
