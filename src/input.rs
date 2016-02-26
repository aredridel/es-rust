// read input from files or strings

extern crate libc;
use es::{Es, Flags};
use var::{Binding,VarLookup};
use term::Term;
use list::List;

#[allow(dead_code)]
static BUFSIZE: i32 = 1024;

pub struct Input {
    pub prev: Option<Box<Input>>,
    pub name: Option<String>,
    //     buf: *mut libc::c_char,
    //     bufend: *mut libc::c_char,
    //     bufbegin: *mut libc::c_char,
    //     rbuf: *mut libc::c_char,
    //     buflen: libc::size_t,
    //     unget: [int, ..2],
    //     ungot: int,
    pub lineno: i32,
    pub fd: i32,
    pub runflags: Flags,
}

impl Input {
    /// fill input buffer by reading from a file descriptor
    #[allow(dead_code)]
    fn fdfill() {
        unimplemented!();
        //         int nread;
        //         static const char *lastinbuf = None;
        //         Boolean dolog;
        //         HistEvent ev;
        //         int editing;
        //         memzero(&ev, sizeof(HistEvent));
        //         assert(in->buf == in->bufend);
        //         assert(in->fd >= 0);
        //
        //         el_get(el, EL_EDITMODE, &editing);
        //         if (in->runflags & run_interactive && in->fd == 0 && editing) {
        //             const char *rlinebuf = callreadline(prompt, &nread);
        //             dolog = FALSE;
        //             if (rlinebuf == None)
        //                 nread = 0;
        //             else {
        //                 if (in->buflen < nread) {
        //                     while (in->buflen < nread)
        //                         in->buflen *= 2;
        //                     efree(in->bufbegin);
        //                     in->bufbegin = erealloc(in->bufbegin, in->buflen);
        //                 }
        //                 memcpy(in->bufbegin, rlinebuf, nread - 1);
        //                 in->bufbegin[nread - 1] = '\n';
        //                 history(hist, &ev, prompt == prompt2 ? H_ADD : H_ENTER, rlinebuf);
        //                 lastinbuf = rlinebuf;
        //             }
        //         } else
        //         do {
        //             nread = eread(in->fd, (char *) in->bufbegin, in->buflen);
        //             SIGCHK();
        //         } while (nread == -1 && errno == EINTR);
        //
        //         if (nread <= 0) {
        //             close(in->fd);
        //             in->fd = -1;
        //             in->fill = eoffill;
        //             in->runflags &= ~run_interactive;
        //             if (nread == -1)
        //                 fail("$&parse", "%s: %s", in->name == None ?
        //                       "es" :
        //                       in->name, esstrerror(errno));
        //             return EOF;
        //         }
        //
        //         if (in->runflags & run_interactive) {
        //             history(hist, &ev, H_SAVE, histfile);
        //         }
        //
        //         in->buf = in->bufbegin;
        //         in->bufend = &in->buf[nread];
        //         return *in->buf++;
    }

    /// cleanup after running from a file descriptor
    #[allow(dead_code)]
    fn fdcleanup() {
        unimplemented!();
        //         /*
        //         unregisterfd(&in->fd);
        //         if (in->fd != -1)
        //             close(in->fd);
        //         efree(in->bufbegin);
        //         */
    }
}

impl Input {
    #[allow(dead_code)]
    fn get(&self) -> i8 {
        if self.runflags.run_echoinput {
            unimplemented!();
            //             /*
            //             if (in->fill == ungetfill)
            //                 return get(in);
            //             else {
            //                 int c = get(in);
            //                 if (c != EOF) {
            //                     char buf = c;
            //                     ewrite(2, &buf, 1);
            //                 }
            //                 return c;
            //             }
            //             */
        } else {
            unimplemented!();
            //             /*
            //             int c;
            //             while ((c = (in->buf < in->bufend ?
            //                  *in->buf++ :
            //                  (*in->fill)(in))) == '\0')
            //                 warn("null character ignored");
            //             return c;
            //             */
        }
    }

