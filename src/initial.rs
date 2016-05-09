use var::{Lookup, Vars};
use list::List;
use term::Term;
use std::rc::Rc;

pub fn runinitial(mut vars: Vars) -> Vars {
    // Primitive defns go here
    vars.insert("fn-true".to_string(),
                Rc::new(List::Cons(Term::Str("result".to_string()),
                           List::cell(Term::Str("0".to_string())))));
    vars.insert("fn-false".to_string(),
                Rc::new(List::Cons(Term::Str("result".to_string()),
                           List::cell(Term::Str("1".to_string())))));
    vars.insert("fn-%eval-noprint".to_string(), List::empty());
    return vars;
}
