/* prim-ctl.rs -- control flow primitives */
use list::List;
use list::List::Nil;
use var::Vars;
use prim::Prims;

#[allow(unused_variables)]
fn seq(vars: &Vars, lp: &List /* , binding: &Binding, evalflags: int */) -> List {
    return Nil;
}

#[allow(unused_variables)]
fn _if(vars: &Vars, lp: &List) -> List {
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
    return Nil;
}

#[allow(unused_variables)]
fn forever(vars: &Vars, lp: &List) -> List {
    /* Ref(List *, body, list);
     * for (;;)
     * list = eval(body, NULL, evalflags & eval_exitonfalse);
     * RefEnd(body);
     * return list;
     * */
    return Nil;
}

#[allow(unused_variables)]
fn throw(vars: &Vars, lp: &List) -> List {
    /* if (list == NULL)
     * fail("$&throw", "usage: throw exception [args ...]");
     * throw(list);
     * NOTREACHED;
     * */
    return Nil;
}

#[allow(unused_variables)]
fn catch(vars: &Vars, lp: &List) -> List {
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
    return Nil;
}

pub fn initprims_controlflow(prims: &mut Prims) {
    prims.insert("seq".to_string(), seq);
    prims.insert("if".to_string(), _if);
    prims.insert("throw".to_string(), throw);
    prims.insert("forever".to_string(), forever);
    prims.insert("catch".to_string(), catch);
}