    #[allow(dead_code)]
    fn fill(&self) -> i32 {
        0
    }

    #[allow(dead_code)]
    fn rfill(&self) -> i32 {
        0
    }

    #[allow(dead_code)]
    fn cleanup(&self) {}
}

// /*
//
// Input *input;
// char *prompt, *prompt2;
//
// Boolean disablehistory = FALSE;
// Boolean resetterminal = FALSE;
// static char *histfile = None;
//
// static History *hist;
// static EditLine *el;
//
// */
// /*
//
// /*
//  * errors and warnings
//  */
//
// /* locate -- identify where an error came from */
// static char *locate(Input *in, char *s) {
// 	return (in->runflags & run_interactive)
// 		? s
// 		: str("%s:%d: %s", in->name, in->lineno, s);
// }
//
// static char *error = None;
//
// /* yyerror -- yacc error entry point */
// extern void yyerror(char *s) {
// #if sgi
// 	/* this is so that trip.es works */
// 	if (streq(s, "Syntax error"))
// 		s = "syntax error";
// #endif
// 	if (error == None)	/* first error is generally the most informative */
// 		error = locate(input, s);
// }
//
// /* warn -- print a warning */
// static void warn(char *s) {
// 	eprint("warning: %s\n", locate(input, s));
// }
//
//
// /*
//  * history
//  */
//
// /* sethistory -- change the file for the history log */
// extern void sethistory(char *file) {
// 	HistEvent ev;
// 	history(hist, &ev, H_SAVE, histfile);
// 	histfile = file;
// }
//
//
// /*
//  * unget -- character pushback
//  */
//
// /* ungetfill -- input->fill routine for ungotten characters */
// static i32 ungetfill(Input *in) {
// 	i32 c;
// 	assert(in->ungot > 0);
// 	c = in->unget[--in->ungot];
// 	if (in->ungot == 0) {
// 		assert(in->rfill != None);
// 		in->fill = in->rfill;
// 		in->rfill = None;
// 		assert(in->rbuf != None);
// 		in->buf = in->rbuf;
// 		in->rbuf = None;
// 	}
// 	return c;
// }
//
// /* unget -- push back one character */
// extern void unget(Input *in, i32 c) {
// 	if (in->ungot > 0) {
// 		assert(in->ungot < MAXUNGET);
// 		in->unget[in->ungot++] = c;
// 	} else if (in->bufbegin < in->buf && in->buf[-1] == c && (input->runflags & run_echoinput) == 0)
// 		--in->buf;
// 	else {
// 		assert(in->rfill == None);
// 		in->rfill = in->fill;
// 		in->fill = ungetfill;
// 		assert(in->rbuf == None);
// 		in->rbuf = in->buf;
// 		in->buf = in->bufend;
// 		assert(in->ungot == 0);
// 		in->ungot = 1;
// 		in->unget[0] = c;
// 	}
// }
//
//
// /*
//  * getting characters
//  */
//
// /* get -- get a character, filter out nulls */
// static i32 get(Input *in) {
// }
//
// /* getverbose -- get a character, print it to standard error */
//
// /* eoffill -- report eof when called to fill input buffer */
// static i32 eoffill(Input *in) {
// 	assert(in->fd == -1);
// 	return EOF;
// }
//
// /* callreadline -- readline wrapper */
// static const char *callreadline(char *prompt, i32 *n) {
// 	const char *r;
// 	if (resetterminal) {
// 		resetterminal = FALSE;
// 	}
// 	interrupted = FALSE;
// 	if (!setjmp(slowlabel)) {
// 		slow = TRUE;
// 		r = interrupted ? None : el_gets(el, n);
// 	} else
// 		r = None;
// 	slow = FALSE;
// 	if (r == None)
// 		errno = EINTR;
// 	SIGCHK();
// 	return r;
// }
//
// static char *getprompt (EditLine *el) {
//     return prompt;
// }
//
// /* getenv -- fake version of getenv for readline (or other libraries) */
// static char *esgetenv(const char *name) {
// 	List *value = varlookup(name, None);
// 	if (value == None)
// 		return None;
// 	else {
// 		char *export;
// 		static Dict *envdict;
// 		static Boolean initialized = FALSE;
// 		Ref(char *, string, None);
//
// 		gcdisable();
// 		if (!initialized) {
// 			initialized = TRUE;
// 			envdict = mkdict();
// 			globalroot(&envdict);
// 		}
//
// 		string = dictget(envdict, name);
// 		if (string != None)
// 			efree(string);
//
// 		export = str("%W", value);
// 		string = ealloc(strlen(export) + 1);
// 		strcpy(string, export);
// 		envdict = dictput(envdict, (char *) name, string);
//
// 		gcenable();
// 		RefReturn(string);
// 	}
// }
//
// static i32 fdfill(Input *in) {
// }
//
//
// /*
//  * the input loop
//  */
//
// /* parse -- call yyparse(), but disable garbage collection and catch errors */
// extern Tree *parse(char *pr1, char *pr2) {
// 	i32 result;
// 	assert(error == None);
//
// 	inityy();
// 	emptyherequeue();
//
// 	if (ISEOF(input))
// 		throw(mklist("eof", None));
//
// 	prompt = (pr1 == None) ? "" : pr1;
// 	prompt2 = pr2;
//
// 	gcreserve(300 * sizeof (Tree));
// 	gcdisable();
// 	result = yyparse();
// 	gcenable();
//
// 	if (result || error != None) {
// 		char *e;
// 		assert(error != None);
// 		e = error;
// 		error = None;
// 		fail("$&parse", "%s", e);
// 	}
// 	return parsetree;
// }
//
// /* resetparser -- clear parser errors in the signal handler */
// extern void resetparser(void) {
// 	error = None;
// }

