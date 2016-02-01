/* prim-ctl.rs -- control flow primitives */
use std::collections::BTreeMap;
use std::collections::LinkedList;
use term::Term;

#[allow(unused_variables)]
fn seq(lp: &LinkedList<Term> /* , binding: &Binding, evalflags: int */) -> LinkedList<Term> {
    return LinkedList::new();
}

#[allow(unused_variables)]
fn _if(lp: &LinkedList<Term>) -> LinkedList<Term> {
    /* Ref(List *, lp, list);
     * for (; lp != NULL; lp = lp->next) {
     * List *cond = eval1(lp->term, evalflags & (lp->next == NULL ? eval_inchild : 0));
     * lp = lp->next;
     * if (lp == NULL) {
     * RefPop(lp);
     * return cond;
     * }
     * if (istrue(cond)) {
     * List *result = eval1(lp->term, evalflags);
     * RefPop(lp);
     * return result;
     * }
     * }
     * RefEnd(lp);
     * return true;
     * */
    return LinkedList::new();
}

#[allow(unused_variables)]
fn forever(lp: &LinkedList<Term>) -> LinkedList<Term> {
    /* Ref(List *, body, list);
     * for (;;)
     * list = eval(body, NULL, evalflags & eval_exitonfalse);
     * RefEnd(body);
     * return list;
     * */
    return LinkedList::new();
}

#[allow(unused_variables)]
fn throw(lp: &LinkedList<Term>) -> LinkedList<Term> {
    /* if (list == NULL)
     * fail("$&throw", "usage: throw exception [args ...]");
     * throw(list);
     * NOTREACHED;
     * */
    return LinkedList::new();
}

#[allow(unused_variables)]
fn catch(lp: &LinkedList<Term>) -> LinkedList<Term> {
    /* Atomic retry;
     *
     * if (list == NULL)
     * fail("$&catch", "usage: catch catcher body");
     *
     * Ref(List *, result, NULL);
     * Ref(List *, lp, list);
     *
     * do {
     * retry = FALSE;
     *
     * ExceptionHandler
     *
     * result = eval(lp->next, NULL, evalflags);
     *
     * CatchException (frombody)
     *
     * blocksignals();
     * ExceptionHandler
     * result
     * = eval(mklist(mkstr("$&noreturn"),
     * mklist(lp->term, frombody)),
     * NULL,
     * evalflags);
     * unblocksignals();
     * CatchException (fromcatcher)
     *
     * if (termeq(fromcatcher->term, "retry")) {
     * retry = TRUE;
     * unblocksignals();
     * } else {
     *
     * unblocksignals();
     * throw(fromcatcher);
     * }
     *
     * EndExceptionHandler
     *
     * EndExceptionHandler
     * } while (retry);
     * RefEnd(lp);
     * RefReturn(result);
     * */
    return LinkedList::new();
}

pub fn initprims_controlflow(prims: &mut BTreeMap<String, fn(&LinkedList<Term>) -> LinkedList<Term>>) {
    prims.insert("seq".to_string(), seq);
    prims.insert("if".to_string(), _if);
    prims.insert("throw".to_string(), throw);
    prims.insert("forever".to_string(), forever);
    prims.insert("catch".to_string(), catch);
}
