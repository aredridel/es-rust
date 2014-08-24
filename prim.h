/* prim.h -- definitions for es primitives ($Revision: 1.1.1.1 $) */

#define	PRIM(name)	static List *CONCAT(prim_,name)( \
				List *list, Binding *binding, int evalflags \
			)
#define	X(name)		(primdict = dictput( \
				primdict, \
				STRING(name), \
				(void *) CONCAT(prim_,name) \
			))