impl Es {
    /// run from an input source
    #[allow(unused_variables)]
    pub fn runinput(&self, mut inp: Box<Input>, runflags: &Flags) -> Box<List<Term>> {

        let dispatcher = ["fn-%eval-noprint",
                          "fn-%eval-print",
                          "fn-%noeval-noprint",
                          "fn-%noeval-print"];

        inp.runflags = runflags.clone();
        inp.runflags.eval_inchild = true;
        // inp.prev = Some(input);

        match {
            let dispatchfn = dispatcher[if runflags.run_printcmds { 1 } else { 0 } +
                                        if runflags.run_noexec { 2 } else { 0 }];
            let mut dispatch = self.vars.varlookup(dispatchfn.to_string());

            match *dispatch {
                List::Nil => panic!("no dispatch found in '{}'", dispatchfn),
                List::Cons(ref term, ref next) => {}
            }

            if runflags.eval_exitonfalse {
                dispatch = List::cons(Term { str: "%exit-on-false".to_string() }, *dispatch);
            }

            let push = Binding::varpush("fn-%dispatch".to_string(), dispatch);

            let repl = self.vars.varlookup(if runflags.run_interactive {
                                              "fn-%interactive-loop"
                                          } else {
                                              "fn-%batch-loop"
                                          }
                                          .to_string());
            let result = match *repl {
                List::Nil => {
                    Err("")
                    // prim("batchloop", None, None, runflags)
                }
                List::Cons(ref term, ref next) => {
                    Err("")
                    // Ok(List::Nil) as Result<List, String>
                    // eval(repl, None, runflags)
                }
            };

            Binding::varpop(push);

            result

        } {
            Err(e) => {
                // input.cleanup();
                // input = input.prev;
                panic!(e);
            }
            Ok(res) => {
                // input = inp.prev;
                inp.cleanup();
                return Box::new(res);
            }
        }
    }

