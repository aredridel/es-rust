/* main.rs -- initialization for es ($Revision: 1.3 $) */

extern crate getopts;
extern crate libc;
use getopts::{optopt,optflag,getopts,usage};
use std::os;
use std::io;
mod fd;
mod status;

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
	
	Ref(List *, list, NULL);
	for (i = arraysize(path); i-- > 0;) {
		Term *t = mkstr((char *) path[i]);
		list = mklist(t, list);
	}
	vardef("path", NULL, list);
	RefEnd(list);
}
*/

/* initpid -- set $pid for this shell */
/*
static void initpid(void) {
	vardef("pid", NULL, mklist(mkstr(str("%d", getpid())), NULL));
}
*/

/* runesrc -- run the user's profile, if it exists */
/*
static void runesrc(void) {
	char *esrc = str("%L/.esrc", varlookup("home", NULL), "\001");
	int fd = eopen(esrc, oOpen);
	if (fd != -1) {
		ExceptionHandler
			runfd(fd, esrc, 0);
		CatchException (e)
			if (termeq(e->term, "exit"))
				exit(status::exitstatus(e->next));
			else if (termeq(e->term, "error"))
				eprint("%L\n",
				       e->next == NULL ? NULL : e->next->next,
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
    let mut args: Vec<String> = os::args();

	let runflags = 0i;		/* -[einvxL] */
	let protected = false;	/* -p */
	let allowquit = false;	/* -d */
	let cmd_stdin = false;		/* -s */
    let mut loginshell = false;	/* -l or $0[0] == '-' */
	let keepclosed = false;		/* -o */
    let cmd = ""; /* -c */

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

    /*
	while ((c = getopt(argc, argv, "eilxvnpodsc:?GIL")) != EOF)
		switch (c) {
		case 'l':	loginshell = TRUE;		break;
		case 'p':	protected = TRUE;		break;
		case 'o':	keepclosed = TRUE;		break;
		case 'd':	allowquit = TRUE;		break;
		case 's':	cmd_stdin = TRUE;			goto getopt_done;
#if GCVERBOSE
		case 'G':	gcverbose = TRUE;		break;
#endif
#if GCINFO
		case 'I':	gcinfo = TRUE;			break;
#endif
		default:
			usage();
		}

getopt_done:

    */

    let cmd_stdin = realopts.opt_present("s"); // Stop processing, this is broken
    let cmd = realopts.opt_str("c");
    let eval_exitonfalse = realopts.opt_present("e");
    let mut run_interactive = realopts.opt_present("i");
    let run_noexec = realopts.opt_present("n");
    let run_echoinput = realopts.opt_present("v");
    let run_printcmds = realopts.opt_present("x");
    let loginshell = realopts.opt_present("l");
    let protected = realopts.opt_present("p");
    let keepclosed = realopts.opt_present("o");
    let allowquit = realopts.opt_present("d");

    if cmd_stdin && !cmd.is_none() {
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


	if cmd.is_none() && (realopts.free.len() == 0 || cmd_stdin) && !run_interactive && unsafe { libc::isatty(0) != 0 } {
		run_interactive = true;
    }

	let result = try!({
		roothandler = &_localhandler;	/* unhygeinic */

		initinput();
		initprims();
		initvars();
	
		runinitial();
	
		initpath();
		initpid();
		initsignals(runflags & run_interactive, allowquit);
		hidevariables();
		initenv(environ, protected);
	
		if loginshell {
			runesrc();
        }
	
		if cmd.is_none() && !cmd_stdin && realopts.free.len() > 0 {
            let file = realopts.free[0];
			if ((fd = eopen(file, oOpen)) == -1) {
				eprint("%s: %s\n", file, esstrerror(errno));
				return 1;
			}
			vardef("*", NULL, listify(ac - optind, av + optind));
			vardef("0", NULL, mklist(mkstr(file), NULL));
			return status::exitstatus(runfd(fd, file, runflags));
		}
	
		vardef("*", NULL, listify(ac - optind, av + optind));
		vardef("0", NULL, mklist(mkstr(av[0]), NULL));
		return status::exitstatus(if !cmd.is_none() {
			runstring(cmd, NULL, runflags)
        } else {
            runfd(0, "stdin", runflags)
        });

    });

    if result.is_err() {
        /*
		if (termeq(e->term, "exit"))
			return status::exitstatus(e->next);
		else if (termeq(e->term, "error"))
			eprint("%L\n",
			       e->next == NULL ? NULL : e->next->next,
			       " ");
		else if (!issilentsignal(e))
			eprint("uncaught exception: %L\n", e, " ");
            */
		return 1;
    }
}
