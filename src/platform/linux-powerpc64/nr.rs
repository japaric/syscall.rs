/* automatically generated by nr_from_src.py */

pub const _LLSEEK: usize = 140;
pub const _NEWSELECT: usize = 142;
pub const _SYSCTL: usize = 149;
pub const ACCEPT: usize = 330;
pub const ACCEPT4: usize = 344;
pub const ACCESS: usize = 33;
pub const ACCT: usize = 51;
pub const ADD_KEY: usize = 269;
pub const ADJTIMEX: usize = 124;
pub const AFS_SYSCALL: usize = 137;
pub const ALARM: usize = 27;
pub const BDFLUSH: usize = 134;
pub const BIND: usize = 327;
pub const BPF: usize = 361;
pub const BREAK: usize = 17;
pub const BRK: usize = 45;
pub const CAPGET: usize = 183;
pub const CAPSET: usize = 184;
pub const CHDIR: usize = 12;
pub const CHMOD: usize = 15;
pub const CHOWN: usize = 181;
pub const CHROOT: usize = 61;
pub const CLOCK_ADJTIME: usize = 347;
pub const CLOCK_GETRES: usize = 247;
pub const CLOCK_GETTIME: usize = 246;
pub const CLOCK_NANOSLEEP: usize = 248;
pub const CLOCK_SETTIME: usize = 245;
pub const CLONE: usize = 120;
pub const CLOSE: usize = 6;
pub const CONNECT: usize = 328;
pub const COPY_FILE_RANGE: usize = 379;
pub const CREAT: usize = 8;
pub const CREATE_MODULE: usize = 127;
pub const DELETE_MODULE: usize = 129;
pub const DUP: usize = 41;
pub const DUP2: usize = 63;
pub const DUP3: usize = 316;
pub const EPOLL_CREATE: usize = 236;
pub const EPOLL_CREATE1: usize = 315;
pub const EPOLL_CTL: usize = 237;
pub const EPOLL_PWAIT: usize = 303;
pub const EPOLL_WAIT: usize = 238;
pub const EVENTFD: usize = 307;
pub const EVENTFD2: usize = 314;
pub const EXECVE: usize = 11;
pub const EXECVEAT: usize = 362;
pub const EXIT: usize = 1;
pub const EXIT_GROUP: usize = 234;
pub const FACCESSAT: usize = 298;
pub const FADVISE64: usize = 233;
pub const FALLOCATE: usize = 309;
pub const FANOTIFY_INIT: usize = 323;
pub const FANOTIFY_MARK: usize = 324;
pub const FCHDIR: usize = 133;
pub const FCHMOD: usize = 94;
pub const FCHMODAT: usize = 297;
pub const FCHOWN: usize = 95;
pub const FCHOWNAT: usize = 289;
pub const FCNTL: usize = 55;
pub const FDATASYNC: usize = 148;
pub const FGETXATTR: usize = 214;
pub const FINIT_MODULE: usize = 353;
pub const FLISTXATTR: usize = 217;
pub const FLOCK: usize = 143;
pub const FORK: usize = 2;
pub const FREMOVEXATTR: usize = 220;
pub const FSETXATTR: usize = 211;
pub const FSTAT: usize = 108;
pub const FSTATFS: usize = 100;
pub const FSTATFS64: usize = 253;
pub const FSYNC: usize = 118;
pub const FTIME: usize = 35;
pub const FTRUNCATE: usize = 93;
pub const FUTEX: usize = 221;
pub const FUTIMESAT: usize = 290;
pub const GET_KERNEL_SYMS: usize = 130;
pub const GET_MEMPOLICY: usize = 260;
pub const GET_ROBUST_LIST: usize = 299;
pub const GETCPU: usize = 302;
pub const GETCWD: usize = 182;
pub const GETDENTS: usize = 141;
pub const GETDENTS64: usize = 202;
pub const GETEGID: usize = 50;
pub const GETEUID: usize = 49;
pub const GETGID: usize = 47;
pub const GETGROUPS: usize = 80;
pub const GETITIMER: usize = 105;
pub const GETPEERNAME: usize = 332;
pub const GETPGID: usize = 132;
pub const GETPGRP: usize = 65;
pub const GETPID: usize = 20;
pub const GETPMSG: usize = 187;
pub const GETPPID: usize = 64;
pub const GETPRIORITY: usize = 96;
pub const GETRANDOM: usize = 359;
pub const GETRESGID: usize = 170;
pub const GETRESUID: usize = 165;
pub const GETRLIMIT: usize = 76;
pub const GETRUSAGE: usize = 77;
pub const GETSID: usize = 147;
pub const GETSOCKNAME: usize = 331;
pub const GETSOCKOPT: usize = 340;
pub const GETTID: usize = 207;
pub const GETTIMEOFDAY: usize = 78;
pub const GETUID: usize = 24;
pub const GETXATTR: usize = 212;
pub const GTTY: usize = 32;
pub const IDLE: usize = 112;
pub const INIT_MODULE: usize = 128;
pub const INOTIFY_ADD_WATCH: usize = 276;
pub const INOTIFY_INIT: usize = 275;
pub const INOTIFY_INIT1: usize = 318;
pub const INOTIFY_RM_WATCH: usize = 277;
pub const IO_CANCEL: usize = 231;
pub const IO_DESTROY: usize = 228;
pub const IO_GETEVENTS: usize = 229;
pub const IO_SETUP: usize = 227;
pub const IO_SUBMIT: usize = 230;
pub const IOCTL: usize = 54;
pub const IOPERM: usize = 101;
pub const IOPL: usize = 110;
pub const IOPRIO_GET: usize = 274;
pub const IOPRIO_SET: usize = 273;
pub const IPC: usize = 117;
pub const KCMP: usize = 354;
pub const KEXEC_FILE_LOAD: usize = 382;
pub const KEXEC_LOAD: usize = 268;
pub const KEYCTL: usize = 271;
pub const KILL: usize = 37;
pub const LCHOWN: usize = 16;
pub const LGETXATTR: usize = 213;
pub const LINK: usize = 9;
pub const LINKAT: usize = 294;
pub const LISTEN: usize = 329;
pub const LISTXATTR: usize = 215;
pub const LLISTXATTR: usize = 216;
pub const LOCK: usize = 53;
pub const LOOKUP_DCOOKIE: usize = 235;
pub const LREMOVEXATTR: usize = 219;
pub const LSEEK: usize = 19;
pub const LSETXATTR: usize = 210;
pub const LSTAT: usize = 107;
pub const MADVISE: usize = 205;
pub const MBIND: usize = 259;
pub const MEMBARRIER: usize = 365;
pub const MEMFD_CREATE: usize = 360;
pub const MIGRATE_PAGES: usize = 258;
pub const MINCORE: usize = 206;
pub const MKDIR: usize = 39;
pub const MKDIRAT: usize = 287;
pub const MKNOD: usize = 14;
pub const MKNODAT: usize = 288;
pub const MLOCK: usize = 150;
pub const MLOCK2: usize = 378;
pub const MLOCKALL: usize = 152;
pub const MMAP: usize = 90;
pub const MODIFY_LDT: usize = 123;
pub const MOUNT: usize = 21;
pub const MOVE_PAGES: usize = 301;
pub const MPROTECT: usize = 125;
pub const MPX: usize = 56;
pub const MQ_GETSETATTR: usize = 267;
pub const MQ_NOTIFY: usize = 266;
pub const MQ_OPEN: usize = 262;
pub const MQ_TIMEDRECEIVE: usize = 265;
pub const MQ_TIMEDSEND: usize = 264;
pub const MQ_UNLINK: usize = 263;
pub const MREMAP: usize = 163;
pub const MSYNC: usize = 144;
pub const MULTIPLEXER: usize = 201;
pub const MUNLOCK: usize = 151;
pub const MUNLOCKALL: usize = 153;
pub const MUNMAP: usize = 91;
pub const NAME_TO_HANDLE_AT: usize = 345;
pub const NANOSLEEP: usize = 162;
pub const NEWFSTATAT: usize = 291;
pub const NFSSERVCTL: usize = 168;
pub const NICE: usize = 34;
pub const OLDFSTAT: usize = 28;
pub const OLDLSTAT: usize = 84;
pub const OLDOLDUNAME: usize = 59;
pub const OLDSTAT: usize = 18;
pub const OLDUNAME: usize = 109;
pub const OPEN: usize = 5;
pub const OPEN_BY_HANDLE_AT: usize = 346;
pub const OPENAT: usize = 286;
pub const PAUSE: usize = 29;
pub const PCICONFIG_IOBASE: usize = 200;
pub const PCICONFIG_READ: usize = 198;
pub const PCICONFIG_WRITE: usize = 199;
pub const PERF_EVENT_OPEN: usize = 319;
pub const PERSONALITY: usize = 136;
pub const PIPE: usize = 42;
pub const PIPE2: usize = 317;
pub const PIVOT_ROOT: usize = 203;
pub const POLL: usize = 167;
pub const PPOLL: usize = 281;
pub const PRCTL: usize = 171;
pub const PREAD64: usize = 179;
pub const PREADV: usize = 320;
pub const PREADV2: usize = 380;
pub const PRLIMIT64: usize = 325;
pub const PROCESS_VM_READV: usize = 351;
pub const PROCESS_VM_WRITEV: usize = 352;
pub const PROF: usize = 44;
pub const PROFIL: usize = 98;
pub const PSELECT6: usize = 280;
pub const PTRACE: usize = 26;
pub const PUTPMSG: usize = 188;
pub const PWRITE64: usize = 180;
pub const PWRITEV: usize = 321;
pub const PWRITEV2: usize = 381;
pub const QUERY_MODULE: usize = 166;
pub const QUOTACTL: usize = 131;
pub const READ: usize = 3;
pub const READAHEAD: usize = 191;
pub const READDIR: usize = 89;
pub const READLINK: usize = 85;
pub const READLINKAT: usize = 296;
pub const READV: usize = 145;
pub const REBOOT: usize = 88;
pub const RECV: usize = 336;
pub const RECVFROM: usize = 337;
pub const RECVMMSG: usize = 343;
pub const RECVMSG: usize = 342;
pub const REMAP_FILE_PAGES: usize = 239;
pub const REMOVEXATTR: usize = 218;
pub const RENAME: usize = 38;
pub const RENAMEAT: usize = 293;
pub const RENAMEAT2: usize = 357;
pub const REQUEST_KEY: usize = 270;
pub const RESTART_SYSCALL: usize = 0;
pub const RMDIR: usize = 40;
pub const RT_SIGACTION: usize = 173;
pub const RT_SIGPENDING: usize = 175;
pub const RT_SIGPROCMASK: usize = 174;
pub const RT_SIGQUEUEINFO: usize = 177;
pub const RT_SIGRETURN: usize = 172;
pub const RT_SIGSUSPEND: usize = 178;
pub const RT_SIGTIMEDWAIT: usize = 176;
pub const RT_TGSIGQUEUEINFO: usize = 322;
pub const RTAS: usize = 255;
pub const SCHED_GET_PRIORITY_MAX: usize = 159;
pub const SCHED_GET_PRIORITY_MIN: usize = 160;
pub const SCHED_GETAFFINITY: usize = 223;
pub const SCHED_GETATTR: usize = 356;
pub const SCHED_GETPARAM: usize = 155;
pub const SCHED_GETSCHEDULER: usize = 157;
pub const SCHED_RR_GET_INTERVAL: usize = 161;
pub const SCHED_SETAFFINITY: usize = 222;
pub const SCHED_SETATTR: usize = 355;
pub const SCHED_SETPARAM: usize = 154;
pub const SCHED_SETSCHEDULER: usize = 156;
pub const SCHED_YIELD: usize = 158;
pub const SECCOMP: usize = 358;
pub const SELECT: usize = 82;
pub const SEND: usize = 334;
pub const SENDFILE: usize = 186;
pub const SENDMMSG: usize = 349;
pub const SENDMSG: usize = 341;
pub const SENDTO: usize = 335;
pub const SET_MEMPOLICY: usize = 261;
pub const SET_ROBUST_LIST: usize = 300;
pub const SET_TID_ADDRESS: usize = 232;
pub const SETDOMAINNAME: usize = 121;
pub const SETFSGID: usize = 139;
pub const SETFSUID: usize = 138;
pub const SETGID: usize = 46;
pub const SETGROUPS: usize = 81;
pub const SETHOSTNAME: usize = 74;
pub const SETITIMER: usize = 104;
pub const SETNS: usize = 350;
pub const SETPGID: usize = 57;
pub const SETPRIORITY: usize = 97;
pub const SETREGID: usize = 71;
pub const SETRESGID: usize = 169;
pub const SETRESUID: usize = 164;
pub const SETREUID: usize = 70;
pub const SETRLIMIT: usize = 75;
pub const SETSID: usize = 66;
pub const SETSOCKOPT: usize = 339;
pub const SETTIMEOFDAY: usize = 79;
pub const SETUID: usize = 23;
pub const SETXATTR: usize = 209;
pub const SGETMASK: usize = 68;
pub const SHUTDOWN: usize = 338;
pub const SIGACTION: usize = 67;
pub const SIGALTSTACK: usize = 185;
pub const SIGNAL: usize = 48;
pub const SIGNALFD: usize = 305;
pub const SIGNALFD4: usize = 313;
pub const SIGPENDING: usize = 73;
pub const SIGPROCMASK: usize = 126;
pub const SIGRETURN: usize = 119;
pub const SIGSUSPEND: usize = 72;
pub const SOCKET: usize = 326;
pub const SOCKETCALL: usize = 102;
pub const SOCKETPAIR: usize = 333;
pub const SPLICE: usize = 283;
pub const SPU_CREATE: usize = 279;
pub const SPU_RUN: usize = 278;
pub const SSETMASK: usize = 69;
pub const STAT: usize = 106;
pub const STATFS: usize = 99;
pub const STATFS64: usize = 252;
pub const STATX: usize = 383;
pub const STIME: usize = 25;
pub const STTY: usize = 31;
pub const SUBPAGE_PROT: usize = 310;
pub const SWAPCONTEXT: usize = 249;
pub const SWAPOFF: usize = 115;
pub const SWAPON: usize = 87;
pub const SWITCH_ENDIAN: usize = 363;
pub const SYMLINK: usize = 83;
pub const SYMLINKAT: usize = 295;
pub const SYNC: usize = 36;
pub const SYNC_FILE_RANGE2: usize = 308;
pub const SYNCFS: usize = 348;
pub const SYS_DEBUG_SETCONTEXT: usize = 256;
pub const SYSFS: usize = 135;
pub const SYSINFO: usize = 116;
pub const SYSLOG: usize = 103;
pub const TEE: usize = 284;
pub const TGKILL: usize = 250;
pub const TIME: usize = 13;
pub const TIMER_CREATE: usize = 240;
pub const TIMER_DELETE: usize = 244;
pub const TIMER_GETOVERRUN: usize = 243;
pub const TIMER_GETTIME: usize = 242;
pub const TIMER_SETTIME: usize = 241;
pub const TIMERFD_CREATE: usize = 306;
pub const TIMERFD_GETTIME: usize = 312;
pub const TIMERFD_SETTIME: usize = 311;
pub const TIMES: usize = 43;
pub const TKILL: usize = 208;
pub const TRUNCATE: usize = 92;
pub const TUXCALL: usize = 225;
pub const UGETRLIMIT: usize = 190;
pub const ULIMIT: usize = 58;
pub const UMASK: usize = 60;
pub const UMOUNT: usize = 22;
pub const UMOUNT2: usize = 52;
pub const UNAME: usize = 122;
pub const UNLINK: usize = 10;
pub const UNLINKAT: usize = 292;
pub const UNSHARE: usize = 282;
pub const USELIB: usize = 86;
pub const USERFAULTFD: usize = 364;
pub const USTAT: usize = 62;
pub const UTIME: usize = 30;
pub const UTIMENSAT: usize = 304;
pub const UTIMES: usize = 251;
pub const VFORK: usize = 189;
pub const VHANGUP: usize = 111;
pub const VM86: usize = 113;
pub const VMSPLICE: usize = 285;
pub const WAIT4: usize = 114;
pub const WAITID: usize = 272;
pub const WAITPID: usize = 7;
pub const WRITE: usize = 4;
pub const WRITEV: usize = 146;
