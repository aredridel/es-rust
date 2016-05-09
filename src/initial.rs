use var::Vars;
use list::List::{Cons, Nil};
use term::Term::Str;
use std::rc::Rc;

pub fn runinitial(mut vars: Vars) -> Vars {
    // Primitive defns go here
    vars.insert("fn-true".to_string(),
                Rc::new(Cons(Str("result".to_string()),
                             Rc::new(Cons(Str("0".to_string()), Rc::new(Nil))))));
    vars.insert("fn-false".to_string(),
                Rc::new(Cons(Str("result".to_string()),
                             Rc::new(Cons(Str("1".to_string()), Rc::new(Nil))))));
    vars.insert("fn-%eval-noprint".to_string(), Rc::new(Nil));
    return vars;
}
