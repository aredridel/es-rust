use var::{Vars,Lookup};
use list::List;
use std::rc::Rc;
use term::Term;

pub fn runinitial(mut vars: Vars) -> Vars {
    vars.insert("fn-true".to_string(), List::cons(Term { str: "result".to_string() }, List::cell(Term { str: "0".to_string() })));
    vars.insert("fn-%eval-noprint".to_string(), List::cell(Term { str: "".to_string() }));
    return vars;
}
