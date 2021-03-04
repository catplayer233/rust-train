use std::mem::drop;
use std::rc::Rc;

use box_explorer::new_box;
use rc_explorer::RCList;

use crate::box_explorer::List::{Cons, Nil};
use crate::deref_drop_explorer::{CustomSmartPointer, hello, MyBox};

mod box_explorer;
mod deref_drop_explorer;
mod rc_explorer;
mod reference_cycle_explorer;
mod weak_reference_explorer;

fn main() {
    let i32_box = new_box(1);
    println!("i32:{}", i32_box);
    //it's ok to use recursive struct with Box
    let list = Cons(1, new_box(Cons(2, new_box(Cons(3, new_box(Nil))))));

    let x = 5;
    let y = MyBox::new(5);
    assert_eq!(x, *y);

    let name_box = MyBox::new(String::from("catplayer"));
    //no deref coercion
    hello(&(*name_box)[..]);
    //deref coercion
    hello(&name_box);

    //call drop with reverse order of their creation
    let c = CustomSmartPointer {
        data: String::from("my stuff"),
    };
    drop(c);
    let d = CustomSmartPointer {
        data: String::from("other stuff"),
    };

    println!("CustomSmartPointers created.");

    let a = Rc::new(RCList::Cons(
        5,
        Rc::new(RCList::Cons(10, Rc::new(RCList::Nil))),
    ));
    let b = RCList::Cons(3, Rc::clone(&a));
    let c = RCList::Cons(4, Rc::clone(&a));
}

#[cfg(test)]
mod test {
    use std::borrow::BorrowMut;
    use std::cell::RefCell;
    use std::rc::{Rc, Weak};

    use crate::reference_cycle_explorer::List::{Cons, Nil};
    use crate::weak_reference_explorer::Node;

    #[test]
    fn test_reference_cycle() {
        //build a reference count smart pointer a for List type
        let a = Rc::new(Cons(RefCell::new(Rc::new(Nil))));

        println!("a initial rc count = {}", Rc::strong_count(&a));
        println!("a next item = {:?}", a.tail());
        let b = Rc::new(Cons(RefCell::new(Rc::clone(&a))));

        println!("a rc count after b creation = {}", Rc::strong_count(&a));
        println!("b initial rc count = {}", Rc::strong_count(&b));
        println!("b next item is {:?}", b.tail());

        if let Some(link) = a.tail() {
            *link.borrow_mut() = Rc::clone(&b);
        }

        println!("b rc count after changing a = {}", Rc::strong_count(&b));
        println!("a rc count after changing a = {}", Rc::strong_count(&a));

        // println!("a next item = {:?}", a.tail());
    }

    #[test]
    fn weak_reference_test() {
        let leaf = Rc::new(Node {
            value: 3,
            parent: RefCell::new(Weak::new()),
            children: RefCell::new(vec![]),
        });
        //None
        println!("leaf's parent = {:?}", leaf.parent.borrow().upgrade());
        println!("leaf strong = {}, weak = {}", Rc::strong_count(&leaf), Rc::weak_count(&leaf));

        {
            let branch = Rc::new(Node {
                value: 5,
                parent: RefCell::new(Weak::new()),
                children: RefCell::new(vec![leaf.clone()]),
            });

            //branch's ref downgrade to weak
            *leaf.parent.borrow_mut() = Rc::downgrade(&branch);
            //Some->branch
            println!("leaf's parent = {:?}", leaf.parent.borrow().upgrade());
            println!(
                "branch strong = {}, weak = {}",
                Rc::strong_count(&branch),
                Rc::weak_count(&branch),
            );

            println!(
                "leaf strong = {}, weak = {}",
                Rc::strong_count(&leaf),
                Rc::weak_count(&leaf),
            );
        }


        println!(
            "leaf strong = {}, weak = {}",
            Rc::strong_count(&leaf),
            Rc::weak_count(&leaf),
        );
    }
}
