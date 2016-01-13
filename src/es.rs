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
    pub allowquit: bool
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
            allowquit: self.allowquit.clone()
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
    Pipe(Box<Tree>, Box<Tree>)
}

#[allow(dead_code)]
pub struct Es {
#[allow(dead_code)]
    flags: Flags
    //vars: Dict
}
