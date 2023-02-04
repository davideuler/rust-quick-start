use std::fmt::Display;
use std::fmt::Debug;

/// we can implement a trait on a type only if at least one of the trait or the type is local to our crate. 
/// https://doc.rust-lang.org/book/ch10-02-traits.html
/// 
pub trait Summary<T> {
    fn summarize(&self) -> String;
}

pub trait SummaryWithDefaultImpl {
    fn summarize(&self) -> String {
        String::from("(Read more...)")
    }
}

impl<T> Summary<T> for Vec<T>
where 
    T: Display + Copy
{
    fn summarize(&self) -> String {
        return format!("vec length:{}", self.len());
    }
}

// Traits as Parameters
pub fn notify<T>(item: &impl Summary<T>) {
    println!("Breaking news! {}", item.summarize());
}

// Trait Bound Syntax
pub fn notify2<X, T: Summary<X>>(item: &T) {
    println!("Breaking news! {}", item.summarize());
}

// Specifying Multiple Trait Bounds with the + Syntax
pub fn notify3<T>(item: &(impl Summary<T> + Display)) {
}

// 
pub fn notify4<X, T: Summary<X> + Display>(item: &T) {}

// Multiple Trait Bounds is long to read
fn some_function1<T: Display + Clone, U: Clone + Debug>(t: &T, u: &U) -> i32 {200}

// More easier to read with where Clauses:
fn some_function2<T, U>(t: &T, u: &U) -> i32
where
    T: Display + Clone,
    U: Clone + Debug,
{
    100
}

// https://doc.rust-lang.org/book/ch10-02-traits.html
// We can also conditionally implement a trait for any type that implements another trait. 
// Implementations of a trait on any type that satisfies the trait bounds are called __blanket implementations__ and are extensively used in the Rust standard library.
// For example, the standard library implements the ToString trait on any type that implements the Display trait. 
//    The impl block in the standard library looks similar to this code:
// impl<T: Display> ToString for T { ... }
//
// Because the standard library has this blanket implementation, 
// we can call the to_string method defined by the ToString trait on any type that implements the Display trait. 
// 
//   For example:
//   let s = 3.to_string();


struct Pair<T> {
    x: T,
    y: T,
}

impl<T> Pair<T> {
    fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

// conditionally implement a trait for a type that implements trait of Display + PartialOrd. 
impl<T: Display + PartialOrd> Pair<T> {
    fn cmp_display(&self) {
        if self.x >= self.y {
            println!("The largest member is x = {}", self.x);
        } else {
            println!("The largest member is y = {}", self.y);
        }
    }
}

// you can implement Display for your wrapper type
use std::fmt;

struct Foo(Vec<u8>);

impl fmt::Display for Foo {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Values:\n")?;
        for v in &self.0 {
            write!(f, "\t{}", v)?;
        }
        Ok(())
    }
}

fn main(){
    let p = Pair::new(50u64, 60u64);
    p.cmp_display();

    let f = Foo(vec![42, 35, 68]);
    println!("vector: {}", f);

    let v = vec![100, 102, 103];
    println!("summary, {}", v.summarize());
}