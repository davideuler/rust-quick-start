fn main() {
    let a =3;
    let b =1;
    println!("b={b}");

    let str = String::from("hello world");
    let another_str = str;
    //println!("borrowed string:{str}"); //  value borrowed here after move

    
    //Only one mutable reference in a particular scope. Prevents datarace 
    let mut s = String::from("hello");
    let r1 = &mut s; // first mutable borrow occurs here
    // let r2 = &mut s; // error[E0499]: cannot borrow s as mutable more than once at a time 
    // println!("r2={r2} r1={r1}"); // error: r1 - first borrow later used here
    
    //Combination of mutable and immutable references are not allowed.  to guarantee immutability. 
    let mut s = String::from("hello");
    let r1 = &s; // no problem (immutable)
    let r2 = &s; // no problem (immutable)
    // let r3 = &mut s; //  error[E0502]: cannot borrow s as mutable because it is also borrowed as immutable

}
