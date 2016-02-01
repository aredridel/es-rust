extern crate libc;
extern crate errno;
use var::Vars;
use std::result::Result;
use std::ffi::CString;
use errno::errno;
use libc::c_int;
use fd;

pub struct Flags {
    pub cmd_stdin: bool,
    pub cmd: Option<String>,
    pub eval_exitonfalse: bool,
    pub eval_inchild: bool,
    pub run_interactive: bool,
    pub run_noexec: bool,
    pub run_echoinput: bool,
    pub run_printcmds: bool,
    pub loginshell: bool,
    pub protected: bool,
    pub keepclosed: bool,
    pub allowquit: bool,
}


impl Clone for Flags {
    fn clone(&self) -> Flags {
        Flags {
            cmd_stdin: self.cmd_stdin.clone(),
            cmd: self.cmd.clone(),
            eval_exitonfalse: self.eval_exitonfalse.clone(),
            eval_inchild: self.eval_inchild.clone(),
            run_interactive: self.run_interactive.clone(),
            run_noexec: self.run_noexec.clone(),
            run_echoinput: self.run_echoinput.clone(),
            run_printcmds: self.run_printcmds.clone(),
            loginshell: self.loginshell.clone(),
            protected: self.protected.clone(),
            keepclosed: self.keepclosed.clone(),
            allowquit: self.allowquit.clone(),
        }
    }
}

#[allow(dead_code)]
pub enum Tree {
    Word(u32),
    Qword(u64),
    Prim(String),
    Call(Box<Tree>),
    Thunk(Box<Tree>),
    Var(Box<Tree>),
    Assign(Box<Tree>, Box<Tree>),
    Concat(Box<Tree>, Box<Tree>),
    Closure(Box<Tree>, Box<Tree>),
    For(Box<Tree>, Box<Tree>),
    Lambda(Box<Tree>, Box<Tree>),
    Varsub(Box<Tree>, Box<Tree>),
    Match(Box<Tree>, Box<Tree>),
    Extract(Box<Tree>, Box<Tree>),
    Redir(Box<Tree>, Box<Tree>),
    Pipe(Box<Tree>, Box<Tree>),
}

#[allow(dead_code)]
pub struct Es {
    #[allow(dead_code)]
    pub flags: Flags,
    vars: Vars,
}

impl Es {
    pub fn new(f: Flags, vars: Vars) -> Result<Es, &'static str> {
        if f.cmd_stdin && !f.cmd.is_none() {
            return Err("es: -s and -c are incompatible\n");
        }

        if !f.keepclosed {
            checkfd(0i32, 0);
            checkfd(1i32, libc::O_CREAT as u16);
            checkfd(2i32, libc::O_CREAT as u16);
        }


        let es = Es {
            flags: f,
            vars: vars,
        };

        /* dump::runinitial();
         *
         * initpath();
         * initpid();
         * signal::initsignals(runflags & run_interactive, allowquit);
         * var::hidevariables();
         * var::initenv(environ, protected);
         * */

        if es.flags.loginshell {
            // runesrc();
        }

        return Ok(es);
    }
}

/* checkfd -- open /dev/null on an fd if it is closed */
fn checkfd(fd: i32, r: u16) {
    unsafe {
        let new = libc::dup(fd);
        if new != -1 {
            libc::close(new);
        } else if errno() == errno::Errno(libc::EBADF) {
            let null = libc::open(CString::new("/dev/null").unwrap().into_raw(), 0, r as c_int);
            if null != -1i32 {
                fd::mvfd(new, fd);
            }
        }
    }
}
