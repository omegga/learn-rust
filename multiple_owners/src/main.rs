fn main() {
    not_working();
    graph();
    count_references();
}

fn not_working() {
    enum List {
        Cons(i32, Box<List>),
        Nil,
    }
    use List::{Cons, Nil};
    let a = Cons(5, Box::new(Cons(10, Box::new(Nil))));
    // here, a is moved into b
    let b = Cons(3, Box::new(a));
    // let c = Cons(4, Box::new(a)); // this won't work
}

fn graph() {
    use std::rc::Rc;
    enum List {
        Cons(i32, Rc<List>),
        Nil,
    }
    use List::{Cons, Nil};
    let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    // Rc::clone increments the reference count.
    // The data won't be cleaned up unless there are zero references to it.
    // We could use a.clone() to do the same job, but Rc::clone is preferred
    // as a convention, because it is distinguishable from deep-copy kinds of clones.
    let b = Cons(3, Rc::clone(&a)); // reference count = 2
    let c = Cons(4, Rc::clone(&a)); // reference count = 3
}

fn count_references() {
    use std::rc::Rc;
    #[derive(Debug)]
    enum List {
        Cons(i32, Rc<List>),
        Nil,
    }
    use List::{Cons, Nil};
    let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    println!("*a = {:?}", *a);
    println!("count after creating a = {}", Rc::strong_count(&a));
    let b = Cons(3, Rc::clone(&a));
    println!("count after creating b = {}", Rc::strong_count(&a));
    {
        let c = Cons(4, Rc::clone(&a)); // reference count = 3
        println!("count after creating c = {}", Rc::strong_count(&a));
    }
    // reference count = 2
    println!("count after c goes out of scope = {}", Rc::strong_count(&a));
}