    /* runfd -- run commands from a file descriptor */
    pub fn runfd(&self, fd: i32, name: Option<String>, runflags: &Flags) -> Box<List<Term>> {
        let inp = Input {
            prev: None,
            /* buf: &0,
             * rbuf: &0,
             * buflen: BUFSIZE as u64,
             * bufbegin: &0,
             * bufend: &0,
             * unget: [0, 0],
             * ungot: 0,
             * */
            runflags: Flags {
                run_interactive: true,
                cmd_stdin: false,
                cmd: Some("".to_string()),
                eval_exitonfalse: false,
                eval_inchild: false,
                run_noexec: false,
                run_echoinput: false,
                run_printcmds: false,
                loginshell: false,
                protected: false,
                keepclosed: false,
                allowquit: false,
            },
            lineno: 1,
            fd: fd,
            name: match name {
                None => Some(format!("fd {}", fd).to_string()),
                Some(n) => Some(n),
            },
        };

        // registerfd(&inp.fd, TRUE);
        // in.bufbegin = in.buf = ealloc(in.buflen);

        // RefAdd(inp.name);
        let result = self.runinput(Box::new(inp), runflags);
        // RefRemove(inp.name);

        return result;
    }
}

// /* stringcleanup -- cleanup after running from a string */
// static void stringcleanup(Input *in) {
// 	efree(in->bufbegin);
// }
//
// /* stringfill -- placeholder than turns into EOF right away */
// static i32 stringfill(Input *in) {
// 	in->fill = eoffill;
// 	return EOF;
// }

/// run commands from a string
#[allow(unused_variables)]
pub fn runstring(s: String, name: Option<String>, flags: Flags) -> Box<List<Term>> {
    Box::new(List::Nil)
}

// extern List *runstring(const char *str, const char *name, int flags) {
// 	Input in;
// 	List *result;
// 	unsigned char *buf;
//
// 	assert(str != None);
//
// 	memzero(&in, sizeof (Input));
// 	in.fd = -1;
// 	in.lineno = 1;
// 	in.name = (name == None) ? str : name;
// 	in.fill = stringfill;
// 	in.buflen = strlen(str);
// 	buf = ealloc(in.buflen + 1);
// 	memcpy(buf, str, in.buflen);
// 	in.bufbegin = in.buf = buf;
// 	in.bufend = in.buf + in.buflen;
// 	in.cleanup = stringcleanup;
//
// 	RefAdd(in.name);
// 	result = runinput(&in, flags);
// 	RefRemove(in.name);
// 	return result;
// }
//
// /* parseinput -- turn an input source into a tree */
// extern Tree *parseinput(Input *in) {
// 	Tree * volatile result;
//
// 	in->prev = input;
// 	in->runflags = 0;
// 	in->get = get;
// 	input = in;
//
// 	ExceptionHandler
// 		result = parse(None, None);
// 		if (get(in) != EOF)
// 			fail("$&parse", "more than one value in term");
// 	CatchException (e)
// 		(*input->cleanup)(input);
// 		input = input->prev;
// 		throw(e);
// 	EndExceptionHandler
//
// 	input = in->prev;
// 	(*in->cleanup)(in);
// 	return result;
// }

// /// turn a string into a tree; must be exactly one tree
// extern Tree *parsestring(const char *str) {
// 	Input in;
// 	Tree *result;
// 	unsigned char *buf;
//
// 	assert(str != None);
//
// 	/* TODO: abstract out common code with runstring */
//
// 	memzero(&in, sizeof (Input));
// 	in.fd = -1;
// 	in.lineno = 1;
// 	in.name = str;
// 	in.fill = stringfill;
// 	in.buflen = strlen(str);
// 	buf = ealloc(in.buflen + 1);
// 	memcpy(buf, str, in.buflen);
// 	in.bufbegin = in.buf = buf;
// 	in.bufend = in.buf + in.buflen;
// 	in.cleanup = stringcleanup;
//
// 	RefAdd(in.name);
// 	result = parseinput(&in);
// 	RefRemove(in.name);
// 	return result;
// }

// /// is the innermost input source interactive?
// extern Boolean isinteractive(void) {
// 	return input == None ? FALSE : ((input->runflags & run_interactive) != 0);
// }
