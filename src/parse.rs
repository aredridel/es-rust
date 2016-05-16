
use es::Es;
use list::List::{Cons, Nil};
use list::List;
use term::Term::{Prim, Str};
use std::rc::Rc;

impl Es {
    pub fn parse(&self, s: &str) -> Result<List, &'static str> {
        println!("parse");
        Ok(Cons(Prim("debug".to_string()),
                Rc::new(Cons(Prim("debug".to_string()),
                             Rc::new(Cons(Str("hello, world!".to_string()), Rc::new(Nil)))))))
    }
}
