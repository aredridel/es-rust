/* prim-ctl.rs -- control flow primitives */
use std::collections::{TreeMap};

fn seq(lp: &::list::List) -> ::list::List {
    /*
	for (; l != NULL; l = lp->next)
		result = eval1(lp->term, evalflags &~ (lp->next == NULL ? 0 : eval_inchild));
    */

    return ::list::Nil;
}

fn _if(lp: &::list::List) -> ::list::List {
    /*
	Ref(List *, lp, ::list);
	for (; lp != NULL; lp = lp->next) {
		List *cond = eval1(lp->term, evalflags & (lp->next == NULL ? eval_inchild : 0));
		lp = lp->next;
		if (lp == NULL) {
			RefPop(lp);
			return cond;
		}
		if (istrue(cond)) {
			List *result = eval1(lp->term, evalflags);
			RefPop(lp);
			return result;
		}
	}
	RefEnd(lp);
	return true;
    */
    return ::list::Nil;
}

fn forever(lp: &::list::List) -> ::list::List {
    /*
	Ref(List *, body, ::list);
	for (;;)
		list = eval(body, NULL, evalflags & eval_exitonfalse);
	RefEnd(body);
	return ::list;
    */
    return ::list::Nil;
}

fn throw(lp: &::list::List) -> ::list::List {
    /*
	if (list == NULL)
		fail("$&throw", "usage: throw exception [args ...]");
	throw(list);
	NOTREACHED;
    */
    return ::list::Nil;
}

fn catch(lp: &::list::List) -> ::list::List {
    /*
	Atomic retry;

	if (list == NULL)
		fail("$&catch", "usage: catch catcher body");

	Ref(List *, result, NULL);
	Ref(List *, lp, ::list);

	do {
		retry = FALSE;

		ExceptionHandler

			result = eval(lp->next, NULL, evalflags);

		CatchException (frombody)

			blocksignals();
			ExceptionHandler
				result
				  = eval(mklist(mkstr("$&noreturn"),
					        mklist(lp->term, frombody)),
					 NULL,
					 evalflags);
				unblocksignals();
			CatchException (fromcatcher)

				if (termeq(fromcatcher->term, "retry")) {
					retry = TRUE;
					unblocksignals();
				} else {
		
					unblocksignals();
					throw(fromcatcher);
				}
	   
			EndExceptionHandler

		EndExceptionHandler
	} while (retry);
	RefEnd(lp);
	RefReturn(result);
    */
    return ::list::Nil;
}

pub fn initprims_controlflow(prims: &mut TreeMap<String, |&::list::List| -> ::list::List>) {
    prims.insert("seq".to_string(), seq);
    prims.insert("if".to_string(), _if);
    prims.insert("throw".to_string(), throw);
    prims.insert("forever".to_string(), forever);
    prims.insert("catch".to_string(), catch);
}
