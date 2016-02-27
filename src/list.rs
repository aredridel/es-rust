/* list.rs -- operations on lists */

extern crate libc;

use term::Term;
use std::rc::Rc;

pub enum List<T> {
    Cons(T, Rc<List<T>>),
    Nil,
}

impl<T> List<T> {
    pub fn cons(term: T, l: Rc<List<T>>) -> Rc<List<T>> {
        return Rc::new(List::Cons(term, l));
    }

    pub fn cell(term: T) -> Rc<List<T>> {
        return Rc::new(List::Cons(term, Rc::new(List::Nil)));
    }
}

// allocation and garbage collector support

// DefineTag(List, static);

// extern List *mklist(Term *term, List *next) {
// 	gcdisable();
// 	assert(term != NULL);
// 	Ref(List *, list, gcnew(List));
// 	list->term = term;
// 	list->next = next;
// 	gcenable();
// 	RefReturn(list);
// }
//
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
pub fn listify(argv: Vec<String>) -> Rc<List<Term>> {
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
