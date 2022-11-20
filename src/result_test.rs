#[derive(Debug)]
enum Version { Version1, Version2 }

// the function return a Result<T,E>, where Version is the returned type on Valide result.
// str is the Error type for an error message. 'static is a lifetime indicator.
// 'static indicates that the data pointed to by the reference lives for the entire lifetime of the running program.
fn parse_version(header: &[u8]) -> Result<Version, &'static str> {

    // Arrays can be safely accessed using `.get`, which returns an
    // `Option`. This can be matched as shown below, or used with
    // `.expect()` if you would like the program to exit with a nice
    // message instead of happily continue.
    match header.get(0) {
        None => Err("invalid header length"),
        Some(&1) => Ok(Version::Version1),
        Some(&2) => Ok(Version::Version2),
        Some(_) => Err("invalid version"),
    }
}

fn main() {
    let version = parse_version(&[1, 2, 3, 4]);
    match version {
        Ok(v) => println!("working with version: {:?}", v),
        Err(e) => println!("error parsing header: {:?}", e),
    }
    
    // match string 
    let line = String::from("hello world");
    match line.as_str() {
        "a" => println!("0"),
        "b" => println!("1"),
        "c" => println!("2"),
        _ => println!("something else!"),
    }
}
