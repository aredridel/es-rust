/* main.rs -- initialization for es ($Revision: 1.3 $) */

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

/* checkfd -- open /dev/null on an fd if it is closed */
fn checkfd(fd: i32, r: i32) {
    unsafe {
        let new = libc::dup(fd);
        if new != -1 {
            libc::close(new);
        } else if os::errno() == libc::EBADF as int{
            let null = "/dev/null".with_c_str(|path| libc::open(path, 0, r as libc::c_int));
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
    let mut input: Box<input::Input> = box input::Input {
        prev: None,
        name: None,
        /*
        buf: &0,
        bufend: &0,
        bufbegin: &0,
        rbuf: &0,
        buflen: 0,
        unget: [0, 0],
        ungot: 0,
        */
        lineno: 0,
        fd: 0,
        runflags: es::Flags {
            run_interactive: true,
            cmd_stdin: false,
            cmd: Some("".to_str()),
            eval_exitonfalse: false,
            eval_inchild: false,
            run_noexec: false,
            run_echoinput: false,
            run_printcmds: false,
            loginshell: false,
            protected: false,
            keepclosed: false,
            allowquit: false
        }
    };
    let mut args: Vec<String> = os::args();

	let runflags = 0i;		/* -[einvxL] */
	let protected = false;	/* -p */
	let allowquit = false;	/* -d */
	let cmd_stdin = false;		/* -s */
    let mut loginshell = false;	/* -l or $0[0] == '-' */
	let keepclosed = false;		/* -o */

    /*
	initgc();
	initconv();
    */

	if args.len() == 0 {
        args = vec!("es".to_string());
	}
	if args.get(0).as_slice() == "-" {
		loginshell = true;
    }

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
        Err(f) => { fail!(f.to_str()) }
    };

    let mut runflags = es::Flags {
        cmd_stdin: realopts.opt_present("s"), // Stop processing, this is broken
        cmd: realopts.opt_str("c"),
        eval_inchild: false,
        eval_exitonfalse: realopts.opt_present("e"),
        run_interactive: realopts.opt_present("i"),
        run_noexec: realopts.opt_present("n"),
        run_echoinput: realopts.opt_present("v"),
        run_printcmds: realopts.opt_present("x"),
        loginshell: realopts.opt_present("l"),
        protected: realopts.opt_present("p"),
        keepclosed: realopts.opt_present("o"),
        allowquit: realopts.opt_present("d")
    };

    if runflags.cmd_stdin && !runflags.cmd.is_none() {
		fail!("es: -s and -c are incompatible\n");
	}

    fn b0rk(message: String) {
        let mut stderr = io::stderr();

        writeln!(stderr, "{}", message);
        std::os::set_exit_status(1);
    }

    b0rk(usage("es [options] [file [args...]]", opts));

	if !keepclosed {
		checkfd(0, 0);
		checkfd(1, libc::O_CREAT);
		checkfd(2, libc::O_CREAT);
	}

	if runflags.cmd.is_none() && (realopts.free.len() == 0 || cmd_stdin) && !runflags.run_interactive && unsafe { libc::isatty(0) != 0 } {
		runflags.run_interactive = true;
    }

	let result = {
		//roothandler = &_localhandler;	/* unhygeinic */

        /*
		input::initinput();
		prim::initprims();
		var::initvars();
	
		dump::runinitial();
	
		initpath();
		initpid();
		signal::initsignals(runflags & run_interactive, allowquit);
		var::hidevariables();
		var::initenv(environ, protected);
        */
	
		if loginshell {
			// runesrc();
        }
	
		if runflags.cmd.is_none() && !cmd_stdin && realopts.free.len() > 0 {
            let file = realopts.free.get(0);
            let fd = unsafe { file.as_slice().with_c_str({|f| libc::open(f, 0, libc::O_RDONLY) }) };
			if (fd == -1) {
                let mut stderr = io::stderr();
				writeln!(stderr, "{}: {}\n", file, unsafe { libc::strerror(os::errno() as i32 )});
                os::set_exit_status(1);
				return;
			}
			var::vardef("*".to_string(), None, list::listify(realopts.free.clone()));
			var::vardef("0".to_string(), None, list::mklist(term::Term { str: file.clone() }, None));
			os::set_exit_status( status::exitstatus(input::runfd(fd, Some(file.clone()), &mut runflags)));
            return;
		}
	
		var::vardef("*".to_string(), None, list::listify(realopts.free.clone()));
		var::vardef("0".to_string(), None, list::mklist(term::Term { str: std::os::args().get(0).to_string() }, None));

		status::exitstatus(match runflags.cmd.clone() {
            Some(cmd) => {
                input::runstring(cmd, None, runflags)
            }
            None => {
                input::runfd(0, Some("stdin".to_string()), &mut runflags)
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
