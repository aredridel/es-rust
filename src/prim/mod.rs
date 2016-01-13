/* prim.c -- primitives and primitive dispatching ($Revision: 1.1.1.1 $) */

use std::collections::BTreeMap;

/* static Dict *prims;
 *
 * extern List *prim(char *s, List *list, Binding *binding, int evalflags) {
 * List *(*p)(List *, Binding *, int);
 * p = (List *(*)(List *, Binding *, int)) dictget(prims, s);
 * if (p == NULL)
 * fail("es:prim", "unknown primitive: %s", s);
 * return (*p)(list, binding, evalflags);
 * }
 *
 * PRIM(primitives) {
 * static List *primlist = NULL;
 * if (primlist == NULL) {
 * globalroot(&primlist);
 * dictforall(prims, addtolist, &primlist);
 * primlist = sortlist(primlist);
 * }
 * return primlist;
 * }
 *
 * */


mod ctl;

pub fn initprims() {
    let mut prims: BTreeMap<String, fn(&::list::List) -> ::list::List> = BTreeMap::new();
    ctl::initprims_controlflow(&mut prims);
    /* prims = initprims_io(prims);
     * prims = initprims_etc(prims);
     * prims = initprims_sys(prims);
     * prims = initprims_proc(prims);
     * prims = initprims_access(prims);
     *
     * #define	primdict prims
     * X(primitives);
     * */
}
