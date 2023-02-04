use std::{fmt::Binary, ops::Add};
pub struct Item<T,M>
where 
  T: Add + Binary + Copy,
  M: Send + Sync,
{
    price:T,
    name:M,
}

impl<T,M> Item<T,M> 
where 
    T: Add<Output = T> + Binary + Copy,
    M: Send + Sync,
{
    pub fn new(price:T, name:M) -> Item<T,M> {
        // for struct Generic Types, param position does not matter in struct intitialization
        Item{name, price}
    }

    pub fn add(& self, item: & Item<T,M>) -> T{
        return self.price + item.price;
    }
}

fn main() {

    let notebook = Item::new(50u32, String::from("notebook"));
    let iphone = Item::new(6000u32, String::from("iPhone"));
    println!("{} + {} , total:{}", notebook.name, iphone.name, notebook.add(&iphone));
 
}