
/// Partial Order Set: 
///   The word partial in the names "partial order" and "partially ordered set" is used 
///   as an indication that not every pair of elements needs to be comparable. 
/// https://en.wikipedia.org/wiki/Partially_ordered_set#Partial_order
/// 
/// The definition reads as:
/// The function largest is generic over some type T. 
/// This function has one parameter named list, which is a slice of values of type T. 
/// The largest function will return a reference to a value of the same type T.
fn largest<T:PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn main() {
    let number_list = vec![34, 50, 25, 100, 65];

    let result = largest(&number_list);
    println!("The largest number is {}", result);

    let char_list = vec!['y', 'm', 'a', 'q'];

    let result = largest(&char_list);
    println!("The largest char is {}", result);
}
