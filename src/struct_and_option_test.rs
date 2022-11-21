//One difference between Rust and C++ is that data and behaviour are much more strictly separated in Rust than C++ (or Java, or other OO languages).
// Behaviour is defined by functions and those can be defined in traits and impls (implementations), 
// but traits cannot contain data, they are similar to Java's interfaces in that respect.

// https://github.com/nrc/r4cppp/blob/master/data-types.md

// 


struct S {
    field1: i32,
    field2: SomeOtherStruct
}

// You can't have cycles of struct names involving definitions and field types
// If you need such a structure then you should use some kind of pointer; cycles with pointers are allowed:
struct R {
    value:  i64,
    // parent: Option<R>,     // illegal definition
    parent: Option<Box<R>>,
    //If we didn't have the Option in this struct, there would be no way to instantiate the struct and Rust would signal an error.
}


// Options
// Option has two variants - Some and None. None has no data and Some has a single field with type T.
use std::rc::Rc;

struct Node {
    parent: Option<Rc<Node>>, // Rc is Reference Counter Pointer.
    value: i32
}

fn is_root(node: Node) -> bool {
    // Using Option is safer because you must always check it before use;
    // Here, the parent field could be either a None or a Some containing an Rc<Node>
    match node.parent {
        Some(_) => false,
        None => true
    }
  
  // There are also convenience methods on Option, 
  // so you could write the body of is_root as node.parent.is_none() or !node.parent.is_some().
}

// https://github.com/nrc/r4cppp/blob/master/rc-raw.md

// RC: Reference Counter Pointer
// A reference counted pointer to an object of type T has type Rc<T>. You create reference counted pointers using a static method: 
// Rc::new(...)

// As with the other pointer types, the . operator does all the dereferencing you need it to. You can use * to manually dereference.


use std::rc::Rc;

fn bar(x: Rc<i32>) { }

fn baz(x: &i32) { }

fn foo() {
    let x = Rc::new(45);
    bar(x.clone());   // Increments the ref-count, // To pass a ref-counted pointer you need to use the clone method. 
    
    baz(&*x);         // Does not increment, // You can take a (borrowed) reference to the pointed at value
    println!("{}", 100 - *x);
}  // Once this scope closes, all Rc pointers are gone, so ref-count == 0
   // and the memory will be deleted.

// Cell and RefCell are structs which allow you to 'cheat' the mutability rules. 
// if you want a mutable, ref counted object you need a Cell or RefCell wrapped in an Rc. (for objects with move semantics)
// So, for a mutable, ref-counted int you would use Rc<Cell<int>>.

// Use Cell for types which have copy semantics (pretty much just primitive types).
// Use RefCell for types which have move semantics, that means nearly everything in Rust.

// *T - raw pointers
// Rust has two kinds of raw pointers (aka unsafe pointers): 
//    *const T for an immutable raw pointer, 
//    and *mut T for a mutable raw pointer. 
// They are created using & or &mut (you might need to specify a type to get a *T rather than a &T since the & operator can create either a borrowed reference or a raw pointer).
// Raw pointers are like C pointers, just a pointer to memory with no restrictions on how they are used

// unsafe code
// Rust has the concept of unsafe blocks, marked by the unsafe keyword. In unsafe code you can do unsafe things - dereference a raw pointer, 
//   index into an array without bounds checking, call code written in another language via the FFI, or cast variables. 

// An example of using an raw pointer:

fn foo() {
    let mut x = 5;
    let x_p: *mut i32 = &mut x;
    println!("x+5={}", add_5(x_p));
}

fn add_5(p: *mut i32) -> i32 {
    unsafe {
        if !p.is_null() { // Note that *-pointers do not auto-deref, so this is
                          // a method implemented on *i32, not i32.
            *p + 5
        } else {
            -1            // Not a recommended error handling strategy.
        }
    }
}


