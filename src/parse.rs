
use es::Es;
use list::List::{Cons, Nil};
use list::List;
use term::Term::Prim;
use std::rc::Rc;

pub trait Parse {
    fn parse(&self) -> Result<List, &'static str>;
}

impl Parse for Es {
    fn parse(&self) -> Result<List, &'static str> {
        println!("parse");
        Ok::<List, &'static str>(Cons(Prim("echo".to_string()),
                                      Rc::new(Cons(Prim("echo".to_string()), Rc::new(Nil)))))
    }
}
