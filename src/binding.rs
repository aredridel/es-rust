extern crate libc;
use list;
use term;

pub struct Binding {
#[allow(dead_code)]
	name: *mut libc::c_char,
#[allow(dead_code)]
    defn: Box<list::List>,
#[allow(dead_code)]
    next: Option<Box<Binding>>
}
