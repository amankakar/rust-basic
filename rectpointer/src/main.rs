
// Boxes provide only the indirection and heap allocation; they don’t have any other special capabilities, like those we’ll see with the other smart pointer types.

use std::rc::Rc;
#[derive(Debug)]
enum List {
    // cons is list with recursive data so we can not find the end of list that why we need to use Box which will point to the heap address of List
    Cons(i32 , Rc<List>), 
    Nil // it points end of list
}
use crate::List::{Cons , Nil};

fn main() {
// new we need to check for reference of list which is being used by other lists
    // let a = Cons(1, Rc::new(Cons(2, Rc::new(Cons(3, Rc::new(Nil))))));
    // println!("List Rc with recursive data : {:#?}" , a);
    // let b = Cons(4, Rc::new(a));
    //    |                   ^ value used here after move
    // we need something which will help us to share the ownership of a 
    // instead of using the Box we will use Rc here and pass the reference of a
    let a = Rc::new(Cons(1, Rc::new(Cons(2, Rc::new(Cons(3, Rc::new(Nil)))))));
    println!("List Rc with recursive data : {:#?}" , a);

    let b = Cons(4, Rc::clone(&a));

    let c = Cons(5, Rc::clone(&a));
    {
        let d = Cons(6, Rc::clone(&a));
        println!("List Rc with recursive data : {:#?}" , Rc::strong_count(&a));

    }
    println!("List Rc with recursive data : {:#?}" , Rc::strong_count(&a));


}
