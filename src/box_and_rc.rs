use std::rc::Rc;

/*
Box<T> is for single ownership.
Rc<T> is for multiple ownership.
Arc<T> is for multiple ownership, but threadsafe.
Cell<T> is for "interior mutability" for Copy types;
    that is, when you need to mutate something behind a &T.
*/

fn main() {
    // if you don't need shared access, use Box;
    //    otherwise, use Rc (or Arc for multi-threaded shared usage)
    //    and keep in mind you will be needing Cell or RefCell for internal mutability.
    //  Box provides exclusive ownership and thus mutation is allowed:
    let mut a = Box::new(1);
    *a = 5;
    let mut a1 = &mut a;
  
    // when a1 is &mut, a could not be borrowed again:
    // let a2 = &a; // cannot borrow `a` as immutable because it is also borrowed as mutable
    //*a = 6;
    *a1 = Box::new(7);

    // Rc provides shared ownership so by default its contents can't be mutated
    // let mut b = Rc::new(1); // Rc<T> should not be mutable, error occur
    // *b = 6; // cannot assign to data in an `Rc`, error occur
    let b = Rc::new(2);  // multiple ownership, immutable
    let b1 = b.clone();
    let b2 = b.clone();

    // println!("a1={} a2={}", a1, a2); //=> 1 1
    println!("a={} ", a ); //
    println!("b1={} b2={}", b1, b2); //=> 1 1
}
