

fn main() {
    // parse string to i64:
    let i = "123".parse::<i64>();
    println!("{:?}", i);
    
    // concatenate two string:
    let world = String::from( "world");
    let r2 = "hello ";
    let a = r2.to_owned() + &world;
    println!("a={a}");
}
