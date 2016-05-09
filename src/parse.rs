
use es::Es;
use list::List;
use term::Term;
use std::rc::Rc;
use prim;

pub trait Parse {
    fn parse(&self) -> Result<List, &'static str>;
}

impl Parse for Es {
    fn parse(&self) -> Result<List, &'static str> {
        println!("parse");
        Ok::<List, &'static str>(List::Cons(Term::Prim(prim::echo), Rc::new(List::Nil)))
    }
}
