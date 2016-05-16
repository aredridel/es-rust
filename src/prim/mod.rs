/* prim.c -- primitives and primitive dispatching ($Revision: 1.1.1.1 $) */

use std::collections::HashMap;
use list::List;
use es::Es;
use var::Vars;
use eval::eval;

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

pub type Prims = HashMap<String, fn(&Vars, &List) -> List>;

pub fn initprims() -> Prims {
    let mut prims: Prims = HashMap::new();
    ctl::initprims_controlflow(&mut prims);
    prims.insert("debug".to_string(), debug);
    /* prims = initprims_io(prims);
     * prims = initprims_etc(prims);
     * prims = initprims_sys(prims);
     * prims = initprims_proc(prims);
     * prims = initprims_access(prims);
     *
     * #define	primdict prims
     * X(primitives);
     * */
    prims
}

impl Es {
    #[allow(unused_variables)]
    pub fn batchloop(&self, args: &List) -> List {
        let dispatch = self.vars.lookup("fn-%dispatch");
        let parser = self.vars.lookup("%fn-parse");
        let cmdtail = match parser {
            Some(p) => eval(p, &self.vars, &self.prims, &self.flags),
            None => self.parse("$&debug Hello, World"),
        };

        let cmd = match cmdtail {
            Err(e) => List::Nil,
            Ok(l) => {
                match dispatch {
                    None => l,
                    Some(d) => d.append(&l),
                }
            }
        };

        match eval(cmd, &self.vars, &self.prims, &self.flags) {
            Ok(l) => l,
            Err(e) => List::Nil,
        }
    }
}

#[allow(unused_variables)]
pub fn debug(vars: &Vars, args: &List) -> List {
    println!("{:?}", args);
    List::Nil
}
