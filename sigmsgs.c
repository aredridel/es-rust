#include "es.h"
#include "sigmsgs.h"

const Sigmsgs signals[] = {
#ifdef SIGHUP
	{ SIGHUP,	"sighup",	"hangup" },
#endif
#ifdef SIGINT
	{ SIGINT,	"sigint",	"" },
#endif
#ifdef SIGQUIT
	{ SIGQUIT,	"sigquit",	"quit" },
#endif
#ifdef SIGILL
	{ SIGILL,	"sigill",	"illegal instruction" },
#endif
#ifdef SIGTRAP
	{ SIGTRAP,	"sigtrap",	"trace trap" },
#endif
#ifdef SIGABRT
	{ SIGABRT,	"sigabrt",	"abort" },
#endif
#ifdef SIGPOLL
	{ SIGPOLL,	"sigpoll",	"pollable event occurred" },
#endif
#ifdef SIGIOT
	{ SIGIOT,	"sigiot",	"IOT instruction" },
#endif
#ifdef SIGEMT
	{ SIGEMT,	"sigemt",	"EMT instruction" },
#endif
#ifdef SIGFPE
	{ SIGFPE,	"sigfpe",	"floating point exception" },
#endif
#ifdef SIGKILL
	{ SIGKILL,	"sigkill",	"killed" },
#endif
#ifdef SIGBUS
	{ SIGBUS,	"sigbus",	"bus error" },
#endif
#ifdef SIGSEGV
	{ SIGSEGV,	"sigsegv",	"segmentation violation" },
#endif
#ifdef SIGSYS
	{ SIGSYS,	"sigsys",	"bad argument to system call" },
#endif
#ifdef SIGPIPE
	{ SIGPIPE,	"sigpipe",	"" },
#endif
#ifdef SIGALRM
	{ SIGALRM,	"sigalrm",	"alarm clock" },
#endif
#ifdef SIGTERM
	{ SIGTERM,	"sigterm",	"terminated" },
#endif
#ifdef SIGURG
	{ SIGURG,	"sigurg",	"urgent condition on i/o channel" },
#endif
#ifdef SIGSTOP
	{ SIGSTOP,	"sigstop",	"asynchronous stop" },
#endif
#ifdef SIGTSTP
	{ SIGTSTP,	"sigtstp",	"stopped" },
#endif
#ifdef SIGCONT
	{ SIGCONT,	"sigcont",	"continue" },
#endif
#ifdef SIGCHLD
	{ SIGCHLD,	"sigchld",	"child stopped or exited" },
#endif
#ifdef SIGTTIN
	{ SIGTTIN,	"sigttin",	"background tty read" },
#endif
#ifdef SIGTTOU
	{ SIGTTOU,	"sigttou",	"background tty write" },
#endif
#ifdef SIGIO
	{ SIGIO,	"sigio",	"input/output possible" },
#endif
#ifdef SIGXCPU
	{ SIGXCPU,	"sigxcpu",	"exceeded CPU time limit" },
#endif
#ifdef SIGXFSZ
	{ SIGXFSZ,	"sigxfsz",	"exceeded file size limit" },
#endif
#ifdef SIGVTALRM
	{ SIGVTALRM,	"sigvtalrm",	"virtual timer alarm" },
#endif
#ifdef SIGPROF
	{ SIGPROF,	"sigprof",	"profiling timer alarm" },
#endif
#ifdef SIGWINCH
	{ SIGWINCH,	"sigwinch",	"window size changed" },
#endif
#ifdef SIGINFO
	{ SIGINFO,	"siginfo",	"information request" },
#endif
#ifdef SIGUSR1
	{ SIGUSR1,	"sigusr1",	"user defined signal 1" },
#endif
#ifdef SIGUSR2
	{ SIGUSR2,	"sigusr2",	"user defined signal 2" },
#endif
};

const int nsignals = arraysize(signals);
