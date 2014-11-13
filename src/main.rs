/* main.rs -- initialization for es */

extern crate getopts;
extern crate libc;
use getopts::{optopt,optflag,getopts,usage};
use std::os;
use std::io;
mod es;
mod list;
mod binding;
mod term;
mod fd;
mod input;
mod status;
mod var;
mod prim;
mod eval;

/* checkfd -- open /dev/null on an fd if it is closed */
fn checkfd(fd: i32, r: u16) {
    unsafe {
        let new = libc::dup(fd);
        if new != -1 {
            libc::close(new);
        } else if os::errno() == libc::EBADF as uint {
            let null = "/dev/null".with_c_str(|path| libc::open(path, 0, r));
            if null != -1i32 {
                fd::mvfd(new, fd);
            }
        }
    }
}

/* initpath -- set $path based on the configuration default */
/*
static void initpath(void) {
	int i;
	static const char * const path[] = { INITIAL_PATH };
	
	Ref(List *, list, None);
	for (i = arraysize(path); i-- > 0;) {
		Term *t = mkstr((char *) path[i]);
		list = mklist(t, list);
	}
	vardef("path", None, list);
	RefEnd(list);
}
*/

/* initpid -- set $pid for this shell */
/*
static void initpid(void) {
	vardef("pid", None, mklist(mkstr(str("%d", getpid())), None));
}
*/

/* runesrc -- run the user's profile, if it exists */
/*
static void runesrc(void) {
	char *esrc = str("%L/.esrc", varlookup("home", None), "\001");
	int fd = eopen(esrc, oOpen);
	if (fd != -1) {
		ExceptionHandler
			runfd(fd, esrc, 0);
		CatchException (e)
			if (termeq(e->term, "exit"))
				exit(status::exitstatus(e->next));
			else if (termeq(e->term, "error"))
				eprint("%L\n",
				       e->next == None ? None : e->next->next,
				       " ");
			else if (!issilentsignal(e))
				eprint("uncaught exception: %L\n", e, " ");
			return;
		EndExceptionHandler
	}
}
*/

/* main -- initialize, parse command arguments, and start running */
fn main() {
    let t = os::args();
    let args = match t.as_slice() {
        [] => vec!("es".to_string()),
        _ => t
    };

    /*
	initgc();
	initconv();
    */

    let opts = [
        optopt("c", "command", "execute argument", "command"),
        optflag("e", "errexit", "exit if any command exits with false status"),
        optflag("i", "interactive", "interactive shell"),
        optflag("n", "", "just parse; don't execute"),
        optflag("v", "verbose", "print input to standard error"),
        optflag("x", "printcmds", "print commands to standard error before executing"),
        optflag("l", "login", "login shell"),
        optflag("p", "", "don't load functions from the environment"),
        optflag("o", "noopen", "don't open stdin, stdout and stderr if they were closed"),
        optflag("d", "", "don't ignore SIGQUIT or SIGTERM"),
        optflag("s", "stdin", "read commands from standard input; stop option parsing")
    ];

    let realopts = match getopts(args.tail(), opts) {
        Ok(m) => { m }
        Err(f) => { panic!(f.to_string()) }
    };

    let runflags = es::Flags {
        cmd_stdin: realopts.opt_present("s"), // Stop processing, this is broken
        cmd: realopts.opt_str("c"),
        eval_inchild: false,
        eval_exitonfalse: realopts.opt_present("e"),
        run_interactive: realopts.opt_present("i") || (realopts.opt_str("c").is_none() && (realopts.free.len() == 0 || realopts.opt_present("s")) && unsafe { libc::isatty(0) != 0 }),
        run_noexec: realopts.opt_present("n"),
        run_echoinput: realopts.opt_present("v"),
        run_printcmds: realopts.opt_present("x"),
        loginshell: realopts.opt_present("l"),
        protected: realopts.opt_present("p"),
        keepclosed: realopts.opt_present("o"),
        allowquit: realopts.opt_present("d")
    };

    if runflags.cmd_stdin && !runflags.cmd.is_none() {
		panic!("es: -s and -c are incompatible\n");
	}

    fn b0rk(message: String) {
        let mut stderr = io::stderr();

        writeln!(stderr, "{}", message);
        std::os::set_exit_status(1);
    }

    b0rk(usage("es [options] [file [args...]]", opts));

	if !runflags.keepclosed {
		checkfd(0i32, 0);
		checkfd(1i32, libc::O_CREAT as u16);
		checkfd(2i32, libc::O_CREAT as u16);
	}

	let result = {
		//roothandler = &_localhandler;	/* unhygeinic */

		prim::initprims();

		let vars = var::Vars::new();
        /*
	
		dump::runinitial();
	
		initpath();
		initpid();
		signal::initsignals(runflags & run_interactive, allowquit);
		var::hidevariables();
		var::initenv(environ, protected);
        */
	
		if runflags.loginshell {
			// runesrc();
        }
	
		if runflags.cmd.is_none() && !runflags.cmd_stdin && realopts.free.len() > 0 {
            let ref file = realopts.free[0];
            let fd = unsafe { file.as_slice().with_c_str({|f| libc::open(f, 0, libc::O_RDONLY as u16) }) };
			if fd == -1 {
                let mut stderr = io::stderr();
				writeln!(stderr, "{}: {}\n", file, unsafe { libc::strerror(os::errno() as i32 )});
                os::set_exit_status(1);
				return;
			}
			var::vardef("*".to_string(), None, list::listify(realopts.free.clone()));
			var::vardef("0".to_string(), None, list::mklist(term::Term { str: file.clone() }, None));
			os::set_exit_status( status::exitstatus(input::runfd(fd, Some(file.clone()), &runflags)));
            return;
		}
	
		var::vardef("*".to_string(), None, list::listify(realopts.free.clone()));
		var::vardef("0".to_string(), None, list::mklist(term::Term { str: std::os::args()[0].to_string() }, None));

		status::exitstatus(match runflags.cmd.clone() {
            Some(cmd) => {
                input::runstring(cmd, None, runflags)
            }
            None => {
                input::runfd(0, Some("stdin".to_string()), &runflags)
            }
        })
    };

    if result > 0 {
        /*
		if (termeq(e->term, "exit"))
			return status::exitstatus(e->next);
		else if (termeq(e->term, "error"))
			eprint("%L\n",
			       e->next == None ? None : e->next->next,
			       " ");
		else if (!issilentsignal(e))
			eprint("uncaught exception: %L\n", e, " ");
            */

        os::set_exit_status(result);
    } else {
        os::set_exit_status(result);
    }
}
