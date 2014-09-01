extern crate libc;
use list;
use term;

pub struct Binding {
	name: *libc::c_char,
    defn: Box<list::List>,
    next: Option<Box<Binding>>
}
