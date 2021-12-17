use std::ops::Deref;

enum List {
    Cons(i32, Box<List>),
    Nil,
}

struct MyBox<T>(T);

impl <T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

impl <T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target  {
        &self.0
    }
}

fn main() {
    let b = Box::new(5);
    println!("b = {}", b);

    let x = 5;
    let y = &x;

    assert_eq!(5, x);
    assert_eq!(5, *y);
    // assert_eq!(3, *y);
    
    println!("{}  {:#?}", y, *y);
    
    let y = Box::new(x);
    
    println!("{}  {:#?}", y, *y);
    
    let y = MyBox::new(x);
    assert_eq!(5, x);
    assert_eq!(5, *y);
    
    println!("{:?}", *y);


}
