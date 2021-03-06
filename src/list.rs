/* list.rs -- operations on lists */

extern crate libc;

use term::Term;
use std::rc::Rc;
use std::fmt;

#[derive(Clone)]
pub enum List {
    Cons(Term, Rc<List>),
    Cell(Term),
    Nil,
}

impl fmt::Debug for List {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            &List::Cons(ref head, ref rest) => write!(f, "{:?} {:?}", head, rest),
            &List::Cell(ref head) => write!(f, "{:?}", head),
            &List::Nil => write!(f, "nil"),
        }
    }
}

impl List {
    #[allow(unused_variables)]
    pub fn append(&self, tail: &List) -> List {
        unimplemented!();
        // // merge two lists, non-destructively
        // extern List *append(List *head, List *tail) {
        // 	List *lp, **prevp;
        // 	Ref(List *, hp, head);
        // 	Ref(List *, tp, tail);
        // 	gcreserve(40 * sizeof (List));
        // 	gcdisable();
        // 	head = hp;
        // 	tail = tp;
        // 	RefEnd2(tp, hp);
        //
        // 	for (prevp = &lp; head != NULL; head = head->next) {
        // 		List *np = mklist(head->term, NULL);
        // 		*prevp = np;
        // 		prevp = &np->next;
        // 	}
        // 	*prevp = tail;
        //
        // 	Ref(List *, result, lp);
        // 	gcenable();
        // 	RefReturn(result);
        // }
    }
}

// allocation and garbage collector support

// DefineTag(List, static);

// static void *ListCopy(void *op) {
// 	void *np = gcnew(List);
// 	memcpy(np, op, sizeof (List));
// 	return np;
// }
//
// static size_t ListScan(void *p) {
// 	List *list = p;
// 	list->term = forward(list->term);
// 	list->next = forward(list->next);
// 	return sizeof (List);
// }
//

// basic list manipulations

/// destructively reverse a list
// extern List *reverse(List *list) {
// 	List *prev, *next;
// 	if (list == NULL)
// 		return NULL;
// 	prev = NULL;
// 	do {
// 		next = list->next;
// 		list->next = prev;
// 		prev = list;
// 	} while ((list = next) != NULL);
// 	return prev;
// }
//
//
// // make a copy of a list
// extern List *listcopy(List *list) {
// 	return append(list, NULL);
// }
//
// // lenth of a list
// extern int length(List *list) {
// 	int len = 0;
// 	for (; list != NULL; list = list->next)
// 		++len;
// 	return len;
// }
/// turn an argc/argv vector into a list
#[allow(unused_variables)]
pub fn listify(argv: Vec<String>) -> Rc<List> {
    Rc::new(List::Nil)
}
// extern List *listify(int argc, char **argv) {
// 	Ref(List *, list, NULL);
// 	while (argc > 0) {
// 		Term *term = mkstr(argv[--argc]);
// 		list = mklist(term, list);
// 	}
// 	RefReturn(list);
// }

// /// return nth element of a list, indexed from 1
// extern Term *nth(List *list, int n) {
// 	assert(n > 0);
// 	for (; list != NULL; list = list->next) {
// 		assert(list->term != NULL);
// 		if (--n == 0)
// 			return list->term;
// 	}
// 	return NULL;
// }

// extern List *sortlist(List *list) {
// 	if (length(list) > 1) {
// 		Vector *v = vectorize(list);
// 		sortvector(v);
// 		gcdisable();
// 		Ref(List *, lp, listify(v->count, v->vector));
// 		gcenable();
// 		list = lp;
// 		RefEnd(lp);
// 	}
// 	return list;
// }
