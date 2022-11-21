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
    parent: Option<Rc<Node>>, //
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



