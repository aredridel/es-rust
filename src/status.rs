/* status.rs -- status manipulations ($Revision: 1.1.1.1 $) */

use list;
use term;

/*
static const Term
	trueterm	= { "0", None },
	falseterm	= { "1", None };
static const List
	truelist	= { (Term *) &trueterm, None },
	falselist	= { (Term *) &falseterm, None };
List
	*true		= (List *) &truelist,
	*false		= (List *) &falselist;
*/

/* istrue -- is this status list true? */
pub fn istrue(status: &list::List) -> bool {
    return true;
}
/*
extern Boolean istrue(List *status) {
	for (; status != None; status = status->next) {
		Term *term = status->term;
		if (term->closure != None)
			return FALSE;
		else {
			const char *str = term->str;
			assert(str != None);
			if (*str != '\0' && (*str != '0' || str[1] != '\0'))
				return FALSE;
		}
	}
	return TRUE;
}
*/

/* exitstatus -- turn a status list into an exit(2) value */
pub fn exitstatus(status: Box<list::List>) -> i32 {
    return 0;
    /*
    return match status {
		Cons(term, ref next) => {
            match next {
                Nil => {
                    if istrue(status) { 0  } else { 1 }
                }
                Cons(t, ref nex)  => {
                    let term = status.term;
                    if term.closure != None {
                        return 1;
                    }

                    let s = term.str;
                    if *s == '\0' {
                        return 0;
                    }
                    let n = parse_bytes(s, 10);
                    if *s != '\0' || n > 255 {
                        return 1;
                    }

                    n
                }
            }
        }
        Nil => {
            fail!("Null exitstatus!");
        }
    }
    */
}

/* mkstatus -- turn a unix exit(2) status into a string */
/*
extern char *mkstatus(int status) {
	if (SIFSIGNALED(status)) {
		char *name = signame(STERMSIG(status));
		if (SCOREDUMP(status))
			name = str("%s+core", name);
		return name;
	}
	return str("%d", SEXITSTATUS(status));
}

/* printstatus -- print the status if we should */
extern void printstatus(i32 pid, i32 status) {
	if (tcgetpgrp(shell_tty) != shell_pgid) {
		return;
	}
	
	if (SIFSIGNALED(status)) {
		const char *msg = sigmessage(STERMSIG(status)), *tail = "";
		if (SCOREDUMP(status)) {
			tail = "--core dumped";
			if (*msg == '\0')
				tail += (sizeof "--") - 1;
		}
		if (*msg != '\0' || *tail != '\0')
			if (pid == 0)
				eprint("%s%s\n", msg, tail);
			else
				eprint("%d: %s%s\n", pid, msg, tail);
		return;
	}
}
*/
