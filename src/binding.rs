extern crate libc;
use list::List;
use term::Term;

pub struct Binding {
    #[allow(dead_code)]
    name: *mut libc::c_char,
    #[allow(dead_code)]
    defn: Box<List<Term>>,
    #[allow(dead_code)]
    next: Option<Box<Binding>>,
}
