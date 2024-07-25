
// Boxes provide only the indirection and heap allocation; they don’t have any other special capabilities, like those we’ll see with the other smart pointer types.

use crate::List::{Cons , Nil}; // import list
use std::ops::Deref;

#[derive(Debug)]
enum List {
    // cons is list with recursive data so we can not find the end of list that why we need to use Box which will point to the heap address of List
    Cons(i32 , Box<List>), 
    Nil // it points end of list
}





struct MyBox<T> (T);

impl<T> MyBox<T> {
    fn new(x:T) -> MyBox<T> {
        MyBox(x)
    } 
}

// implemented this only to support *
impl<T> Deref for  MyBox<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        &self.0 // .0 will access the first elem in tuple
    }
}
fn main() {

    let a = Box::new(5); // it will store the data on the heap
    println!("Simple Box : {a}");
    let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
    println!("List Box with recursive data : {:#?}" , list);

    // first ends here , Box<T> is done here
    // 2nd Treating Smart Pointers Like Regular References with the Deref Trait


    let x = 5 ;
    // let y =x; 
    // now lets change y to point reference of x
    // let y = &x;
    // let y = Box::new(x); // the Box will pass the reference to y
    // with our custom impl 
    let y = MyBox::new(x); // the Box will pass the reference to y


    assert_eq!(5,x);
    // assert_eq!(5, *y); // here it will throw error because we can not dereference the data which get stored at stack
    // assert_eq!(5, *y); // now in this case if we put y instead of *y it will throw error
    assert_eq!(5, *y); // In case of My box it will throw error Becuase we need to implement deref for this type , Now rust will run this line as : *(y.deref())





}
