/* main.rs -- initialization for es */

extern crate getopts;
extern crate libc;
use getopts::Options;
use std::io;
use std::io::Write;
extern crate errno;
use std::ffi::CString;
use errno::errno;
mod es;
use es::Es;
mod list;
use list::List;
mod binding;
mod term;
mod fd;
mod input;
mod status;
use status::exitstatus;
mod var;
mod prim;
mod eval;

/* initpath -- set $path based on the configuration default */
/* static void initpath(void) {
 * int i;
 * static const char * const path[] = { INITIAL_PATH };
 *
 * Ref(List *, list, None);
 * for (i = arraysize(path); i-- > 0;) {
 * Term *t = mkstr((char *) path[i]);
 * list = mklist(t, list);
 * }
 * vardef("path", None, list);
 * RefEnd(list);
 * }
 * */

/* initpid -- set $pid for this shell */
/* static void initpid(void) {
 * vardef("pid", None, mklist(mkstr(str("%d", getpid())), None));
 * }
 * */

/* runesrc -- run the user's profile, if it exists */
/* static void runesrc(void) {
 * char *esrc = str("%L/.esrc", varlookup("home", None), "\001");
 * int fd = eopen(esrc, oOpen);
 * if (fd != -1) {
 * ExceptionHandler
 * runfd(fd, esrc, 0);
 * CatchException (e)
 * if (termeq(e->term, "exit"))
 * exit(exitstatus(e->next));
 * else if (termeq(e->term, "error"))
 * eprint("%L\n",
 * e->next == None ? None : e->next->next,
 * " ");
 * else if (!issilentsignal(e))
 * eprint("uncaught exception: %L\n", e, " ");
 * return;
 * EndExceptionHandler
 * }
 * }
 * */

/* main -- initialize, parse command arguments, and start running */
#[allow(unused_variables)]
fn main() {
    let args: Vec<String> = std::env::args().collect();

    /* initgc();
     * initconv();
     * */

    let mut opts = Options::new();

    opts.optopt("c", "command", "execute argument", "command");
    opts.optflag("e",
                 "errexit",
                 "exit if any command exits with false status");
    opts.optflag("i", "interactive", "interactive shell");
    opts.optflag("n", "", "just parse; don't execute");
    opts.optflag("v", "verbose", "print input to standard error");
    opts.optflag("x",
                 "printcmds",
                 "print commands to standard error before executing");
    opts.optflag("l", "login", "login shell");
    opts.optflag("p", "", "don't load functions from the environment");
    opts.optflag("o",
                 "noopen",
                 "don't open stdin, stdout and stderr if they were closed");
    opts.optflag("d", "", "don't ignore SIGQUIT or SIGTERM");
    opts.optflag("s",
                 "stdin",
                 "read commands from standard input; stop option parsing");

    let realopts = match opts.parse(&args[1..]) {
        Ok(m) => m,
        Err(f) => {
            b0rk(opts.usage("es [options] [file [args...]]"));
            std::process::exit(1);
        }
    };

    let runflags = es::Flags {
        cmd_stdin: realopts.opt_present("s"), // Stop processing, this is broken
        cmd: realopts.opt_str("c"),
        eval_inchild: false,
        eval_exitonfalse: realopts.opt_present("e"),
        run_interactive: realopts.opt_present("i") ||
                         (realopts.opt_str("c").is_none() &&
                          (realopts.free.len() == 0 || realopts.opt_present("s")) &&
                          unsafe { libc::isatty(0) != 0 }),
        run_noexec: realopts.opt_present("n"),
        run_echoinput: realopts.opt_present("v"),
        run_printcmds: realopts.opt_present("x"),
        loginshell: realopts.opt_present("l"),
        protected: realopts.opt_present("p"),
        keepclosed: realopts.opt_present("o"),
        allowquit: realopts.opt_present("d"),
    };

    fn b0rk(message: String) -> i32 {
        let mut stderr = io::stderr();

        writeln!(stderr, "{}", message).unwrap();
        return 1;
    }

    let result = {
        match Es::new(runflags, var::Vars::new()) {
            Ok(es) => {
                // roothandler = &_localhandler;	/* unhygeinic */

                prim::initprims();

                if es.flags.cmd.is_none() && !es.flags.cmd_stdin && realopts.free.len() > 0 {
                    let ref file = realopts.free[0];
                    let fd = unsafe {
                        libc::open(CString::new(file.to_string().into_bytes()).unwrap().into_raw(),
                                   0,
                                   libc::O_RDONLY)
                    };
                    if fd == -1 {
                        let mut stderr = io::stderr();
                        writeln!(stderr, "{}: {}\n", file, errno()).unwrap();
                        std::process::exit(1);
                    }
                    var::vardef("*".to_string(), None, list::listify(realopts.free.clone()));
                    var::vardef("0".to_string(),
                                None,
                                List::cell(term::Term { str: file.clone() }));
                    std::process::exit(exitstatus(es.runfd(fd, Some(file.clone()), &es.flags)));
                }

                var::vardef("*".to_string(), None, list::listify(realopts.free.clone()));
                var::vardef("0".to_string(),
                            None,
                            List::cell(term::Term { str: std::env::args().nth(0).unwrap() }));

                exitstatus(match es.flags.cmd.clone() {
                    Some(cmd) => input::runstring(cmd, None, es.flags),
                    None => es.runfd(0, Some("stdin".to_string()), &es.flags),
                })
            }
            Err(e) => b0rk(e.to_string()),

        }
    };

    if result > 0 {
        /* if (termeq(e->term, "exit"))
         * return exitstatus(e->next);
         * else if (termeq(e->term, "error"))
         * eprint("%L\n",
         * e->next == None ? None : e->next->next,
         * " ");
         * else if (!issilentsignal(e))
         * eprint("uncaught exception: %L\n", e, " ");
         * */

        std::process::exit(result);
    } else {
        std::process::exit(result);
    }
}
