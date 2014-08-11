/* prim.c -- primitives and primitive dispatching ($Revision: 1.1.1.1 $) */

/*
static Dict *prims;

extern List *prim(char *s, List *list, Binding *binding, int evalflags) {
	List *(*p)(List *, Binding *, int);
	p = (List *(*)(List *, Binding *, int)) dictget(prims, s);
	if (p == NULL)
		fail("es:prim", "unknown primitive: %s", s);
	return (*p)(list, binding, evalflags);
}

PRIM(primitives) {
	static List *primlist = NULL;
	if (primlist == NULL) {
		globalroot(&primlist);
		dictforall(prims, addtolist, &primlist);
		primlist = sortlist(primlist);
	}
	return primlist;
}

*/

use std::collections::TreeMap;

pub fn initprims() {
    let prims:TreeMap<String, String> = TreeMap::new();
/*
	prims = initprims_controlflow(prims);
	prims = initprims_io(prims);
	prims = initprims_etc(prims);
	prims = initprims_sys(prims);
	prims = initprims_proc(prims);
	prims = initprims_access(prims);

#define	primdict prims
	X(primitives);
*/
}
