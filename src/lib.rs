//! # IBM_DB
//!
//! `ibm_db` is a library for connecting to DB2.

// suppress for the whole module with inner attribute...
#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, dead_code, improper_ctypes)]
#[repr(C)]
#[derive(Copy, Clone, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct __BindgenBitfieldUnit<Storage, Align> {
    storage: Storage,
    align: [Align; 0],
}
impl<Storage, Align> __BindgenBitfieldUnit<Storage, Align> {
    #[inline]
    pub const fn new(storage: Storage) -> Self {
        Self { storage, align: [] }
    }
}
impl<Storage, Align> __BindgenBitfieldUnit<Storage, Align>
where
    Storage: AsRef<[u8]> + AsMut<[u8]>,
{
    #[inline]
    pub fn get_bit(&self, index: usize) -> bool {
        debug_assert!(index / 8 < self.storage.as_ref().len());
        let byte_index = index / 8;
        let byte = self.storage.as_ref()[byte_index];
        let bit_index = if cfg!(target_endian = "big") {
            7 - (index % 8)
        } else {
            index % 8
        };
        let mask = 1 << bit_index;
        byte & mask == mask
    }
    #[inline]
    pub fn set_bit(&mut self, index: usize, val: bool) {
        debug_assert!(index / 8 < self.storage.as_ref().len());
        let byte_index = index / 8;
        let byte = &mut self.storage.as_mut()[byte_index];
        let bit_index = if cfg!(target_endian = "big") {
            7 - (index % 8)
        } else {
            index % 8
        };
        let mask = 1 << bit_index;
        if val {
            *byte |= mask;
        } else {
            *byte &= !mask;
        }
    }
    #[inline]
    pub fn get(&self, bit_offset: usize, bit_width: u8) -> u64 {
        debug_assert!(bit_width <= 64);
        debug_assert!(bit_offset / 8 < self.storage.as_ref().len());
        debug_assert!((bit_offset + (bit_width as usize)) / 8 <= self.storage.as_ref().len());
        let mut val = 0;
        for i in 0..(bit_width as usize) {
            if self.get_bit(i + bit_offset) {
                let index = if cfg!(target_endian = "big") {
                    bit_width as usize - 1 - i
                } else {
                    i
                };
                val |= 1 << index;
            }
        }
        val
    }
    #[inline]
    pub fn set(&mut self, bit_offset: usize, bit_width: u8, val: u64) {
        debug_assert!(bit_width <= 64);
        debug_assert!(bit_offset / 8 < self.storage.as_ref().len());
        debug_assert!((bit_offset + (bit_width as usize)) / 8 <= self.storage.as_ref().len());
        for i in 0..(bit_width as usize) {
            let mask = 1 << i;
            let val_bit_is_set = val & mask == mask;
            let index = if cfg!(target_endian = "big") {
                bit_width as usize - 1 - i
            } else {
                i
            };
            self.set_bit(index + bit_offset, val_bit_is_set);
        }
    }
}
pub const DB2DARWIN: u32 = 1;
pub const SQL_CMP_NA_ERRORS: u32 = 1;
pub const SQL_CMP_ROWS_AFFECTED: u32 = 2;
pub const SQL_CMP_STMTS_COMPLETED: u32 = 3;
pub const SQL_CMP_REF_INT_ROWS: u32 = 4;
pub const SQL_CONNECT_DB_APP2DB_CONVFACTOR: u32 = 0;
pub const SQL_CONNECT_DB_DB2APP_CONVFACTOR: u32 = 1;
pub const SQL_CONNECT_DB_UPDATEABILITY_IN_UOW: u32 = 2;
pub const SQL_CONNECT_DB_COMMIT_TYPE: u32 = 3;
pub const SQL_DB_UPDATEABLE: u32 = 1;
pub const SQL_DB_READ_ONLY: u32 = 2;
pub const SQL_DB_ONE_PHASE_COMMIT: u32 = 1;
pub const SQL_DB_ONE_PHASE_READ_ONLY: u32 = 2;
pub const SQL_DB_TWO_PHASE_COMMIT: u32 = 3;
pub const SQL_ERRD_NODE_NUM: u32 = 1;
pub const DB2CLI_VER: u32 = 784;
pub const __MAC_10_0: u32 = 1000;
pub const __MAC_10_1: u32 = 1010;
pub const __MAC_10_2: u32 = 1020;
pub const __MAC_10_3: u32 = 1030;
pub const __MAC_10_4: u32 = 1040;
pub const __MAC_10_5: u32 = 1050;
pub const __MAC_10_6: u32 = 1060;
pub const __MAC_10_7: u32 = 1070;
pub const __MAC_10_8: u32 = 1080;
pub const __MAC_10_9: u32 = 1090;
pub const __MAC_10_10: u32 = 101000;
pub const __MAC_10_10_2: u32 = 101002;
pub const __MAC_10_10_3: u32 = 101003;
pub const __MAC_10_11: u32 = 101100;
pub const __MAC_10_11_2: u32 = 101102;
pub const __MAC_10_11_3: u32 = 101103;
pub const __MAC_10_11_4: u32 = 101104;
pub const __MAC_10_12: u32 = 101200;
pub const __MAC_10_12_1: u32 = 101201;
pub const __MAC_10_12_2: u32 = 101202;
pub const __MAC_10_12_4: u32 = 101204;
pub const __MAC_10_13: u32 = 101300;
pub const __MAC_10_13_1: u32 = 101301;
pub const __MAC_10_13_2: u32 = 101302;
pub const __MAC_10_13_4: u32 = 101304;
pub const __IPHONE_2_0: u32 = 20000;
pub const __IPHONE_2_1: u32 = 20100;
pub const __IPHONE_2_2: u32 = 20200;
pub const __IPHONE_3_0: u32 = 30000;
pub const __IPHONE_3_1: u32 = 30100;
pub const __IPHONE_3_2: u32 = 30200;
pub const __IPHONE_4_0: u32 = 40000;
pub const __IPHONE_4_1: u32 = 40100;
pub const __IPHONE_4_2: u32 = 40200;
pub const __IPHONE_4_3: u32 = 40300;
pub const __IPHONE_5_0: u32 = 50000;
pub const __IPHONE_5_1: u32 = 50100;
pub const __IPHONE_6_0: u32 = 60000;
pub const __IPHONE_6_1: u32 = 60100;
pub const __IPHONE_7_0: u32 = 70000;
pub const __IPHONE_7_1: u32 = 70100;
pub const __IPHONE_8_0: u32 = 80000;
pub const __IPHONE_8_1: u32 = 80100;
pub const __IPHONE_8_2: u32 = 80200;
pub const __IPHONE_8_3: u32 = 80300;
pub const __IPHONE_8_4: u32 = 80400;
pub const __IPHONE_9_0: u32 = 90000;
pub const __IPHONE_9_1: u32 = 90100;
pub const __IPHONE_9_2: u32 = 90200;
pub const __IPHONE_9_3: u32 = 90300;
pub const __IPHONE_10_0: u32 = 100000;
pub const __IPHONE_10_1: u32 = 100100;
pub const __IPHONE_10_2: u32 = 100200;
pub const __IPHONE_10_3: u32 = 100300;
pub const __IPHONE_11_0: u32 = 110000;
pub const __IPHONE_11_1: u32 = 110100;
pub const __IPHONE_11_2: u32 = 110200;
pub const __IPHONE_11_3: u32 = 110300;
pub const __TVOS_9_0: u32 = 90000;
pub const __TVOS_9_1: u32 = 90100;
pub const __TVOS_9_2: u32 = 90200;
pub const __TVOS_10_0: u32 = 100000;
pub const __TVOS_10_0_1: u32 = 100001;
pub const __TVOS_10_1: u32 = 100100;
pub const __TVOS_10_2: u32 = 100200;
pub const __TVOS_11_0: u32 = 110000;
pub const __TVOS_11_1: u32 = 110100;
pub const __TVOS_11_2: u32 = 110200;
pub const __TVOS_11_3: u32 = 110300;
pub const __WATCHOS_1_0: u32 = 10000;
pub const __WATCHOS_2_0: u32 = 20000;
pub const __WATCHOS_2_1: u32 = 20100;
pub const __WATCHOS_2_2: u32 = 20200;
pub const __WATCHOS_3_0: u32 = 30000;
pub const __WATCHOS_3_1: u32 = 30100;
pub const __WATCHOS_3_1_1: u32 = 30101;
pub const __WATCHOS_3_2: u32 = 30200;
pub const __WATCHOS_4_0: u32 = 40000;
pub const __WATCHOS_4_1: u32 = 40100;
pub const __WATCHOS_4_2: u32 = 40200;
pub const __WATCHOS_4_3: u32 = 40300;
pub const __MAC_OS_X_VERSION_MAX_ALLOWED: u32 = 101304;
pub const __DARWIN_ONLY_64_BIT_INO_T: u32 = 0;
pub const __DARWIN_ONLY_VERS_1050: u32 = 0;
pub const __DARWIN_ONLY_UNIX_CONFORMANCE: u32 = 1;
pub const __DARWIN_UNIX03: u32 = 1;
pub const __DARWIN_64_BIT_INO_T: u32 = 1;
pub const __DARWIN_VERS_1050: u32 = 1;
pub const __DARWIN_NON_CANCELABLE: u32 = 0;
pub const __DARWIN_SUF_64_BIT_INO_T: &'static [u8; 9usize] = b"$INODE64\0";
pub const __DARWIN_SUF_1050: &'static [u8; 6usize] = b"$1050\0";
pub const __DARWIN_SUF_EXTSN: &'static [u8; 14usize] = b"$DARWIN_EXTSN\0";
pub const __DARWIN_C_ANSI: u32 = 4096;
pub const __DARWIN_C_FULL: u32 = 900000;
pub const __DARWIN_C_LEVEL: u32 = 900000;
pub const __STDC_WANT_LIB_EXT1__: u32 = 1;
pub const __DARWIN_NO_LONG_LONG: u32 = 0;
pub const _DARWIN_FEATURE_64_BIT_INODE: u32 = 1;
pub const _DARWIN_FEATURE_ONLY_UNIX_CONFORMANCE: u32 = 1;
pub const _DARWIN_FEATURE_UNIX_CONFORMANCE: u32 = 3;
pub const __PTHREAD_SIZE__: u32 = 8176;
pub const __PTHREAD_ATTR_SIZE__: u32 = 56;
pub const __PTHREAD_MUTEXATTR_SIZE__: u32 = 8;
pub const __PTHREAD_MUTEX_SIZE__: u32 = 56;
pub const __PTHREAD_CONDATTR_SIZE__: u32 = 8;
pub const __PTHREAD_COND_SIZE__: u32 = 40;
pub const __PTHREAD_ONCE_SIZE__: u32 = 8;
pub const __PTHREAD_RWLOCK_SIZE__: u32 = 192;
pub const __PTHREAD_RWLOCKATTR_SIZE__: u32 = 16;
pub const __DARWIN_WCHAR_MIN: i32 = -2147483648;
pub const _FORTIFY_SOURCE: u32 = 2;
pub const __DARWIN_NSIG: u32 = 32;
pub const NSIG: u32 = 32;
pub const _I386_SIGNAL_H_: u32 = 1;
pub const SIGHUP: u32 = 1;
pub const SIGINT: u32 = 2;
pub const SIGQUIT: u32 = 3;
pub const SIGILL: u32 = 4;
pub const SIGTRAP: u32 = 5;
pub const SIGABRT: u32 = 6;
pub const SIGIOT: u32 = 6;
pub const SIGEMT: u32 = 7;
pub const SIGFPE: u32 = 8;
pub const SIGKILL: u32 = 9;
pub const SIGBUS: u32 = 10;
pub const SIGSEGV: u32 = 11;
pub const SIGSYS: u32 = 12;
pub const SIGPIPE: u32 = 13;
pub const SIGALRM: u32 = 14;
pub const SIGTERM: u32 = 15;
pub const SIGURG: u32 = 16;
pub const SIGSTOP: u32 = 17;
pub const SIGTSTP: u32 = 18;
pub const SIGCONT: u32 = 19;
pub const SIGCHLD: u32 = 20;
pub const SIGTTIN: u32 = 21;
pub const SIGTTOU: u32 = 22;
pub const SIGIO: u32 = 23;
pub const SIGXCPU: u32 = 24;
pub const SIGXFSZ: u32 = 25;
pub const SIGVTALRM: u32 = 26;
pub const SIGPROF: u32 = 27;
pub const SIGWINCH: u32 = 28;
pub const SIGINFO: u32 = 29;
pub const SIGUSR1: u32 = 30;
pub const SIGUSR2: u32 = 31;
pub const FP_PREC_24B: u32 = 0;
pub const FP_PREC_53B: u32 = 2;
pub const FP_PREC_64B: u32 = 3;
pub const FP_RND_NEAR: u32 = 0;
pub const FP_RND_DOWN: u32 = 1;
pub const FP_RND_UP: u32 = 2;
pub const FP_CHOP: u32 = 3;
pub const FP_STATE_BYTES: u32 = 512;
pub const SIGEV_NONE: u32 = 0;
pub const SIGEV_SIGNAL: u32 = 1;
pub const SIGEV_THREAD: u32 = 3;
pub const ILL_NOOP: u32 = 0;
pub const ILL_ILLOPC: u32 = 1;
pub const ILL_ILLTRP: u32 = 2;
pub const ILL_PRVOPC: u32 = 3;
pub const ILL_ILLOPN: u32 = 4;
pub const ILL_ILLADR: u32 = 5;
pub const ILL_PRVREG: u32 = 6;
pub const ILL_COPROC: u32 = 7;
pub const ILL_BADSTK: u32 = 8;
pub const FPE_NOOP: u32 = 0;
pub const FPE_FLTDIV: u32 = 1;
pub const FPE_FLTOVF: u32 = 2;
pub const FPE_FLTUND: u32 = 3;
pub const FPE_FLTRES: u32 = 4;
pub const FPE_FLTINV: u32 = 5;
pub const FPE_FLTSUB: u32 = 6;
pub const FPE_INTDIV: u32 = 7;
pub const FPE_INTOVF: u32 = 8;
pub const SEGV_NOOP: u32 = 0;
pub const SEGV_MAPERR: u32 = 1;
pub const SEGV_ACCERR: u32 = 2;
pub const BUS_NOOP: u32 = 0;
pub const BUS_ADRALN: u32 = 1;
pub const BUS_ADRERR: u32 = 2;
pub const BUS_OBJERR: u32 = 3;
pub const TRAP_BRKPT: u32 = 1;
pub const TRAP_TRACE: u32 = 2;
pub const CLD_NOOP: u32 = 0;
pub const CLD_EXITED: u32 = 1;
pub const CLD_KILLED: u32 = 2;
pub const CLD_DUMPED: u32 = 3;
pub const CLD_TRAPPED: u32 = 4;
pub const CLD_STOPPED: u32 = 5;
pub const CLD_CONTINUED: u32 = 6;
pub const POLL_IN: u32 = 1;
pub const POLL_OUT: u32 = 2;
pub const POLL_MSG: u32 = 3;
pub const POLL_ERR: u32 = 4;
pub const POLL_PRI: u32 = 5;
pub const POLL_HUP: u32 = 6;
pub const SA_ONSTACK: u32 = 1;
pub const SA_RESTART: u32 = 2;
pub const SA_RESETHAND: u32 = 4;
pub const SA_NOCLDSTOP: u32 = 8;
pub const SA_NODEFER: u32 = 16;
pub const SA_NOCLDWAIT: u32 = 32;
pub const SA_SIGINFO: u32 = 64;
pub const SA_USERTRAMP: u32 = 256;
pub const SA_64REGSET: u32 = 512;
pub const SA_USERSPACE_MASK: u32 = 127;
pub const SIG_BLOCK: u32 = 1;
pub const SIG_UNBLOCK: u32 = 2;
pub const SIG_SETMASK: u32 = 3;
pub const SI_USER: u32 = 65537;
pub const SI_QUEUE: u32 = 65538;
pub const SI_TIMER: u32 = 65539;
pub const SI_ASYNCIO: u32 = 65540;
pub const SI_MESGQ: u32 = 65541;
pub const SS_ONSTACK: u32 = 1;
pub const SS_DISABLE: u32 = 4;
pub const MINSIGSTKSZ: u32 = 32768;
pub const SIGSTKSZ: u32 = 131072;
pub const SV_ONSTACK: u32 = 1;
pub const SV_INTERRUPT: u32 = 2;
pub const SV_RESETHAND: u32 = 4;
pub const SV_NODEFER: u32 = 16;
pub const SV_NOCLDSTOP: u32 = 8;
pub const SV_SIGINFO: u32 = 64;
pub const __WORDSIZE: u32 = 64;
pub const INT8_MAX: u32 = 127;
pub const INT16_MAX: u32 = 32767;
pub const INT32_MAX: u32 = 2147483647;
pub const INT64_MAX: u64 = 9223372036854775807;
pub const INT8_MIN: i32 = -128;
pub const INT16_MIN: i32 = -32768;
pub const INT32_MIN: i32 = -2147483648;
pub const INT64_MIN: i64 = -9223372036854775808;
pub const UINT8_MAX: u32 = 255;
pub const UINT16_MAX: u32 = 65535;
pub const UINT32_MAX: u32 = 4294967295;
pub const UINT64_MAX: i32 = -1;
pub const INT_LEAST8_MIN: i32 = -128;
pub const INT_LEAST16_MIN: i32 = -32768;
pub const INT_LEAST32_MIN: i32 = -2147483648;
pub const INT_LEAST64_MIN: i64 = -9223372036854775808;
pub const INT_LEAST8_MAX: u32 = 127;
pub const INT_LEAST16_MAX: u32 = 32767;
pub const INT_LEAST32_MAX: u32 = 2147483647;
pub const INT_LEAST64_MAX: u64 = 9223372036854775807;
pub const UINT_LEAST8_MAX: u32 = 255;
pub const UINT_LEAST16_MAX: u32 = 65535;
pub const UINT_LEAST32_MAX: u32 = 4294967295;
pub const UINT_LEAST64_MAX: i32 = -1;
pub const INT_FAST8_MIN: i32 = -128;
pub const INT_FAST16_MIN: i32 = -32768;
pub const INT_FAST32_MIN: i32 = -2147483648;
pub const INT_FAST64_MIN: i64 = -9223372036854775808;
pub const INT_FAST8_MAX: u32 = 127;
pub const INT_FAST16_MAX: u32 = 32767;
pub const INT_FAST32_MAX: u32 = 2147483647;
pub const INT_FAST64_MAX: u64 = 9223372036854775807;
pub const UINT_FAST8_MAX: u32 = 255;
pub const UINT_FAST16_MAX: u32 = 65535;
pub const UINT_FAST32_MAX: u32 = 4294967295;
pub const UINT_FAST64_MAX: i32 = -1;
pub const INTPTR_MAX: u64 = 9223372036854775807;
pub const INTPTR_MIN: i64 = -9223372036854775808;
pub const UINTPTR_MAX: i32 = -1;
pub const INTMAX_MIN: i64 = -9223372036854775808;
pub const INTMAX_MAX: u64 = 9223372036854775807;
pub const UINTMAX_MAX: i32 = -1;
pub const PTRDIFF_MIN: i64 = -9223372036854775808;
pub const PTRDIFF_MAX: u64 = 9223372036854775807;
pub const SIZE_MAX: i32 = -1;
pub const RSIZE_MAX: i32 = -1;
pub const WINT_MIN: i32 = -2147483648;
pub const WINT_MAX: u32 = 2147483647;
pub const SIG_ATOMIC_MIN: i32 = -2147483648;
pub const SIG_ATOMIC_MAX: u32 = 2147483647;
pub const PRIO_PROCESS: u32 = 0;
pub const PRIO_PGRP: u32 = 1;
pub const PRIO_USER: u32 = 2;
pub const PRIO_DARWIN_THREAD: u32 = 3;
pub const PRIO_DARWIN_PROCESS: u32 = 4;
pub const PRIO_MIN: i32 = -20;
pub const PRIO_MAX: u32 = 20;
pub const PRIO_DARWIN_BG: u32 = 4096;
pub const PRIO_DARWIN_NONUI: u32 = 4097;
pub const RUSAGE_SELF: u32 = 0;
pub const RUSAGE_CHILDREN: i32 = -1;
pub const RUSAGE_INFO_V0: u32 = 0;
pub const RUSAGE_INFO_V1: u32 = 1;
pub const RUSAGE_INFO_V2: u32 = 2;
pub const RUSAGE_INFO_V3: u32 = 3;
pub const RUSAGE_INFO_V4: u32 = 4;
pub const RUSAGE_INFO_CURRENT: u32 = 4;
pub const RLIMIT_CPU: u32 = 0;
pub const RLIMIT_FSIZE: u32 = 1;
pub const RLIMIT_DATA: u32 = 2;
pub const RLIMIT_STACK: u32 = 3;
pub const RLIMIT_CORE: u32 = 4;
pub const RLIMIT_AS: u32 = 5;
pub const RLIMIT_RSS: u32 = 5;
pub const RLIMIT_MEMLOCK: u32 = 6;
pub const RLIMIT_NPROC: u32 = 7;
pub const RLIMIT_NOFILE: u32 = 8;
pub const RLIM_NLIMITS: u32 = 9;
pub const _RLIMIT_POSIX_FLAG: u32 = 4096;
pub const RLIMIT_WAKEUPS_MONITOR: u32 = 1;
pub const RLIMIT_CPU_USAGE_MONITOR: u32 = 2;
pub const RLIMIT_THREAD_CPULIMITS: u32 = 3;
pub const WAKEMON_ENABLE: u32 = 1;
pub const WAKEMON_DISABLE: u32 = 2;
pub const WAKEMON_GET_PARAMS: u32 = 4;
pub const WAKEMON_SET_DEFAULTS: u32 = 8;
pub const WAKEMON_MAKE_FATAL: u32 = 16;
pub const CPUMON_MAKE_FATAL: u32 = 4096;
pub const IOPOL_TYPE_DISK: u32 = 0;
pub const IOPOL_SCOPE_PROCESS: u32 = 0;
pub const IOPOL_SCOPE_THREAD: u32 = 1;
pub const IOPOL_SCOPE_DARWIN_BG: u32 = 2;
pub const IOPOL_DEFAULT: u32 = 0;
pub const IOPOL_IMPORTANT: u32 = 1;
pub const IOPOL_PASSIVE: u32 = 2;
pub const IOPOL_THROTTLE: u32 = 3;
pub const IOPOL_UTILITY: u32 = 4;
pub const IOPOL_STANDARD: u32 = 5;
pub const IOPOL_APPLICATION: u32 = 5;
pub const IOPOL_NORMAL: u32 = 1;
pub const WNOHANG: u32 = 1;
pub const WUNTRACED: u32 = 2;
pub const WCOREFLAG: u32 = 128;
pub const _WSTOPPED: u32 = 127;
pub const WEXITED: u32 = 4;
pub const WSTOPPED: u32 = 8;
pub const WCONTINUED: u32 = 16;
pub const WNOWAIT: u32 = 32;
pub const WAIT_ANY: i32 = -1;
pub const WAIT_MYPGRP: u32 = 0;
pub const _QUAD_HIGHWORD: u32 = 1;
pub const _QUAD_LOWWORD: u32 = 0;
pub const __DARWIN_LITTLE_ENDIAN: u32 = 1234;
pub const __DARWIN_BIG_ENDIAN: u32 = 4321;
pub const __DARWIN_PDP_ENDIAN: u32 = 3412;
pub const __DARWIN_BYTE_ORDER: u32 = 1234;
pub const LITTLE_ENDIAN: u32 = 1234;
pub const BIG_ENDIAN: u32 = 4321;
pub const PDP_ENDIAN: u32 = 3412;
pub const BYTE_ORDER: u32 = 1234;
pub const EXIT_FAILURE: u32 = 1;
pub const EXIT_SUCCESS: u32 = 0;
pub const RAND_MAX: u32 = 2147483647;
pub const SQL_MAX_MESSAGE_LENGTH: u32 = 1024;
pub const SQL_MAX_ID_LENGTH: u32 = 128;
pub const SQL_DATE_LEN: u32 = 10;
pub const SQL_TIME_LEN: u32 = 8;
pub const SQL_TIMESTAMP_LEN: u32 = 19;
pub const SQL_TIMESTAMPTZ_LEN: u32 = 25;
pub const SQL_HANDLE_ENV: u32 = 1;
pub const SQL_HANDLE_DBC: u32 = 2;
pub const SQL_HANDLE_STMT: u32 = 3;
pub const SQL_HANDLE_DESC: u32 = 4;
pub const SQL_SUCCESS: u32 = 0;
pub const SQL_SUCCESS_WITH_INFO: u32 = 1;
pub const SQL_NEED_DATA: u32 = 99;
pub const SQL_NO_DATA: u32 = 100;
pub const SQL_STILL_EXECUTING: u32 = 2;
pub const SQL_ERROR: i32 = -1;
pub const SQL_INVALID_HANDLE: i32 = -2;
pub const SQL_CLOSE: u32 = 0;
pub const SQL_DROP: u32 = 1;
pub const SQL_UNBIND: u32 = 2;
pub const SQL_RESET_PARAMS: u32 = 3;
pub const SQL_COMMIT: u32 = 0;
pub const SQL_ROLLBACK: u32 = 1;
pub const SQL_UNKNOWN_TYPE: u32 = 0;
pub const SQL_CHAR: u32 = 1;
pub const SQL_NUMERIC: u32 = 2;
pub const SQL_DECIMAL: u32 = 3;
pub const SQL_INTEGER: u32 = 4;
pub const SQL_SMALLINT: u32 = 5;
pub const SQL_FLOAT: u32 = 6;
pub const SQL_REAL: u32 = 7;
pub const SQL_DOUBLE: u32 = 8;
pub const SQL_DATETIME: u32 = 9;
pub const SQL_VARCHAR: u32 = 12;
pub const SQL_BOOLEAN: u32 = 16;
pub const SQL_ROW: u32 = 19;
pub const SQL_WCHAR: i32 = -8;
pub const SQL_WVARCHAR: i32 = -9;
pub const SQL_WLONGVARCHAR: i32 = -10;
pub const SQL_DECFLOAT: i32 = -360;
pub const SQL_TYPE_DATE: u32 = 91;
pub const SQL_TYPE_TIME: u32 = 92;
pub const SQL_TYPE_TIMESTAMP: u32 = 93;
pub const SQL_TYPE_TIMESTAMP_WITH_TIMEZONE: u32 = 95;
pub const SQL_UNSPECIFIED: u32 = 0;
pub const SQL_INSENSITIVE: u32 = 1;
pub const SQL_SENSITIVE: u32 = 2;
pub const SQL_DEFAULT: u32 = 99;
pub const SQL_ARD_TYPE: i32 = -99;
pub const SQL_CODE_DATE: u32 = 1;
pub const SQL_CODE_TIME: u32 = 2;
pub const SQL_CODE_TIMESTAMP: u32 = 3;
pub const SQL_CODE_TIMESTAMP_WITH_TIMEZONE: u32 = 4;
pub const SQL_GRAPHIC: i32 = -95;
pub const SQL_VARGRAPHIC: i32 = -96;
pub const SQL_LONGVARGRAPHIC: i32 = -97;
pub const SQL_BLOB: i32 = -98;
pub const SQL_CLOB: i32 = -99;
pub const SQL_DBCLOB: i32 = -350;
pub const SQL_XML: i32 = -370;
pub const SQL_CURSORHANDLE: i32 = -380;
pub const SQL_DATALINK: i32 = -400;
pub const SQL_USER_DEFINED_TYPE: i32 = -450;
pub const SQL_C_DBCHAR: i32 = -350;
pub const SQL_C_DECIMAL_IBM: u32 = 3;
pub const SQL_C_PTR: u32 = 2463;
pub const SQL_C_DECIMAL_OLEDB: u32 = 2514;
pub const SQL_C_DECIMAL64: i32 = -360;
pub const SQL_C_DECIMAL128: i32 = -361;
pub const SQL_C_TIMESTAMP_EXT: i32 = -362;
pub const SQL_C_TYPE_TIMESTAMP_EXT: i32 = -362;
pub const SQL_C_BINARYXML: i32 = -363;
pub const SQL_C_TIMESTAMP_EXT_TZ: i32 = -364;
pub const SQL_C_TYPE_TIMESTAMP_EXT_TZ: i32 = -364;
pub const SQL_C_CURSORHANDLE: i32 = -365;
pub const SQL_BLOB_LOCATOR: u32 = 31;
pub const SQL_CLOB_LOCATOR: u32 = 41;
pub const SQL_DBCLOB_LOCATOR: i32 = -351;
pub const SQL_C_BLOB_LOCATOR: u32 = 31;
pub const SQL_C_CLOB_LOCATOR: u32 = 41;
pub const SQL_C_DBCLOB_LOCATOR: i32 = -351;
pub const SQL_NO_NULLS: u32 = 0;
pub const SQL_NULLABLE: u32 = 1;
pub const SQL_NULLABLE_UNKNOWN: u32 = 2;
pub const SQL_NAMED: u32 = 0;
pub const SQL_UNNAMED: u32 = 1;
pub const SQL_DESC_ALLOC_AUTO: u32 = 1;
pub const SQL_DESC_ALLOC_USER: u32 = 2;
pub const SQL_TYPE_BASE: u32 = 0;
pub const SQL_TYPE_DISTINCT: u32 = 1;
pub const SQL_TYPE_STRUCTURED: u32 = 2;
pub const SQL_TYPE_REFERENCE: u32 = 3;
pub const SQL_NULL_DATA: i32 = -1;
pub const SQL_DATA_AT_EXEC: i32 = -2;
pub const SQL_NTS: i32 = -3;
pub const SQL_NTSL: i32 = -3;
pub const SQL_COLUMN_SCHEMA_NAME: u32 = 16;
pub const SQL_COLUMN_CATALOG_NAME: u32 = 17;
pub const SQL_COLUMN_DISTINCT_TYPE: u32 = 1250;
pub const SQL_DESC_DISTINCT_TYPE: u32 = 1250;
pub const SQL_COLUMN_REFERENCE_TYPE: u32 = 1251;
pub const SQL_DESC_REFERENCE_TYPE: u32 = 1251;
pub const SQL_DESC_STRUCTURED_TYPE: u32 = 1252;
pub const SQL_DESC_USER_TYPE: u32 = 1253;
pub const SQL_DESC_BASE_TYPE: u32 = 1254;
pub const SQL_DESC_KEY_TYPE: u32 = 1255;
pub const SQL_DESC_KEY_MEMBER: u32 = 1266;
pub const SQL_DESC_IDENTITY_VALUE: u32 = 1267;
pub const SQL_DESC_CODEPAGE: u32 = 1268;
pub const SQL_DESC_COUNT: u32 = 1001;
pub const SQL_DESC_TYPE: u32 = 1002;
pub const SQL_DESC_LENGTH: u32 = 1003;
pub const SQL_DESC_OCTET_LENGTH_PTR: u32 = 1004;
pub const SQL_DESC_PRECISION: u32 = 1005;
pub const SQL_DESC_SCALE: u32 = 1006;
pub const SQL_DESC_DATETIME_INTERVAL_CODE: u32 = 1007;
pub const SQL_DESC_NULLABLE: u32 = 1008;
pub const SQL_DESC_INDICATOR_PTR: u32 = 1009;
pub const SQL_DESC_DATA_PTR: u32 = 1010;
pub const SQL_DESC_NAME: u32 = 1011;
pub const SQL_DESC_UNNAMED: u32 = 1012;
pub const SQL_DESC_OCTET_LENGTH: u32 = 1013;
pub const SQL_DESC_ALLOC_TYPE: u32 = 1099;
pub const SQL_DESC_USER_DEFINED_TYPE_CODE: u32 = 1098;
pub const SQL_DESC_CARDINALITY: u32 = 1040;
pub const SQL_DESC_CARDINALITY_PTR: u32 = 1043;
pub const SQL_DESC_ROW_DESC: u32 = 1044;
pub const SQL_KEYTYPE_NONE: u32 = 0;
pub const SQL_KEYTYPE_PRIMARYKEY: u32 = 1;
pub const SQL_KEYTYPE_UNIQUEINDEX: u32 = 2;
pub const SQL_UPDT_READONLY: u32 = 0;
pub const SQL_UPDT_WRITE: u32 = 1;
pub const SQL_UPDT_READWRITE_UNKNOWN: u32 = 2;
pub const SQL_PRED_NONE: u32 = 0;
pub const SQL_PRED_CHAR: u32 = 1;
pub const SQL_PRED_BASIC: u32 = 2;
pub const SQL_NULL_HENV: u32 = 0;
pub const SQL_NULL_HDBC: u32 = 0;
pub const SQL_NULL_HSTMT: u32 = 0;
pub const SQL_NULL_HDESC: u32 = 0;
pub const SQL_NULL_HANDLE: u32 = 0;
pub const SQL_DIAG_RETURNCODE: u32 = 1;
pub const SQL_DIAG_NUMBER: u32 = 2;
pub const SQL_DIAG_ROW_COUNT: u32 = 3;
pub const SQL_DIAG_SQLSTATE: u32 = 4;
pub const SQL_DIAG_NATIVE: u32 = 5;
pub const SQL_DIAG_MESSAGE_TEXT: u32 = 6;
pub const SQL_DIAG_DYNAMIC_FUNCTION: u32 = 7;
pub const SQL_DIAG_CLASS_ORIGIN: u32 = 8;
pub const SQL_DIAG_SUBCLASS_ORIGIN: u32 = 9;
pub const SQL_DIAG_CONNECTION_NAME: u32 = 10;
pub const SQL_DIAG_SERVER_NAME: u32 = 11;
pub const SQL_DIAG_DYNAMIC_FUNCTION_CODE: u32 = 12;
pub const SQL_DIAG_ISAM_ERROR: u32 = 13;
pub const SQL_DIAG_SYSPLEX_STATISTICS: u32 = 2528;
pub const SQL_DIAG_DB2ZLOAD_RETCODE: u32 = 2529;
pub const SQL_DIAG_DB2ZLOAD_LOAD_MSGS: u32 = 2530;
pub const SQL_DIAG_LOG_FILENAME: u32 = 2531;
pub const SQL_DIAG_BAD_FILENAME: u32 = 2532;
pub const SQL_DIAG_ALTER_TABLE: u32 = 4;
pub const SQL_DIAG_CALL: u32 = 7;
pub const SQL_DIAG_CREATE_INDEX: i32 = -1;
pub const SQL_DIAG_CREATE_TABLE: u32 = 77;
pub const SQL_DIAG_CREATE_VIEW: u32 = 84;
pub const SQL_DIAG_DELETE_WHERE: u32 = 19;
pub const SQL_DIAG_DROP_INDEX: i32 = -2;
pub const SQL_DIAG_DROP_TABLE: u32 = 32;
pub const SQL_DIAG_DROP_VIEW: u32 = 36;
pub const SQL_DIAG_DYNAMIC_DELETE_CURSOR: u32 = 38;
pub const SQL_DIAG_DYNAMIC_UPDATE_CURSOR: u32 = 81;
pub const SQL_DIAG_GRANT: u32 = 48;
pub const SQL_DIAG_INSERT: u32 = 50;
pub const SQL_DIAG_MERGE: u32 = 128;
pub const SQL_DIAG_REVOKE: u32 = 59;
pub const SQL_DIAG_SELECT_CURSOR: u32 = 85;
pub const SQL_DIAG_UNKNOWN_STATEMENT: u32 = 0;
pub const SQL_DIAG_UPDATE_WHERE: u32 = 82;
pub const SQL_DIAG_DEFERRED_PREPARE_ERROR: u32 = 1279;
pub const SQL_ROW_NO_ROW_NUMBER: i32 = -1;
pub const SQL_ROW_NUMBER_UNKNOWN: i32 = -2;
pub const SQL_COLUMN_NO_COLUMN_NUMBER: i32 = -1;
pub const SQL_COLUMN_NUMBER_UNKNOWN: i32 = -2;
pub const SQL_MAX_C_NUMERIC_PRECISION: u32 = 38;
pub const SQL_MAX_NUMERIC_LEN: u32 = 16;
pub const SQL_DECIMAL64_LEN: u32 = 8;
pub const SQL_DECIMAL128_LEN: u32 = 16;
pub const ODBCVER: u32 = 896;
pub const SQL_SPEC_MAJOR: u32 = 3;
pub const SQL_SPEC_MINOR: u32 = 80;
pub const SQL_SPEC_STRING: &'static [u8; 6usize] = b"03.80\0";
pub const SQL_SQLSTATE_SIZE: u32 = 5;
pub const SQL_MAX_DSN_LENGTH: u32 = 32;
pub const SQL_MAX_OPTION_STRING_LENGTH: u32 = 256;
pub const SQL_NO_DATA_FOUND: u32 = 100;
pub const SQL_HANDLE_SENV: u32 = 5;
pub const SQL_ATTR_ODBC_VERSION: u32 = 200;
pub const SQL_ATTR_CONNECTION_POOLING: u32 = 201;
pub const SQL_ATTR_CP_MATCH: u32 = 202;
pub const SQL_CP_OFF: u32 = 0;
pub const SQL_CP_ONE_PER_DRIVER: u32 = 1;
pub const SQL_CP_ONE_PER_HENV: u32 = 2;
pub const SQL_CP_DEFAULT: u32 = 0;
pub const SQL_CP_STRICT_MATCH: u32 = 0;
pub const SQL_CP_RELAXED_MATCH: u32 = 1;
pub const SQL_CP_MATCH_DEFAULT: u32 = 0;
pub const SQL_OV_ODBC2: u32 = 2;
pub const SQL_OV_ODBC3: u32 = 3;
pub const SQL_OV_ODBC3_80: u32 = 380;
pub const SQL_ACCESS_MODE: u32 = 101;
pub const SQL_AUTOCOMMIT: u32 = 102;
pub const SQL_LOGIN_TIMEOUT: u32 = 103;
pub const SQL_OPT_TRACE: u32 = 104;
pub const SQL_OPT_TRACEFILE: u32 = 105;
pub const SQL_TRANSLATE_DLL: u32 = 106;
pub const SQL_TRANSLATE_OPTION: u32 = 107;
pub const SQL_TXN_ISOLATION: u32 = 108;
pub const SQL_CURRENT_QUALIFIER: u32 = 109;
pub const SQL_ODBC_CURSORS: u32 = 110;
pub const SQL_QUIET_MODE: u32 = 111;
pub const SQL_PACKET_SIZE: u32 = 112;
pub const SQL_ATTR_ACCESS_MODE: u32 = 101;
pub const SQL_ATTR_AUTOCOMMIT: u32 = 102;
pub const SQL_ATTR_CONNECTION_TIMEOUT: u32 = 113;
pub const SQL_ATTR_CURRENT_CATALOG: u32 = 109;
pub const SQL_ATTR_DISCONNECT_BEHAVIOR: u32 = 114;
pub const SQL_ATTR_ENLIST_IN_DTC: u32 = 1207;
pub const SQL_ATTR_ENLIST_IN_XA: u32 = 1208;
pub const SQL_ATTR_LOGIN_TIMEOUT: u32 = 103;
pub const SQL_ATTR_ODBC_CURSORS: u32 = 110;
pub const SQL_ATTR_PACKET_SIZE: u32 = 112;
pub const SQL_ATTR_QUIET_MODE: u32 = 111;
pub const SQL_ATTR_TRACE: u32 = 104;
pub const SQL_ATTR_TRACEFILE: u32 = 105;
pub const SQL_ATTR_TRANSLATE_LIB: u32 = 106;
pub const SQL_ATTR_TRANSLATE_OPTION: u32 = 107;
pub const SQL_ATTR_TXN_ISOLATION: u32 = 108;
pub const SQL_ATTR_CONNECTION_DEAD: u32 = 1209;
pub const SQL_ATTR_ANSI_APP: u32 = 115;
pub const SQL_ATTR_RESET_CONNECTION: u32 = 116;
pub const SQL_ATTR_ASYNC_DBC_FUNCTIONS_ENABLE: u32 = 117;
pub const SQL_MODE_READ_WRITE: u32 = 0;
pub const SQL_MODE_READ_ONLY: u32 = 1;
pub const SQL_MODE_DEFAULT: u32 = 0;
pub const SQL_AUTOCOMMIT_OFF: u32 = 0;
pub const SQL_AUTOCOMMIT_ON: u32 = 1;
pub const SQL_AUTOCOMMIT_DEFERRED: u32 = 2;
pub const SQL_AUTOCOMMIT_DEFAULT: u32 = 1;
pub const SQL_LOGIN_TIMEOUT_DEFAULT: u32 = 15;
pub const SQL_OPT_TRACE_OFF: u32 = 0;
pub const SQL_OPT_TRACE_ON: u32 = 1;
pub const SQL_OPT_TRACE_DEFAULT: u32 = 0;
pub const SQL_OPT_TRACE_FILE_DEFAULT: &'static [u8; 9usize] = b"\\SQL.LOG\0";
pub const SQL_CUR_USE_IF_NEEDED: u32 = 0;
pub const SQL_CUR_USE_ODBC: u32 = 1;
pub const SQL_CUR_USE_DRIVER: u32 = 2;
pub const SQL_CUR_DEFAULT: u32 = 2;
pub const SQL_DB_RETURN_TO_POOL: u32 = 0;
pub const SQL_DB_DISCONNECT: u32 = 1;
pub const SQL_DB_DEFAULT: u32 = 0;
pub const SQL_DTC_DONE: u32 = 0;
pub const SQL_CD_TRUE: u32 = 1;
pub const SQL_CD_FALSE: u32 = 0;
pub const SQL_AA_TRUE: u32 = 1;
pub const SQL_AA_FALSE: u32 = 0;
pub const SQL_RESET_CONNECTION_YES: u32 = 1;
pub const SQL_ASYNC_DBC_ENABLE_ON: u32 = 1;
pub const SQL_ASYNC_DBC_ENABLE_OFF: u32 = 0;
pub const SQL_ASYNC_DBC_ENABLE_DEFAULT: u32 = 0;
pub const SQL_QUERY_TIMEOUT: u32 = 0;
pub const SQL_MAX_ROWS: u32 = 1;
pub const SQL_NOSCAN: u32 = 2;
pub const SQL_MAX_LENGTH: u32 = 3;
pub const SQL_ASYNC_ENABLE: u32 = 4;
pub const SQL_BIND_TYPE: u32 = 5;
pub const SQL_CURSOR_TYPE: u32 = 6;
pub const SQL_CONCURRENCY: u32 = 7;
pub const SQL_KEYSET_SIZE: u32 = 8;
pub const SQL_ROWSET_SIZE: u32 = 9;
pub const SQL_SIMULATE_CURSOR: u32 = 10;
pub const SQL_RETRIEVE_DATA: u32 = 11;
pub const SQL_USE_BOOKMARKS: u32 = 12;
pub const SQL_GET_BOOKMARK: u32 = 13;
pub const SQL_ROW_NUMBER: u32 = 14;
pub const SQL_ATTR_ASYNC_ENABLE: u32 = 4;
pub const SQL_ATTR_CONCURRENCY: u32 = 7;
pub const SQL_ATTR_CURSOR_TYPE: u32 = 6;
pub const SQL_ATTR_ENABLE_AUTO_IPD: u32 = 15;
pub const SQL_ATTR_FETCH_BOOKMARK_PTR: u32 = 16;
pub const SQL_ATTR_KEYSET_SIZE: u32 = 8;
pub const SQL_ATTR_MAX_LENGTH: u32 = 3;
pub const SQL_ATTR_MAX_ROWS: u32 = 1;
pub const SQL_ATTR_NOSCAN: u32 = 2;
pub const SQL_ATTR_PARAM_BIND_OFFSET_PTR: u32 = 17;
pub const SQL_ATTR_PARAM_BIND_TYPE: u32 = 18;
pub const SQL_ATTR_PARAM_OPERATION_PTR: u32 = 19;
pub const SQL_ATTR_PARAM_STATUS_PTR: u32 = 20;
pub const SQL_ATTR_PARAMS_PROCESSED_PTR: u32 = 21;
pub const SQL_ATTR_PARAMSET_SIZE: u32 = 22;
pub const SQL_ATTR_QUERY_TIMEOUT: u32 = 0;
pub const SQL_ATTR_RETRIEVE_DATA: u32 = 11;
pub const SQL_ATTR_ROW_BIND_OFFSET_PTR: u32 = 23;
pub const SQL_ATTR_ROW_BIND_TYPE: u32 = 5;
pub const SQL_ATTR_ROW_NUMBER: u32 = 14;
pub const SQL_ATTR_ROW_OPERATION_PTR: u32 = 24;
pub const SQL_ATTR_ROW_STATUS_PTR: u32 = 25;
pub const SQL_ATTR_ROWS_FETCHED_PTR: u32 = 26;
pub const SQL_ATTR_ROW_ARRAY_SIZE: u32 = 27;
pub const SQL_ATTR_SIMULATE_CURSOR: u32 = 10;
pub const SQL_ATTR_USE_BOOKMARKS: u32 = 12;
pub const SQL_IS_POINTER: i32 = -4;
pub const SQL_IS_UINTEGER: i32 = -5;
pub const SQL_IS_INTEGER: i32 = -6;
pub const SQL_IS_USMALLINT: i32 = -7;
pub const SQL_IS_SMALLINT: i32 = -8;
pub const SQL_PARAM_BIND_BY_COLUMN: u32 = 0;
pub const SQL_PARAM_BIND_TYPE_DEFAULT: u32 = 0;
pub const SQL_QUERY_TIMEOUT_DEFAULT: u32 = 0;
pub const SQL_MAX_ROWS_DEFAULT: u32 = 0;
pub const SQL_NOSCAN_OFF: u32 = 0;
pub const SQL_NOSCAN_ON: u32 = 1;
pub const SQL_NOSCAN_DEFAULT: u32 = 0;
pub const SQL_MAX_LENGTH_DEFAULT: u32 = 0;
pub const SQL_ASYNC_ENABLE_OFF: u32 = 0;
pub const SQL_ASYNC_ENABLE_ON: u32 = 1;
pub const SQL_ASYNC_ENABLE_DEFAULT: u32 = 0;
pub const SQL_BIND_BY_COLUMN: u32 = 0;
pub const SQL_BIND_TYPE_DEFAULT: u32 = 0;
pub const SQL_CONCUR_READ_ONLY: u32 = 1;
pub const SQL_CONCUR_LOCK: u32 = 2;
pub const SQL_CONCUR_ROWVER: u32 = 3;
pub const SQL_CONCUR_VALUES: u32 = 4;
pub const SQL_CONCUR_DEFAULT: u32 = 1;
pub const SQL_CURSOR_FORWARD_ONLY: u32 = 0;
pub const SQL_CURSOR_KEYSET_DRIVEN: u32 = 1;
pub const SQL_CURSOR_DYNAMIC: u32 = 2;
pub const SQL_CURSOR_STATIC: u32 = 3;
pub const SQL_CURSOR_TYPE_DEFAULT: u32 = 0;
pub const SQL_ROWSET_SIZE_DEFAULT: u32 = 1;
pub const SQL_KEYSET_SIZE_DEFAULT: u32 = 0;
pub const SQL_SC_NON_UNIQUE: u32 = 0;
pub const SQL_SC_TRY_UNIQUE: u32 = 1;
pub const SQL_SC_UNIQUE: u32 = 2;
pub const SQL_RD_OFF: u32 = 0;
pub const SQL_RD_ON: u32 = 1;
pub const SQL_RD_DEFAULT: u32 = 1;
pub const SQL_UB_OFF: u32 = 0;
pub const SQL_UB_ON: u32 = 1;
pub const SQL_UB_DEFAULT: u32 = 0;
pub const SQL_UB_FIXED: u32 = 1;
pub const SQL_UB_VARIABLE: u32 = 2;
pub const SQL_DESC_ARRAY_SIZE: u32 = 20;
pub const SQL_DESC_ARRAY_STATUS_PTR: u32 = 21;
pub const SQL_DESC_BASE_COLUMN_NAME: u32 = 22;
pub const SQL_DESC_BASE_TABLE_NAME: u32 = 23;
pub const SQL_DESC_BIND_OFFSET_PTR: u32 = 24;
pub const SQL_DESC_BIND_TYPE: u32 = 25;
pub const SQL_DESC_DATETIME_INTERVAL_PRECISION: u32 = 26;
pub const SQL_DESC_LITERAL_PREFIX: u32 = 27;
pub const SQL_DESC_LITERAL_SUFFIX: u32 = 28;
pub const SQL_DESC_LOCAL_TYPE_NAME: u32 = 29;
pub const SQL_DESC_MAXIMUM_SCALE: u32 = 30;
pub const SQL_DESC_MINIMUM_SCALE: u32 = 31;
pub const SQL_DESC_NUM_PREC_RADIX: u32 = 32;
pub const SQL_DESC_PARAMETER_TYPE: u32 = 33;
pub const SQL_DESC_ROWS_PROCESSED_PTR: u32 = 34;
pub const SQL_DESC_ROWVER: u32 = 35;
pub const SQL_DIAG_CURSOR_ROW_COUNT: i32 = -1249;
pub const SQL_DIAG_ROW_NUMBER: i32 = -1248;
pub const SQL_DIAG_COLUMN_NUMBER: i32 = -1247;
pub const SQL_DATE: u32 = 9;
pub const SQL_INTERVAL: u32 = 10;
pub const SQL_TIME: u32 = 10;
pub const SQL_TIMESTAMP: u32 = 11;
pub const SQL_LONGVARCHAR: i32 = -1;
pub const SQL_BINARY: i32 = -2;
pub const SQL_VARBINARY: i32 = -3;
pub const SQL_LONGVARBINARY: i32 = -4;
pub const SQL_BIGINT: i32 = -5;
pub const SQL_TINYINT: i32 = -6;
pub const SQL_BIT: i32 = -7;
pub const SQL_GUID: i32 = -11;
pub const SQL_CODE_YEAR: u32 = 1;
pub const SQL_CODE_MONTH: u32 = 2;
pub const SQL_CODE_DAY: u32 = 3;
pub const SQL_CODE_HOUR: u32 = 4;
pub const SQL_CODE_MINUTE: u32 = 5;
pub const SQL_CODE_SECOND: u32 = 6;
pub const SQL_CODE_YEAR_TO_MONTH: u32 = 7;
pub const SQL_CODE_DAY_TO_HOUR: u32 = 8;
pub const SQL_CODE_DAY_TO_MINUTE: u32 = 9;
pub const SQL_CODE_DAY_TO_SECOND: u32 = 10;
pub const SQL_CODE_HOUR_TO_MINUTE: u32 = 11;
pub const SQL_CODE_HOUR_TO_SECOND: u32 = 12;
pub const SQL_CODE_MINUTE_TO_SECOND: u32 = 13;
pub const SQL_INTERVAL_YEAR: u32 = 101;
pub const SQL_INTERVAL_MONTH: u32 = 102;
pub const SQL_INTERVAL_DAY: u32 = 103;
pub const SQL_INTERVAL_HOUR: u32 = 104;
pub const SQL_INTERVAL_MINUTE: u32 = 105;
pub const SQL_INTERVAL_SECOND: u32 = 106;
pub const SQL_INTERVAL_YEAR_TO_MONTH: u32 = 107;
pub const SQL_INTERVAL_DAY_TO_HOUR: u32 = 108;
pub const SQL_INTERVAL_DAY_TO_MINUTE: u32 = 109;
pub const SQL_INTERVAL_DAY_TO_SECOND: u32 = 110;
pub const SQL_INTERVAL_HOUR_TO_MINUTE: u32 = 111;
pub const SQL_INTERVAL_HOUR_TO_SECOND: u32 = 112;
pub const SQL_INTERVAL_MINUTE_TO_SECOND: u32 = 113;
pub const SQL_UNICODE: i32 = -8;
pub const SQL_UNICODE_VARCHAR: i32 = -9;
pub const SQL_UNICODE_LONGVARCHAR: i32 = -10;
pub const SQL_UNICODE_CHAR: i32 = -8;
pub const SQL_C_CHAR: u32 = 1;
pub const SQL_C_LONG: u32 = 4;
pub const SQL_C_SHORT: u32 = 5;
pub const SQL_C_FLOAT: u32 = 7;
pub const SQL_C_DOUBLE: u32 = 8;
pub const SQL_C_NUMERIC: u32 = 2;
pub const SQL_C_DEFAULT: u32 = 99;
pub const SQL_SIGNED_OFFSET: i32 = -20;
pub const SQL_UNSIGNED_OFFSET: i32 = -22;
pub const SQL_C_DATE: u32 = 9;
pub const SQL_C_TIME: u32 = 10;
pub const SQL_C_TIMESTAMP: u32 = 11;
pub const SQL_C_TYPE_DATE: u32 = 91;
pub const SQL_C_TYPE_TIME: u32 = 92;
pub const SQL_C_TYPE_TIMESTAMP: u32 = 93;
pub const SQL_C_INTERVAL_YEAR: u32 = 101;
pub const SQL_C_INTERVAL_MONTH: u32 = 102;
pub const SQL_C_INTERVAL_DAY: u32 = 103;
pub const SQL_C_INTERVAL_HOUR: u32 = 104;
pub const SQL_C_INTERVAL_MINUTE: u32 = 105;
pub const SQL_C_INTERVAL_SECOND: u32 = 106;
pub const SQL_C_INTERVAL_YEAR_TO_MONTH: u32 = 107;
pub const SQL_C_INTERVAL_DAY_TO_HOUR: u32 = 108;
pub const SQL_C_INTERVAL_DAY_TO_MINUTE: u32 = 109;
pub const SQL_C_INTERVAL_DAY_TO_SECOND: u32 = 110;
pub const SQL_C_INTERVAL_HOUR_TO_MINUTE: u32 = 111;
pub const SQL_C_INTERVAL_HOUR_TO_SECOND: u32 = 112;
pub const SQL_C_INTERVAL_MINUTE_TO_SECOND: u32 = 113;
pub const SQL_C_BINARY: i32 = -2;
pub const SQL_C_BIT: i32 = -7;
pub const SQL_C_SBIGINT: i32 = -25;
pub const SQL_C_UBIGINT: i32 = -27;
pub const SQL_C_TINYINT: i32 = -6;
pub const SQL_C_SLONG: i32 = -16;
pub const SQL_C_SSHORT: i32 = -15;
pub const SQL_C_STINYINT: i32 = -26;
pub const SQL_C_ULONG: i32 = -18;
pub const SQL_C_USHORT: i32 = -17;
pub const SQL_C_UTINYINT: i32 = -28;
pub const SQL_C_BOOKMARK: i32 = -18;
pub const SQL_C_GUID: i32 = -11;
pub const SQL_TYPE_NULL: u32 = 0;
pub const SQL_DRIVER_C_TYPE_BASE: u32 = 16384;
pub const SQL_DRIVER_SQL_TYPE_BASE: u32 = 16384;
pub const SQL_DRIVER_DESC_FIELD_BASE: u32 = 16384;
pub const SQL_DRIVER_DIAG_FIELD_BASE: u32 = 16384;
pub const SQL_DRIVER_INFO_TYPE_BASE: u32 = 16384;
pub const SQL_DRIVER_CONN_ATTR_BASE: u32 = 16384;
pub const SQL_DRIVER_STMT_ATTR_BASE: u32 = 16384;
pub const SQL_C_VARBOOKMARK: i32 = -2;
pub const SQL_NO_ROW_NUMBER: i32 = -1;
pub const SQL_NO_COLUMN_NUMBER: i32 = -1;
pub const SQL_DEFAULT_PARAM: i32 = -5;
pub const SQL_IGNORE: i32 = -6;
pub const SQL_COLUMN_IGNORE: i32 = -6;
pub const SQL_LEN_DATA_AT_EXEC_OFFSET: i32 = -100;
pub const SQL_LEN_BINARY_ATTR_OFFSET: i32 = -100;
pub const SQL_SETPARAM_VALUE_MAX: i32 = -1;
pub const SQL_COLUMN_COUNT: u32 = 0;
pub const SQL_COLUMN_NAME: u32 = 1;
pub const SQL_COLUMN_TYPE: u32 = 2;
pub const SQL_COLUMN_LENGTH: u32 = 3;
pub const SQL_COLUMN_PRECISION: u32 = 4;
pub const SQL_COLUMN_SCALE: u32 = 5;
pub const SQL_COLUMN_DISPLAY_SIZE: u32 = 6;
pub const SQL_COLUMN_NULLABLE: u32 = 7;
pub const SQL_COLUMN_UNSIGNED: u32 = 8;
pub const SQL_COLUMN_MONEY: u32 = 9;
pub const SQL_COLUMN_UPDATABLE: u32 = 10;
pub const SQL_COLUMN_AUTO_INCREMENT: u32 = 11;
pub const SQL_COLUMN_CASE_SENSITIVE: u32 = 12;
pub const SQL_COLUMN_SEARCHABLE: u32 = 13;
pub const SQL_COLUMN_TYPE_NAME: u32 = 14;
pub const SQL_COLUMN_TABLE_NAME: u32 = 15;
pub const SQL_COLUMN_OWNER_NAME: u32 = 16;
pub const SQL_COLUMN_QUALIFIER_NAME: u32 = 17;
pub const SQL_COLUMN_LABEL: u32 = 18;
pub const SQL_COLATT_OPT_MAX: u32 = 18;
pub const SQL_COLATT_OPT_MIN: u32 = 0;
pub const SQL_ATTR_READONLY: u32 = 0;
pub const SQL_ATTR_WRITE: u32 = 1;
pub const SQL_ATTR_READWRITE_UNKNOWN: u32 = 2;
pub const SQL_UNSEARCHABLE: u32 = 0;
pub const SQL_LIKE_ONLY: u32 = 1;
pub const SQL_ALL_EXCEPT_LIKE: u32 = 2;
pub const SQL_SEARCHABLE: u32 = 3;
pub const SQL_PRED_SEARCHABLE: u32 = 3;
pub const SQL_NO_TOTAL: i32 = -4;
pub const SQL_API_SQLALLOCHANDLESTD: u32 = 73;
pub const SQL_API_SQLBULKOPERATIONS: u32 = 24;
pub const SQL_API_SQLBINDPARAMETER: u32 = 72;
pub const SQL_API_SQLBROWSECONNECT: u32 = 55;
pub const SQL_API_SQLCOLATTRIBUTES: u32 = 6;
pub const SQL_API_SQLCOLUMNPRIVILEGES: u32 = 56;
pub const SQL_API_SQLDESCRIBEPARAM: u32 = 58;
pub const SQL_API_SQLDRIVERCONNECT: u32 = 41;
pub const SQL_API_SQLDRIVERS: u32 = 71;
pub const SQL_API_SQLEXTENDEDFETCH: u32 = 59;
pub const SQL_API_SQLFOREIGNKEYS: u32 = 60;
pub const SQL_API_SQLMORERESULTS: u32 = 61;
pub const SQL_API_SQLNATIVESQL: u32 = 62;
pub const SQL_API_SQLNUMPARAMS: u32 = 63;
pub const SQL_API_SQLPARAMOPTIONS: u32 = 64;
pub const SQL_API_SQLPRIMARYKEYS: u32 = 65;
pub const SQL_API_SQLPROCEDURECOLUMNS: u32 = 66;
pub const SQL_API_SQLPROCEDURES: u32 = 67;
pub const SQL_API_SQLSETPOS: u32 = 68;
pub const SQL_API_SQLSETSCROLLOPTIONS: u32 = 69;
pub const SQL_API_SQLTABLEPRIVILEGES: u32 = 70;
pub const SQL_API_ALL_FUNCTIONS: u32 = 0;
pub const SQL_API_LOADBYORDINAL: u32 = 199;
pub const SQL_API_ODBC3_ALL_FUNCTIONS: u32 = 999;
pub const SQL_API_ODBC3_ALL_FUNCTIONS_SIZE: u32 = 250;
pub const SQL_INFO_FIRST: u32 = 0;
pub const SQL_ACTIVE_CONNECTIONS: u32 = 0;
pub const SQL_ACTIVE_STATEMENTS: u32 = 1;
pub const SQL_DRIVER_HDBC: u32 = 3;
pub const SQL_DRIVER_HENV: u32 = 4;
pub const SQL_DRIVER_HSTMT: u32 = 5;
pub const SQL_DRIVER_NAME: u32 = 6;
pub const SQL_DRIVER_VER: u32 = 7;
pub const SQL_ODBC_API_CONFORMANCE: u32 = 9;
pub const SQL_ODBC_VER: u32 = 10;
pub const SQL_ROW_UPDATES: u32 = 11;
pub const SQL_ODBC_SAG_CLI_CONFORMANCE: u32 = 12;
pub const SQL_ODBC_SQL_CONFORMANCE: u32 = 15;
pub const SQL_PROCEDURES: u32 = 21;
pub const SQL_CONCAT_NULL_BEHAVIOR: u32 = 22;
pub const SQL_CURSOR_ROLLBACK_BEHAVIOR: u32 = 24;
pub const SQL_EXPRESSIONS_IN_ORDERBY: u32 = 27;
pub const SQL_MAX_OWNER_NAME_LEN: u32 = 32;
pub const SQL_MAX_PROCEDURE_NAME_LEN: u32 = 33;
pub const SQL_MAX_QUALIFIER_NAME_LEN: u32 = 34;
pub const SQL_MULT_RESULT_SETS: u32 = 36;
pub const SQL_MULTIPLE_ACTIVE_TXN: u32 = 37;
pub const SQL_OUTER_JOINS: u32 = 38;
pub const SQL_OWNER_TERM: u32 = 39;
pub const SQL_PROCEDURE_TERM: u32 = 40;
pub const SQL_QUALIFIER_NAME_SEPARATOR: u32 = 41;
pub const SQL_QUALIFIER_TERM: u32 = 42;
pub const SQL_SCROLL_OPTIONS: u32 = 44;
pub const SQL_TABLE_TERM: u32 = 45;
pub const SQL_CONVERT_FUNCTIONS: u32 = 48;
pub const SQL_NUMERIC_FUNCTIONS: u32 = 49;
pub const SQL_STRING_FUNCTIONS: u32 = 50;
pub const SQL_SYSTEM_FUNCTIONS: u32 = 51;
pub const SQL_TIMEDATE_FUNCTIONS: u32 = 52;
pub const SQL_CONVERT_BIGINT: u32 = 53;
pub const SQL_CONVERT_BINARY: u32 = 54;
pub const SQL_CONVERT_BIT: u32 = 55;
pub const SQL_CONVERT_CHAR: u32 = 56;
pub const SQL_CONVERT_DATE: u32 = 57;
pub const SQL_CONVERT_DECIMAL: u32 = 58;
pub const SQL_CONVERT_DOUBLE: u32 = 59;
pub const SQL_CONVERT_FLOAT: u32 = 60;
pub const SQL_CONVERT_INTEGER: u32 = 61;
pub const SQL_CONVERT_LONGVARCHAR: u32 = 62;
pub const SQL_CONVERT_NUMERIC: u32 = 63;
pub const SQL_CONVERT_REAL: u32 = 64;
pub const SQL_CONVERT_SMALLINT: u32 = 65;
pub const SQL_CONVERT_TIME: u32 = 66;
pub const SQL_CONVERT_TIMESTAMP: u32 = 67;
pub const SQL_CONVERT_TINYINT: u32 = 68;
pub const SQL_CONVERT_VARBINARY: u32 = 69;
pub const SQL_CONVERT_VARCHAR: u32 = 70;
pub const SQL_CONVERT_LONGVARBINARY: u32 = 71;
pub const SQL_ODBC_SQL_OPT_IEF: u32 = 73;
pub const SQL_CORRELATION_NAME: u32 = 74;
pub const SQL_NON_NULLABLE_COLUMNS: u32 = 75;
pub const SQL_DRIVER_HLIB: u32 = 76;
pub const SQL_DRIVER_ODBC_VER: u32 = 77;
pub const SQL_LOCK_TYPES: u32 = 78;
pub const SQL_POS_OPERATIONS: u32 = 79;
pub const SQL_POSITIONED_STATEMENTS: u32 = 80;
pub const SQL_BOOKMARK_PERSISTENCE: u32 = 82;
pub const SQL_STATIC_SENSITIVITY: u32 = 83;
pub const SQL_FILE_USAGE: u32 = 84;
pub const SQL_COLUMN_ALIAS: u32 = 87;
pub const SQL_GROUP_BY: u32 = 88;
pub const SQL_KEYWORDS: u32 = 89;
pub const SQL_OWNER_USAGE: u32 = 91;
pub const SQL_QUALIFIER_USAGE: u32 = 92;
pub const SQL_QUOTED_IDENTIFIER_CASE: u32 = 93;
pub const SQL_SUBQUERIES: u32 = 95;
pub const SQL_UNION: u32 = 96;
pub const SQL_MAX_ROW_SIZE_INCLUDES_LONG: u32 = 103;
pub const SQL_MAX_CHAR_LITERAL_LEN: u32 = 108;
pub const SQL_TIMEDATE_ADD_INTERVALS: u32 = 109;
pub const SQL_TIMEDATE_DIFF_INTERVALS: u32 = 110;
pub const SQL_NEED_LONG_DATA_LEN: u32 = 111;
pub const SQL_MAX_BINARY_LITERAL_LEN: u32 = 112;
pub const SQL_LIKE_ESCAPE_CLAUSE: u32 = 113;
pub const SQL_QUALIFIER_LOCATION: u32 = 114;
pub const SQL_ACTIVE_ENVIRONMENTS: u32 = 116;
pub const SQL_ALTER_DOMAIN: u32 = 117;
pub const SQL_SQL_CONFORMANCE: u32 = 118;
pub const SQL_DATETIME_LITERALS: u32 = 119;
pub const SQL_ASYNC_MODE: u32 = 10021;
pub const SQL_BATCH_ROW_COUNT: u32 = 120;
pub const SQL_BATCH_SUPPORT: u32 = 121;
pub const SQL_CATALOG_LOCATION: u32 = 114;
pub const SQL_CATALOG_NAME_SEPARATOR: u32 = 41;
pub const SQL_CATALOG_TERM: u32 = 42;
pub const SQL_CATALOG_USAGE: u32 = 92;
pub const SQL_CONVERT_WCHAR: u32 = 122;
pub const SQL_CONVERT_INTERVAL_DAY_TIME: u32 = 123;
pub const SQL_CONVERT_INTERVAL_YEAR_MONTH: u32 = 124;
pub const SQL_CONVERT_WLONGVARCHAR: u32 = 125;
pub const SQL_CONVERT_WVARCHAR: u32 = 126;
pub const SQL_CREATE_ASSERTION: u32 = 127;
pub const SQL_CREATE_CHARACTER_SET: u32 = 128;
pub const SQL_CREATE_COLLATION: u32 = 129;
pub const SQL_CREATE_DOMAIN: u32 = 130;
pub const SQL_CREATE_SCHEMA: u32 = 131;
pub const SQL_CREATE_TABLE: u32 = 132;
pub const SQL_CREATE_TRANSLATION: u32 = 133;
pub const SQL_CREATE_VIEW: u32 = 134;
pub const SQL_DRIVER_HDESC: u32 = 135;
pub const SQL_DROP_ASSERTION: u32 = 136;
pub const SQL_DROP_CHARACTER_SET: u32 = 137;
pub const SQL_DROP_COLLATION: u32 = 138;
pub const SQL_DROP_DOMAIN: u32 = 139;
pub const SQL_DROP_SCHEMA: u32 = 140;
pub const SQL_DROP_TABLE: u32 = 141;
pub const SQL_DROP_TRANSLATION: u32 = 142;
pub const SQL_DROP_VIEW: u32 = 143;
pub const SQL_DYNAMIC_CURSOR_ATTRIBUTES1: u32 = 144;
pub const SQL_DYNAMIC_CURSOR_ATTRIBUTES2: u32 = 145;
pub const SQL_FORWARD_ONLY_CURSOR_ATTRIBUTES1: u32 = 146;
pub const SQL_FORWARD_ONLY_CURSOR_ATTRIBUTES2: u32 = 147;
pub const SQL_INDEX_KEYWORDS: u32 = 148;
pub const SQL_INFO_SCHEMA_VIEWS: u32 = 149;
pub const SQL_KEYSET_CURSOR_ATTRIBUTES1: u32 = 150;
pub const SQL_KEYSET_CURSOR_ATTRIBUTES2: u32 = 151;
pub const SQL_MAX_ASYNC_CONCURRENT_STATEMENTS: u32 = 10022;
pub const SQL_ODBC_INTERFACE_CONFORMANCE: u32 = 152;
pub const SQL_PARAM_ARRAY_ROW_COUNTS: u32 = 153;
pub const SQL_PARAM_ARRAY_SELECTS: u32 = 154;
pub const SQL_SCHEMA_TERM: u32 = 39;
pub const SQL_SCHEMA_USAGE: u32 = 91;
pub const SQL_SQL92_DATETIME_FUNCTIONS: u32 = 155;
pub const SQL_SQL92_FOREIGN_KEY_DELETE_RULE: u32 = 156;
pub const SQL_SQL92_FOREIGN_KEY_UPDATE_RULE: u32 = 157;
pub const SQL_SQL92_GRANT: u32 = 158;
pub const SQL_SQL92_NUMERIC_VALUE_FUNCTIONS: u32 = 159;
pub const SQL_SQL92_PREDICATES: u32 = 160;
pub const SQL_SQL92_RELATIONAL_JOIN_OPERATORS: u32 = 161;
pub const SQL_SQL92_REVOKE: u32 = 162;
pub const SQL_SQL92_ROW_VALUE_CONSTRUCTOR: u32 = 163;
pub const SQL_SQL92_STRING_FUNCTIONS: u32 = 164;
pub const SQL_SQL92_VALUE_EXPRESSIONS: u32 = 165;
pub const SQL_STANDARD_CLI_CONFORMANCE: u32 = 166;
pub const SQL_STATIC_CURSOR_ATTRIBUTES1: u32 = 167;
pub const SQL_STATIC_CURSOR_ATTRIBUTES2: u32 = 168;
pub const SQL_AGGREGATE_FUNCTIONS: u32 = 169;
pub const SQL_DDL_INDEX: u32 = 170;
pub const SQL_DM_VER: u32 = 171;
pub const SQL_INSERT_STATEMENT: u32 = 172;
pub const SQL_CONVERT_GUID: u32 = 173;
pub const SQL_UNION_STATEMENT: u32 = 96;
pub const SQL_ASYNC_DBC_FUNCTIONS: u32 = 10023;
pub const SQL_DTC_TRANSITION_COST: u32 = 1750;
pub const SQL_AT_ADD_COLUMN_SINGLE: u32 = 32;
pub const SQL_AT_ADD_COLUMN_DEFAULT: u32 = 64;
pub const SQL_AT_ADD_COLUMN_COLLATION: u32 = 128;
pub const SQL_AT_SET_COLUMN_DEFAULT: u32 = 256;
pub const SQL_AT_DROP_COLUMN_DEFAULT: u32 = 512;
pub const SQL_AT_DROP_COLUMN_CASCADE: u32 = 1024;
pub const SQL_AT_DROP_COLUMN_RESTRICT: u32 = 2048;
pub const SQL_AT_ADD_TABLE_CONSTRAINT: u32 = 4096;
pub const SQL_AT_DROP_TABLE_CONSTRAINT_CASCADE: u32 = 8192;
pub const SQL_AT_DROP_TABLE_CONSTRAINT_RESTRICT: u32 = 16384;
pub const SQL_AT_CONSTRAINT_NAME_DEFINITION: u32 = 32768;
pub const SQL_AT_CONSTRAINT_INITIALLY_DEFERRED: u32 = 65536;
pub const SQL_AT_CONSTRAINT_INITIALLY_IMMEDIATE: u32 = 131072;
pub const SQL_AT_CONSTRAINT_DEFERRABLE: u32 = 262144;
pub const SQL_AT_CONSTRAINT_NON_DEFERRABLE: u32 = 524288;
pub const SQL_CVT_CHAR: u32 = 1;
pub const SQL_CVT_NUMERIC: u32 = 2;
pub const SQL_CVT_DECIMAL: u32 = 4;
pub const SQL_CVT_INTEGER: u32 = 8;
pub const SQL_CVT_SMALLINT: u32 = 16;
pub const SQL_CVT_FLOAT: u32 = 32;
pub const SQL_CVT_REAL: u32 = 64;
pub const SQL_CVT_DOUBLE: u32 = 128;
pub const SQL_CVT_VARCHAR: u32 = 256;
pub const SQL_CVT_LONGVARCHAR: u32 = 512;
pub const SQL_CVT_BINARY: u32 = 1024;
pub const SQL_CVT_VARBINARY: u32 = 2048;
pub const SQL_CVT_BIT: u32 = 4096;
pub const SQL_CVT_TINYINT: u32 = 8192;
pub const SQL_CVT_BIGINT: u32 = 16384;
pub const SQL_CVT_DATE: u32 = 32768;
pub const SQL_CVT_TIME: u32 = 65536;
pub const SQL_CVT_TIMESTAMP: u32 = 131072;
pub const SQL_CVT_LONGVARBINARY: u32 = 262144;
pub const SQL_CVT_INTERVAL_YEAR_MONTH: u32 = 524288;
pub const SQL_CVT_INTERVAL_DAY_TIME: u32 = 1048576;
pub const SQL_CVT_WCHAR: u32 = 2097152;
pub const SQL_CVT_WLONGVARCHAR: u32 = 4194304;
pub const SQL_CVT_WVARCHAR: u32 = 8388608;
pub const SQL_CVT_GUID: u32 = 16777216;
pub const SQL_FN_CVT_CONVERT: u32 = 1;
pub const SQL_FN_CVT_CAST: u32 = 2;
pub const SQL_FN_STR_CONCAT: u32 = 1;
pub const SQL_FN_STR_INSERT: u32 = 2;
pub const SQL_FN_STR_LEFT: u32 = 4;
pub const SQL_FN_STR_LTRIM: u32 = 8;
pub const SQL_FN_STR_LENGTH: u32 = 16;
pub const SQL_FN_STR_LOCATE: u32 = 32;
pub const SQL_FN_STR_LCASE: u32 = 64;
pub const SQL_FN_STR_REPEAT: u32 = 128;
pub const SQL_FN_STR_REPLACE: u32 = 256;
pub const SQL_FN_STR_RIGHT: u32 = 512;
pub const SQL_FN_STR_RTRIM: u32 = 1024;
pub const SQL_FN_STR_SUBSTRING: u32 = 2048;
pub const SQL_FN_STR_UCASE: u32 = 4096;
pub const SQL_FN_STR_ASCII: u32 = 8192;
pub const SQL_FN_STR_CHAR: u32 = 16384;
pub const SQL_FN_STR_DIFFERENCE: u32 = 32768;
pub const SQL_FN_STR_LOCATE_2: u32 = 65536;
pub const SQL_FN_STR_SOUNDEX: u32 = 131072;
pub const SQL_FN_STR_SPACE: u32 = 262144;
pub const SQL_FN_STR_BIT_LENGTH: u32 = 524288;
pub const SQL_FN_STR_CHAR_LENGTH: u32 = 1048576;
pub const SQL_FN_STR_CHARACTER_LENGTH: u32 = 2097152;
pub const SQL_FN_STR_OCTET_LENGTH: u32 = 4194304;
pub const SQL_FN_STR_POSITION: u32 = 8388608;
pub const SQL_SSF_CONVERT: u32 = 1;
pub const SQL_SSF_LOWER: u32 = 2;
pub const SQL_SSF_UPPER: u32 = 4;
pub const SQL_SSF_SUBSTRING: u32 = 8;
pub const SQL_SSF_TRANSLATE: u32 = 16;
pub const SQL_SSF_TRIM_BOTH: u32 = 32;
pub const SQL_SSF_TRIM_LEADING: u32 = 64;
pub const SQL_SSF_TRIM_TRAILING: u32 = 128;
pub const SQL_FN_NUM_ABS: u32 = 1;
pub const SQL_FN_NUM_ACOS: u32 = 2;
pub const SQL_FN_NUM_ASIN: u32 = 4;
pub const SQL_FN_NUM_ATAN: u32 = 8;
pub const SQL_FN_NUM_ATAN2: u32 = 16;
pub const SQL_FN_NUM_CEILING: u32 = 32;
pub const SQL_FN_NUM_COS: u32 = 64;
pub const SQL_FN_NUM_COT: u32 = 128;
pub const SQL_FN_NUM_EXP: u32 = 256;
pub const SQL_FN_NUM_FLOOR: u32 = 512;
pub const SQL_FN_NUM_LOG: u32 = 1024;
pub const SQL_FN_NUM_MOD: u32 = 2048;
pub const SQL_FN_NUM_SIGN: u32 = 4096;
pub const SQL_FN_NUM_SIN: u32 = 8192;
pub const SQL_FN_NUM_SQRT: u32 = 16384;
pub const SQL_FN_NUM_TAN: u32 = 32768;
pub const SQL_FN_NUM_PI: u32 = 65536;
pub const SQL_FN_NUM_RAND: u32 = 131072;
pub const SQL_FN_NUM_DEGREES: u32 = 262144;
pub const SQL_FN_NUM_LOG10: u32 = 524288;
pub const SQL_FN_NUM_POWER: u32 = 1048576;
pub const SQL_FN_NUM_RADIANS: u32 = 2097152;
pub const SQL_FN_NUM_ROUND: u32 = 4194304;
pub const SQL_FN_NUM_TRUNCATE: u32 = 8388608;
pub const SQL_SNVF_BIT_LENGTH: u32 = 1;
pub const SQL_SNVF_CHAR_LENGTH: u32 = 2;
pub const SQL_SNVF_CHARACTER_LENGTH: u32 = 4;
pub const SQL_SNVF_EXTRACT: u32 = 8;
pub const SQL_SNVF_OCTET_LENGTH: u32 = 16;
pub const SQL_SNVF_POSITION: u32 = 32;
pub const SQL_FN_TD_NOW: u32 = 1;
pub const SQL_FN_TD_CURDATE: u32 = 2;
pub const SQL_FN_TD_DAYOFMONTH: u32 = 4;
pub const SQL_FN_TD_DAYOFWEEK: u32 = 8;
pub const SQL_FN_TD_DAYOFYEAR: u32 = 16;
pub const SQL_FN_TD_MONTH: u32 = 32;
pub const SQL_FN_TD_QUARTER: u32 = 64;
pub const SQL_FN_TD_WEEK: u32 = 128;
pub const SQL_FN_TD_YEAR: u32 = 256;
pub const SQL_FN_TD_CURTIME: u32 = 512;
pub const SQL_FN_TD_HOUR: u32 = 1024;
pub const SQL_FN_TD_MINUTE: u32 = 2048;
pub const SQL_FN_TD_SECOND: u32 = 4096;
pub const SQL_FN_TD_TIMESTAMPADD: u32 = 8192;
pub const SQL_FN_TD_TIMESTAMPDIFF: u32 = 16384;
pub const SQL_FN_TD_DAYNAME: u32 = 32768;
pub const SQL_FN_TD_MONTHNAME: u32 = 65536;
pub const SQL_FN_TD_CURRENT_DATE: u32 = 131072;
pub const SQL_FN_TD_CURRENT_TIME: u32 = 262144;
pub const SQL_FN_TD_CURRENT_TIMESTAMP: u32 = 524288;
pub const SQL_FN_TD_EXTRACT: u32 = 1048576;
pub const SQL_SDF_CURRENT_DATE: u32 = 1;
pub const SQL_SDF_CURRENT_TIME: u32 = 2;
pub const SQL_SDF_CURRENT_TIMESTAMP: u32 = 4;
pub const SQL_FN_SYS_USERNAME: u32 = 1;
pub const SQL_FN_SYS_DBNAME: u32 = 2;
pub const SQL_FN_SYS_IFNULL: u32 = 4;
pub const SQL_FN_TSI_FRAC_SECOND: u32 = 1;
pub const SQL_FN_TSI_SECOND: u32 = 2;
pub const SQL_FN_TSI_MINUTE: u32 = 4;
pub const SQL_FN_TSI_HOUR: u32 = 8;
pub const SQL_FN_TSI_DAY: u32 = 16;
pub const SQL_FN_TSI_WEEK: u32 = 32;
pub const SQL_FN_TSI_MONTH: u32 = 64;
pub const SQL_FN_TSI_QUARTER: u32 = 128;
pub const SQL_FN_TSI_YEAR: u32 = 256;
pub const SQL_CA1_NEXT: u32 = 1;
pub const SQL_CA1_ABSOLUTE: u32 = 2;
pub const SQL_CA1_RELATIVE: u32 = 4;
pub const SQL_CA1_BOOKMARK: u32 = 8;
pub const SQL_CA1_LOCK_NO_CHANGE: u32 = 64;
pub const SQL_CA1_LOCK_EXCLUSIVE: u32 = 128;
pub const SQL_CA1_LOCK_UNLOCK: u32 = 256;
pub const SQL_CA1_POS_POSITION: u32 = 512;
pub const SQL_CA1_POS_UPDATE: u32 = 1024;
pub const SQL_CA1_POS_DELETE: u32 = 2048;
pub const SQL_CA1_POS_REFRESH: u32 = 4096;
pub const SQL_CA1_POSITIONED_UPDATE: u32 = 8192;
pub const SQL_CA1_POSITIONED_DELETE: u32 = 16384;
pub const SQL_CA1_SELECT_FOR_UPDATE: u32 = 32768;
pub const SQL_CA1_BULK_ADD: u32 = 65536;
pub const SQL_CA1_BULK_UPDATE_BY_BOOKMARK: u32 = 131072;
pub const SQL_CA1_BULK_DELETE_BY_BOOKMARK: u32 = 262144;
pub const SQL_CA1_BULK_FETCH_BY_BOOKMARK: u32 = 524288;
pub const SQL_CA2_READ_ONLY_CONCURRENCY: u32 = 1;
pub const SQL_CA2_LOCK_CONCURRENCY: u32 = 2;
pub const SQL_CA2_OPT_ROWVER_CONCURRENCY: u32 = 4;
pub const SQL_CA2_OPT_VALUES_CONCURRENCY: u32 = 8;
pub const SQL_CA2_SENSITIVITY_ADDITIONS: u32 = 16;
pub const SQL_CA2_SENSITIVITY_DELETIONS: u32 = 32;
pub const SQL_CA2_SENSITIVITY_UPDATES: u32 = 64;
pub const SQL_CA2_MAX_ROWS_SELECT: u32 = 128;
pub const SQL_CA2_MAX_ROWS_INSERT: u32 = 256;
pub const SQL_CA2_MAX_ROWS_DELETE: u32 = 512;
pub const SQL_CA2_MAX_ROWS_UPDATE: u32 = 1024;
pub const SQL_CA2_MAX_ROWS_CATALOG: u32 = 2048;
pub const SQL_CA2_MAX_ROWS_AFFECTS_ALL: u32 = 3968;
pub const SQL_CA2_CRC_EXACT: u32 = 4096;
pub const SQL_CA2_CRC_APPROXIMATE: u32 = 8192;
pub const SQL_CA2_SIMULATE_NON_UNIQUE: u32 = 16384;
pub const SQL_CA2_SIMULATE_TRY_UNIQUE: u32 = 32768;
pub const SQL_CA2_SIMULATE_UNIQUE: u32 = 65536;
pub const SQL_OAC_NONE: u32 = 0;
pub const SQL_OAC_LEVEL1: u32 = 1;
pub const SQL_OAC_LEVEL2: u32 = 2;
pub const SQL_OSCC_NOT_COMPLIANT: u32 = 0;
pub const SQL_OSCC_COMPLIANT: u32 = 1;
pub const SQL_OSC_MINIMUM: u32 = 0;
pub const SQL_OSC_CORE: u32 = 1;
pub const SQL_OSC_EXTENDED: u32 = 2;
pub const SQL_CB_NULL: u32 = 0;
pub const SQL_CB_NON_NULL: u32 = 1;
pub const SQL_SO_FORWARD_ONLY: u32 = 1;
pub const SQL_SO_KEYSET_DRIVEN: u32 = 2;
pub const SQL_SO_DYNAMIC: u32 = 4;
pub const SQL_SO_MIXED: u32 = 8;
pub const SQL_SO_STATIC: u32 = 16;
pub const SQL_FD_FETCH_BOOKMARK: u32 = 128;
pub const SQL_CN_NONE: u32 = 0;
pub const SQL_CN_DIFFERENT: u32 = 1;
pub const SQL_CN_ANY: u32 = 2;
pub const SQL_NNC_NULL: u32 = 0;
pub const SQL_NNC_NON_NULL: u32 = 1;
pub const SQL_NC_START: u32 = 2;
pub const SQL_NC_END: u32 = 4;
pub const SQL_FILE_NOT_SUPPORTED: u32 = 0;
pub const SQL_FILE_TABLE: u32 = 1;
pub const SQL_FILE_QUALIFIER: u32 = 2;
pub const SQL_FILE_CATALOG: u32 = 2;
pub const SQL_GD_BLOCK: u32 = 4;
pub const SQL_GD_BOUND: u32 = 8;
pub const SQL_GD_OUTPUT_PARAMS: u32 = 16;
pub const SQL_PS_POSITIONED_DELETE: u32 = 1;
pub const SQL_PS_POSITIONED_UPDATE: u32 = 2;
pub const SQL_PS_SELECT_FOR_UPDATE: u32 = 4;
pub const SQL_GB_NOT_SUPPORTED: u32 = 0;
pub const SQL_GB_GROUP_BY_EQUALS_SELECT: u32 = 1;
pub const SQL_GB_GROUP_BY_CONTAINS_SELECT: u32 = 2;
pub const SQL_GB_NO_RELATION: u32 = 3;
pub const SQL_GB_COLLATE: u32 = 4;
pub const SQL_OU_DML_STATEMENTS: u32 = 1;
pub const SQL_OU_PROCEDURE_INVOCATION: u32 = 2;
pub const SQL_OU_TABLE_DEFINITION: u32 = 4;
pub const SQL_OU_INDEX_DEFINITION: u32 = 8;
pub const SQL_OU_PRIVILEGE_DEFINITION: u32 = 16;
pub const SQL_SU_DML_STATEMENTS: u32 = 1;
pub const SQL_SU_PROCEDURE_INVOCATION: u32 = 2;
pub const SQL_SU_TABLE_DEFINITION: u32 = 4;
pub const SQL_SU_INDEX_DEFINITION: u32 = 8;
pub const SQL_SU_PRIVILEGE_DEFINITION: u32 = 16;
pub const SQL_QU_DML_STATEMENTS: u32 = 1;
pub const SQL_QU_PROCEDURE_INVOCATION: u32 = 2;
pub const SQL_QU_TABLE_DEFINITION: u32 = 4;
pub const SQL_QU_INDEX_DEFINITION: u32 = 8;
pub const SQL_QU_PRIVILEGE_DEFINITION: u32 = 16;
pub const SQL_CU_DML_STATEMENTS: u32 = 1;
pub const SQL_CU_PROCEDURE_INVOCATION: u32 = 2;
pub const SQL_CU_TABLE_DEFINITION: u32 = 4;
pub const SQL_CU_INDEX_DEFINITION: u32 = 8;
pub const SQL_CU_PRIVILEGE_DEFINITION: u32 = 16;
pub const SQL_SQ_COMPARISON: u32 = 1;
pub const SQL_SQ_EXISTS: u32 = 2;
pub const SQL_SQ_IN: u32 = 4;
pub const SQL_SQ_QUANTIFIED: u32 = 8;
pub const SQL_SQ_CORRELATED_SUBQUERIES: u32 = 16;
pub const SQL_U_UNION: u32 = 1;
pub const SQL_U_UNION_ALL: u32 = 2;
pub const SQL_BP_CLOSE: u32 = 1;
pub const SQL_BP_DELETE: u32 = 2;
pub const SQL_BP_DROP: u32 = 4;
pub const SQL_BP_TRANSACTION: u32 = 8;
pub const SQL_BP_UPDATE: u32 = 16;
pub const SQL_BP_OTHER_HSTMT: u32 = 32;
pub const SQL_BP_SCROLL: u32 = 64;
pub const SQL_SS_ADDITIONS: u32 = 1;
pub const SQL_SS_DELETIONS: u32 = 2;
pub const SQL_SS_UPDATES: u32 = 4;
pub const SQL_CV_CREATE_VIEW: u32 = 1;
pub const SQL_CV_CHECK_OPTION: u32 = 2;
pub const SQL_CV_CASCADED: u32 = 4;
pub const SQL_CV_LOCAL: u32 = 8;
pub const SQL_LCK_NO_CHANGE: u32 = 1;
pub const SQL_LCK_EXCLUSIVE: u32 = 2;
pub const SQL_LCK_UNLOCK: u32 = 4;
pub const SQL_POS_POSITION: u32 = 1;
pub const SQL_POS_REFRESH: u32 = 2;
pub const SQL_POS_UPDATE: u32 = 4;
pub const SQL_POS_DELETE: u32 = 8;
pub const SQL_POS_ADD: u32 = 16;
pub const SQL_QL_START: u32 = 1;
pub const SQL_QL_END: u32 = 2;
pub const SQL_AF_AVG: u32 = 1;
pub const SQL_AF_COUNT: u32 = 2;
pub const SQL_AF_MAX: u32 = 4;
pub const SQL_AF_MIN: u32 = 8;
pub const SQL_AF_SUM: u32 = 16;
pub const SQL_AF_DISTINCT: u32 = 32;
pub const SQL_AF_ALL: u32 = 64;
pub const SQL_SC_SQL92_ENTRY: u32 = 1;
pub const SQL_SC_FIPS127_2_TRANSITIONAL: u32 = 2;
pub const SQL_SC_SQL92_INTERMEDIATE: u32 = 4;
pub const SQL_SC_SQL92_FULL: u32 = 8;
pub const SQL_DL_SQL92_DATE: u32 = 1;
pub const SQL_DL_SQL92_TIME: u32 = 2;
pub const SQL_DL_SQL92_TIMESTAMP: u32 = 4;
pub const SQL_DL_SQL92_INTERVAL_YEAR: u32 = 8;
pub const SQL_DL_SQL92_INTERVAL_MONTH: u32 = 16;
pub const SQL_DL_SQL92_INTERVAL_DAY: u32 = 32;
pub const SQL_DL_SQL92_INTERVAL_HOUR: u32 = 64;
pub const SQL_DL_SQL92_INTERVAL_MINUTE: u32 = 128;
pub const SQL_DL_SQL92_INTERVAL_SECOND: u32 = 256;
pub const SQL_DL_SQL92_INTERVAL_YEAR_TO_MONTH: u32 = 512;
pub const SQL_DL_SQL92_INTERVAL_DAY_TO_HOUR: u32 = 1024;
pub const SQL_DL_SQL92_INTERVAL_DAY_TO_MINUTE: u32 = 2048;
pub const SQL_DL_SQL92_INTERVAL_DAY_TO_SECOND: u32 = 4096;
pub const SQL_DL_SQL92_INTERVAL_HOUR_TO_MINUTE: u32 = 8192;
pub const SQL_DL_SQL92_INTERVAL_HOUR_TO_SECOND: u32 = 16384;
pub const SQL_DL_SQL92_INTERVAL_MINUTE_TO_SECOND: u32 = 32768;
pub const SQL_CL_START: u32 = 1;
pub const SQL_CL_END: u32 = 2;
pub const SQL_BRC_PROCEDURES: u32 = 1;
pub const SQL_BRC_EXPLICIT: u32 = 2;
pub const SQL_BRC_ROLLED_UP: u32 = 4;
pub const SQL_BS_SELECT_EXPLICIT: u32 = 1;
pub const SQL_BS_ROW_COUNT_EXPLICIT: u32 = 2;
pub const SQL_BS_SELECT_PROC: u32 = 4;
pub const SQL_BS_ROW_COUNT_PROC: u32 = 8;
pub const SQL_PARC_BATCH: u32 = 1;
pub const SQL_PARC_NO_BATCH: u32 = 2;
pub const SQL_PAS_BATCH: u32 = 1;
pub const SQL_PAS_NO_BATCH: u32 = 2;
pub const SQL_PAS_NO_SELECT: u32 = 3;
pub const SQL_IK_NONE: u32 = 0;
pub const SQL_IK_ASC: u32 = 1;
pub const SQL_IK_DESC: u32 = 2;
pub const SQL_IK_ALL: u32 = 3;
pub const SQL_ISV_ASSERTIONS: u32 = 1;
pub const SQL_ISV_CHARACTER_SETS: u32 = 2;
pub const SQL_ISV_CHECK_CONSTRAINTS: u32 = 4;
pub const SQL_ISV_COLLATIONS: u32 = 8;
pub const SQL_ISV_COLUMN_DOMAIN_USAGE: u32 = 16;
pub const SQL_ISV_COLUMN_PRIVILEGES: u32 = 32;
pub const SQL_ISV_COLUMNS: u32 = 64;
pub const SQL_ISV_CONSTRAINT_COLUMN_USAGE: u32 = 128;
pub const SQL_ISV_CONSTRAINT_TABLE_USAGE: u32 = 256;
pub const SQL_ISV_DOMAIN_CONSTRAINTS: u32 = 512;
pub const SQL_ISV_DOMAINS: u32 = 1024;
pub const SQL_ISV_KEY_COLUMN_USAGE: u32 = 2048;
pub const SQL_ISV_REFERENTIAL_CONSTRAINTS: u32 = 4096;
pub const SQL_ISV_SCHEMATA: u32 = 8192;
pub const SQL_ISV_SQL_LANGUAGES: u32 = 16384;
pub const SQL_ISV_TABLE_CONSTRAINTS: u32 = 32768;
pub const SQL_ISV_TABLE_PRIVILEGES: u32 = 65536;
pub const SQL_ISV_TABLES: u32 = 131072;
pub const SQL_ISV_TRANSLATIONS: u32 = 262144;
pub const SQL_ISV_USAGE_PRIVILEGES: u32 = 524288;
pub const SQL_ISV_VIEW_COLUMN_USAGE: u32 = 1048576;
pub const SQL_ISV_VIEW_TABLE_USAGE: u32 = 2097152;
pub const SQL_ISV_VIEWS: u32 = 4194304;
pub const SQL_AM_NONE: u32 = 0;
pub const SQL_AM_CONNECTION: u32 = 1;
pub const SQL_AM_STATEMENT: u32 = 2;
pub const SQL_AD_CONSTRAINT_NAME_DEFINITION: u32 = 1;
pub const SQL_AD_ADD_DOMAIN_CONSTRAINT: u32 = 2;
pub const SQL_AD_DROP_DOMAIN_CONSTRAINT: u32 = 4;
pub const SQL_AD_ADD_DOMAIN_DEFAULT: u32 = 8;
pub const SQL_AD_DROP_DOMAIN_DEFAULT: u32 = 16;
pub const SQL_AD_ADD_CONSTRAINT_INITIALLY_DEFERRED: u32 = 32;
pub const SQL_AD_ADD_CONSTRAINT_INITIALLY_IMMEDIATE: u32 = 64;
pub const SQL_AD_ADD_CONSTRAINT_DEFERRABLE: u32 = 128;
pub const SQL_AD_ADD_CONSTRAINT_NON_DEFERRABLE: u32 = 256;
pub const SQL_CS_CREATE_SCHEMA: u32 = 1;
pub const SQL_CS_AUTHORIZATION: u32 = 2;
pub const SQL_CS_DEFAULT_CHARACTER_SET: u32 = 4;
pub const SQL_CTR_CREATE_TRANSLATION: u32 = 1;
pub const SQL_CA_CREATE_ASSERTION: u32 = 1;
pub const SQL_CA_CONSTRAINT_INITIALLY_DEFERRED: u32 = 16;
pub const SQL_CA_CONSTRAINT_INITIALLY_IMMEDIATE: u32 = 32;
pub const SQL_CA_CONSTRAINT_DEFERRABLE: u32 = 64;
pub const SQL_CA_CONSTRAINT_NON_DEFERRABLE: u32 = 128;
pub const SQL_CCS_CREATE_CHARACTER_SET: u32 = 1;
pub const SQL_CCS_COLLATE_CLAUSE: u32 = 2;
pub const SQL_CCS_LIMITED_COLLATION: u32 = 4;
pub const SQL_CCOL_CREATE_COLLATION: u32 = 1;
pub const SQL_CDO_CREATE_DOMAIN: u32 = 1;
pub const SQL_CDO_DEFAULT: u32 = 2;
pub const SQL_CDO_CONSTRAINT: u32 = 4;
pub const SQL_CDO_COLLATION: u32 = 8;
pub const SQL_CDO_CONSTRAINT_NAME_DEFINITION: u32 = 16;
pub const SQL_CDO_CONSTRAINT_INITIALLY_DEFERRED: u32 = 32;
pub const SQL_CDO_CONSTRAINT_INITIALLY_IMMEDIATE: u32 = 64;
pub const SQL_CDO_CONSTRAINT_DEFERRABLE: u32 = 128;
pub const SQL_CDO_CONSTRAINT_NON_DEFERRABLE: u32 = 256;
pub const SQL_CT_CREATE_TABLE: u32 = 1;
pub const SQL_CT_COMMIT_PRESERVE: u32 = 2;
pub const SQL_CT_COMMIT_DELETE: u32 = 4;
pub const SQL_CT_GLOBAL_TEMPORARY: u32 = 8;
pub const SQL_CT_LOCAL_TEMPORARY: u32 = 16;
pub const SQL_CT_CONSTRAINT_INITIALLY_DEFERRED: u32 = 32;
pub const SQL_CT_CONSTRAINT_INITIALLY_IMMEDIATE: u32 = 64;
pub const SQL_CT_CONSTRAINT_DEFERRABLE: u32 = 128;
pub const SQL_CT_CONSTRAINT_NON_DEFERRABLE: u32 = 256;
pub const SQL_CT_COLUMN_CONSTRAINT: u32 = 512;
pub const SQL_CT_COLUMN_DEFAULT: u32 = 1024;
pub const SQL_CT_COLUMN_COLLATION: u32 = 2048;
pub const SQL_CT_TABLE_CONSTRAINT: u32 = 4096;
pub const SQL_CT_CONSTRAINT_NAME_DEFINITION: u32 = 8192;
pub const SQL_DI_CREATE_INDEX: u32 = 1;
pub const SQL_DI_DROP_INDEX: u32 = 2;
pub const SQL_DC_DROP_COLLATION: u32 = 1;
pub const SQL_DD_DROP_DOMAIN: u32 = 1;
pub const SQL_DD_RESTRICT: u32 = 2;
pub const SQL_DD_CASCADE: u32 = 4;
pub const SQL_DS_DROP_SCHEMA: u32 = 1;
pub const SQL_DS_RESTRICT: u32 = 2;
pub const SQL_DS_CASCADE: u32 = 4;
pub const SQL_DCS_DROP_CHARACTER_SET: u32 = 1;
pub const SQL_DA_DROP_ASSERTION: u32 = 1;
pub const SQL_DT_DROP_TABLE: u32 = 1;
pub const SQL_DT_RESTRICT: u32 = 2;
pub const SQL_DT_CASCADE: u32 = 4;
pub const SQL_DTR_DROP_TRANSLATION: u32 = 1;
pub const SQL_DV_DROP_VIEW: u32 = 1;
pub const SQL_DV_RESTRICT: u32 = 2;
pub const SQL_DV_CASCADE: u32 = 4;
pub const SQL_IS_INSERT_LITERALS: u32 = 1;
pub const SQL_IS_INSERT_SEARCHED: u32 = 2;
pub const SQL_IS_SELECT_INTO: u32 = 4;
pub const SQL_OIC_CORE: u32 = 1;
pub const SQL_OIC_LEVEL1: u32 = 2;
pub const SQL_OIC_LEVEL2: u32 = 3;
pub const SQL_SFKD_CASCADE: u32 = 1;
pub const SQL_SFKD_NO_ACTION: u32 = 2;
pub const SQL_SFKD_SET_DEFAULT: u32 = 4;
pub const SQL_SFKD_SET_NULL: u32 = 8;
pub const SQL_SFKU_CASCADE: u32 = 1;
pub const SQL_SFKU_NO_ACTION: u32 = 2;
pub const SQL_SFKU_SET_DEFAULT: u32 = 4;
pub const SQL_SFKU_SET_NULL: u32 = 8;
pub const SQL_SG_USAGE_ON_DOMAIN: u32 = 1;
pub const SQL_SG_USAGE_ON_CHARACTER_SET: u32 = 2;
pub const SQL_SG_USAGE_ON_COLLATION: u32 = 4;
pub const SQL_SG_USAGE_ON_TRANSLATION: u32 = 8;
pub const SQL_SG_WITH_GRANT_OPTION: u32 = 16;
pub const SQL_SG_DELETE_TABLE: u32 = 32;
pub const SQL_SG_INSERT_TABLE: u32 = 64;
pub const SQL_SG_INSERT_COLUMN: u32 = 128;
pub const SQL_SG_REFERENCES_TABLE: u32 = 256;
pub const SQL_SG_REFERENCES_COLUMN: u32 = 512;
pub const SQL_SG_SELECT_TABLE: u32 = 1024;
pub const SQL_SG_UPDATE_TABLE: u32 = 2048;
pub const SQL_SG_UPDATE_COLUMN: u32 = 4096;
pub const SQL_SP_EXISTS: u32 = 1;
pub const SQL_SP_ISNOTNULL: u32 = 2;
pub const SQL_SP_ISNULL: u32 = 4;
pub const SQL_SP_MATCH_FULL: u32 = 8;
pub const SQL_SP_MATCH_PARTIAL: u32 = 16;
pub const SQL_SP_MATCH_UNIQUE_FULL: u32 = 32;
pub const SQL_SP_MATCH_UNIQUE_PARTIAL: u32 = 64;
pub const SQL_SP_OVERLAPS: u32 = 128;
pub const SQL_SP_UNIQUE: u32 = 256;
pub const SQL_SP_LIKE: u32 = 512;
pub const SQL_SP_IN: u32 = 1024;
pub const SQL_SP_BETWEEN: u32 = 2048;
pub const SQL_SP_COMPARISON: u32 = 4096;
pub const SQL_SP_QUANTIFIED_COMPARISON: u32 = 8192;
pub const SQL_SRJO_CORRESPONDING_CLAUSE: u32 = 1;
pub const SQL_SRJO_CROSS_JOIN: u32 = 2;
pub const SQL_SRJO_EXCEPT_JOIN: u32 = 4;
pub const SQL_SRJO_FULL_OUTER_JOIN: u32 = 8;
pub const SQL_SRJO_INNER_JOIN: u32 = 16;
pub const SQL_SRJO_INTERSECT_JOIN: u32 = 32;
pub const SQL_SRJO_LEFT_OUTER_JOIN: u32 = 64;
pub const SQL_SRJO_NATURAL_JOIN: u32 = 128;
pub const SQL_SRJO_RIGHT_OUTER_JOIN: u32 = 256;
pub const SQL_SRJO_UNION_JOIN: u32 = 512;
pub const SQL_SR_USAGE_ON_DOMAIN: u32 = 1;
pub const SQL_SR_USAGE_ON_CHARACTER_SET: u32 = 2;
pub const SQL_SR_USAGE_ON_COLLATION: u32 = 4;
pub const SQL_SR_USAGE_ON_TRANSLATION: u32 = 8;
pub const SQL_SR_GRANT_OPTION_FOR: u32 = 16;
pub const SQL_SR_CASCADE: u32 = 32;
pub const SQL_SR_RESTRICT: u32 = 64;
pub const SQL_SR_DELETE_TABLE: u32 = 128;
pub const SQL_SR_INSERT_TABLE: u32 = 256;
pub const SQL_SR_INSERT_COLUMN: u32 = 512;
pub const SQL_SR_REFERENCES_TABLE: u32 = 1024;
pub const SQL_SR_REFERENCES_COLUMN: u32 = 2048;
pub const SQL_SR_SELECT_TABLE: u32 = 4096;
pub const SQL_SR_UPDATE_TABLE: u32 = 8192;
pub const SQL_SR_UPDATE_COLUMN: u32 = 16384;
pub const SQL_SRVC_VALUE_EXPRESSION: u32 = 1;
pub const SQL_SRVC_NULL: u32 = 2;
pub const SQL_SRVC_DEFAULT: u32 = 4;
pub const SQL_SRVC_ROW_SUBQUERY: u32 = 8;
pub const SQL_SVE_CASE: u32 = 1;
pub const SQL_SVE_CAST: u32 = 2;
pub const SQL_SVE_COALESCE: u32 = 4;
pub const SQL_SVE_NULLIF: u32 = 8;
pub const SQL_SCC_XOPEN_CLI_VERSION1: u32 = 1;
pub const SQL_SCC_ISO92_CLI: u32 = 2;
pub const SQL_US_UNION: u32 = 1;
pub const SQL_US_UNION_ALL: u32 = 2;
pub const SQL_DTC_ENLIST_EXPENSIVE: u32 = 1;
pub const SQL_DTC_UNENLIST_EXPENSIVE: u32 = 2;
pub const SQL_ASYNC_DBC_NOT_CAPABLE: u32 = 0;
pub const SQL_ASYNC_DBC_CAPABLE: u32 = 1;
pub const SQL_FETCH_FIRST_USER: u32 = 31;
pub const SQL_FETCH_FIRST_SYSTEM: u32 = 32;
pub const SQL_ENTIRE_ROWSET: u32 = 0;
pub const SQL_POSITION: u32 = 0;
pub const SQL_REFRESH: u32 = 1;
pub const SQL_UPDATE: u32 = 2;
pub const SQL_DELETE: u32 = 3;
pub const SQL_ADD: u32 = 4;
pub const SQL_SETPOS_MAX_OPTION_VALUE: u32 = 4;
pub const SQL_UPDATE_BY_BOOKMARK: u32 = 5;
pub const SQL_DELETE_BY_BOOKMARK: u32 = 6;
pub const SQL_FETCH_BY_BOOKMARK: u32 = 7;
pub const SQL_LOCK_NO_CHANGE: u32 = 0;
pub const SQL_LOCK_EXCLUSIVE: u32 = 1;
pub const SQL_LOCK_UNLOCK: u32 = 2;
pub const SQL_SETPOS_MAX_LOCK_VALUE: u32 = 2;
pub const SQL_BEST_ROWID: u32 = 1;
pub const SQL_ROWVER: u32 = 2;
pub const SQL_PC_NOT_PSEUDO: u32 = 1;
pub const SQL_QUICK: u32 = 0;
pub const SQL_ENSURE: u32 = 1;
pub const SQL_TABLE_STAT: u32 = 0;
pub const SQL_ALL_CATALOGS: &'static [u8; 2usize] = b"%\0";
pub const SQL_ALL_SCHEMAS: &'static [u8; 2usize] = b"%\0";
pub const SQL_ALL_TABLE_TYPES: &'static [u8; 2usize] = b"%\0";
pub const SQL_DRIVER_NOPROMPT: u32 = 0;
pub const SQL_DRIVER_COMPLETE: u32 = 1;
pub const SQL_DRIVER_PROMPT: u32 = 2;
pub const SQL_DRIVER_COMPLETE_REQUIRED: u32 = 3;
pub const SQL_FETCH_BOOKMARK: u32 = 8;
pub const SQL_ROW_SUCCESS: u32 = 0;
pub const SQL_ROW_DELETED: u32 = 1;
pub const SQL_ROW_UPDATED: u32 = 2;
pub const SQL_ROW_NOROW: u32 = 3;
pub const SQL_ROW_ADDED: u32 = 4;
pub const SQL_ROW_ERROR: u32 = 5;
pub const SQL_ROW_SUCCESS_WITH_INFO: u32 = 6;
pub const SQL_ROW_PROCEED: u32 = 0;
pub const SQL_ROW_IGNORE: u32 = 1;
pub const SQL_PARAM_SUCCESS: u32 = 0;
pub const SQL_PARAM_SUCCESS_WITH_INFO: u32 = 6;
pub const SQL_PARAM_ERROR: u32 = 5;
pub const SQL_PARAM_UNUSED: u32 = 7;
pub const SQL_PARAM_DIAG_UNAVAILABLE: u32 = 1;
pub const SQL_PARAM_PROCEED: u32 = 0;
pub const SQL_PARAM_IGNORE: u32 = 1;
pub const SQL_CASCADE: u32 = 0;
pub const SQL_RESTRICT: u32 = 1;
pub const SQL_SET_NULL: u32 = 2;
pub const SQL_NO_ACTION: u32 = 3;
pub const SQL_SET_DEFAULT: u32 = 4;
pub const SQL_INITIALLY_DEFERRED: u32 = 5;
pub const SQL_INITIALLY_IMMEDIATE: u32 = 6;
pub const SQL_NOT_DEFERRABLE: u32 = 7;
pub const SQL_PARAM_TYPE_UNKNOWN: u32 = 0;
pub const SQL_PARAM_INPUT: u32 = 1;
pub const SQL_PARAM_INPUT_OUTPUT: u32 = 2;
pub const SQL_RESULT_COL: u32 = 3;
pub const SQL_PARAM_OUTPUT: u32 = 4;
pub const SQL_RETURN_VALUE: u32 = 5;
pub const SQL_PARAM_INPUT_OUTPUT_STREAM: u32 = 8;
pub const SQL_PARAM_OUTPUT_STREAM: u32 = 16;
pub const SQL_PT_UNKNOWN: u32 = 0;
pub const SQL_PT_PROCEDURE: u32 = 1;
pub const SQL_PT_FUNCTION: u32 = 2;
pub const SQL_DATABASE_NAME: u32 = 16;
pub const SQL_CONCUR_TIMESTAMP: u32 = 3;
pub const SQL_SCROLL_FORWARD_ONLY: u32 = 0;
pub const SQL_SCROLL_KEYSET_DRIVEN: i32 = -1;
pub const SQL_SCROLL_DYNAMIC: i32 = -2;
pub const SQL_SCROLL_STATIC: i32 = -3;
pub const TRACE_VERSION: u32 = 1000;
pub const TRACE_ON: u32 = 1;
pub const TRACE_VS_EVENT_ON: u32 = 2;
pub const ODBC_VS_FLAG_UNICODE_ARG: u32 = 1;
pub const ODBC_VS_FLAG_UNICODE_COR: u32 = 2;
pub const ODBC_VS_FLAG_RETCODE: u32 = 4;
pub const ODBC_VS_FLAG_STOP: u32 = 8;
pub const SQL_API_SQLALLOCCONNECT: u32 = 1;
pub const SQL_API_SQLALLOCENV: u32 = 2;
pub const SQL_API_SQLALLOCSTMT: u32 = 3;
pub const SQL_API_SQLBINDCOL: u32 = 4;
pub const SQL_API_SQLBINDPARAM: u32 = 1002;
pub const SQL_API_SQLCANCEL: u32 = 5;
pub const SQL_API_SQLCONNECT: u32 = 7;
pub const SQL_API_SQLCOPYDESC: u32 = 1004;
pub const SQL_API_SQLDESCRIBECOL: u32 = 8;
pub const SQL_API_SQLDISCONNECT: u32 = 9;
pub const SQL_API_SQLERROR: u32 = 10;
pub const SQL_API_SQLEXECDIRECT: u32 = 11;
pub const SQL_API_SQLEXECUTE: u32 = 12;
pub const SQL_API_SQLFETCH: u32 = 13;
pub const SQL_API_SQLFREECONNECT: u32 = 14;
pub const SQL_API_SQLFREEENV: u32 = 15;
pub const SQL_API_SQLFREESTMT: u32 = 16;
pub const SQL_API_SQLGETCURSORNAME: u32 = 17;
pub const SQL_API_SQLNUMRESULTCOLS: u32 = 18;
pub const SQL_API_SQLPREPARE: u32 = 19;
pub const SQL_API_SQLROWCOUNT: u32 = 20;
pub const SQL_API_SQLSETCURSORNAME: u32 = 21;
pub const SQL_API_SQLSETDESCFIELD: u32 = 1017;
pub const SQL_API_SQLSETDESCREC: u32 = 1018;
pub const SQL_API_SQLSETENVATTR: u32 = 1019;
pub const SQL_API_SQLSETPARAM: u32 = 22;
pub const SQL_API_SQLTRANSACT: u32 = 23;
pub const SQL_API_SQLCOLUMNS: u32 = 40;
pub const SQL_API_SQLGETCONNECTOPTION: u32 = 42;
pub const SQL_API_SQLGETDATA: u32 = 43;
pub const SQL_API_SQLGETDATAINTERNAL: u32 = 174;
pub const SQL_API_SQLGETDESCFIELD: u32 = 1008;
pub const SQL_API_SQLGETDESCREC: u32 = 1009;
pub const SQL_API_SQLGETDIAGFIELD: u32 = 1010;
pub const SQL_API_SQLGETDIAGREC: u32 = 1011;
pub const SQL_API_SQLGETENVATTR: u32 = 1012;
pub const SQL_API_SQLGETFUNCTIONS: u32 = 44;
pub const SQL_API_SQLGETINFO: u32 = 45;
pub const SQL_API_SQLGETSTMTOPTION: u32 = 46;
pub const SQL_API_SQLGETTYPEINFO: u32 = 47;
pub const SQL_API_SQLPARAMDATA: u32 = 48;
pub const SQL_API_SQLPUTDATA: u32 = 49;
pub const SQL_API_SQLSETCONNECTOPTION: u32 = 50;
pub const SQL_API_SQLSETSTMTOPTION: u32 = 51;
pub const SQL_API_SQLSPECIALCOLUMNS: u32 = 52;
pub const SQL_API_SQLSTATISTICS: u32 = 53;
pub const SQL_API_SQLTABLES: u32 = 54;
pub const SQL_API_SQLDATASOURCES: u32 = 57;
pub const SQL_API_SQLSETCONNECTATTR: u32 = 1016;
pub const SQL_API_SQLSETSTMTATTR: u32 = 1020;
pub const SQL_API_SQLBINDFILETOCOL: u32 = 1250;
pub const SQL_API_SQLBINDFILETOPARAM: u32 = 1251;
pub const SQL_API_SQLSETCOLATTRIBUTES: u32 = 1252;
pub const SQL_API_SQLGETSQLCA: u32 = 1253;
pub const SQL_API_SQLSETCONNECTION: u32 = 1254;
pub const SQL_API_SQLGETDATALINKATTR: u32 = 1255;
pub const SQL_API_SQLBUILDDATALINK: u32 = 1256;
pub const SQL_API_SQLNEXTRESULT: u32 = 1257;
pub const SQL_API_SQLCREATEDB: u32 = 1258;
pub const SQL_API_SQLDROPDB: u32 = 1259;
pub const SQL_API_SQLCREATEPKG: u32 = 1260;
pub const SQL_API_SQLDROPPKG: u32 = 1261;
pub const SQL_API_SQLEXTENDEDPREPARE: u32 = 1296;
pub const SQL_API_SQLEXTENDEDBIND: u32 = 1297;
pub const SQL_API_SQLEXTENDEDDESCRIBE: u32 = 1298;
pub const SQL_API_SQLRELOADCONFIG: u32 = 1299;
pub const SQL_API_SQLFETCHSCROLL: u32 = 1021;
pub const SQL_API_SQLGETLENGTH: u32 = 1022;
pub const SQL_API_SQLGETPOSITION: u32 = 1023;
pub const SQL_API_SQLGETSUBSTRING: u32 = 1024;
pub const SQL_API_SQLEXTENDEDPROCEDURES: u32 = 1025;
pub const SQL_API_SQLEXTENDEDPROCEDURECOLUMNS: u32 = 1026;
pub const SQL_API_SQLALLOCHANDLE: u32 = 1001;
pub const SQL_API_SQLFREEHANDLE: u32 = 1006;
pub const SQL_API_SQLCLOSECURSOR: u32 = 1003;
pub const SQL_API_SQLENDTRAN: u32 = 1005;
pub const SQL_API_SQLCOLATTRIBUTE: u32 = 6;
pub const SQL_API_SQLGETSTMTATTR: u32 = 1014;
pub const SQL_API_SQLGETCONNECTATTR: u32 = 1007;
pub const SQL_EXT_API_LAST: u32 = 72;
pub const SQL_MAX_DRIVER_CONNECTIONS: u32 = 0;
pub const SQL_MAXIMUM_DRIVER_CONNECTIONS: u32 = 0;
pub const SQL_MAX_CONCURRENT_ACTIVITIES: u32 = 1;
pub const SQL_MAXIMUM_CONCURRENT_ACTIVITIES: u32 = 1;
pub const SQL_DROP_MODULE: u32 = 2600;
pub const SQL_MODULE_USAGE: u32 = 2601;
pub const SQL_CREATE_MODULE: u32 = 2602;
pub const SQL_MAX_MODULE_NAME_LEN: u32 = 2603;
pub const SQL_DRIVER_BLDLEVEL: u32 = 2604;
pub const SQL_DATALINK_URL: &'static [u8; 4usize] = b"URL\0";
pub const SQL_ATTR_DATALINK_COMMENT: u32 = 1;
pub const SQL_ATTR_DATALINK_LINKTYPE: u32 = 2;
pub const SQL_ATTR_DATALINK_URLCOMPLETE: u32 = 3;
pub const SQL_ATTR_DATALINK_URLPATH: u32 = 4;
pub const SQL_ATTR_DATALINK_URLPATHONLY: u32 = 5;
pub const SQL_ATTR_DATALINK_URLSCHEME: u32 = 6;
pub const SQL_ATTR_DATALINK_URLSERVER: u32 = 7;
pub const SQL_DATA_SOURCE_NAME: u32 = 2;
pub const SQL_FETCH_DIRECTION: u32 = 8;
pub const SQL_SERVER_NAME: u32 = 13;
pub const SQL_SEARCH_PATTERN_ESCAPE: u32 = 14;
pub const SQL_DBMS_NAME: u32 = 17;
pub const SQL_DBMS_VER: u32 = 18;
pub const SQL_ACCESSIBLE_TABLES: u32 = 19;
pub const SQL_ACCESSIBLE_PROCEDURES: u32 = 20;
pub const SQL_CURSOR_COMMIT_BEHAVIOR: u32 = 23;
pub const SQL_DATA_SOURCE_READ_ONLY: u32 = 25;
pub const SQL_DEFAULT_TXN_ISOLATION: u32 = 26;
pub const SQL_IDENTIFIER_CASE: u32 = 28;
pub const SQL_IDENTIFIER_QUOTE_CHAR: u32 = 29;
pub const SQL_MAX_COLUMN_NAME_LEN: u32 = 30;
pub const SQL_MAXIMUM_COLUMN_NAME_LENGTH: u32 = 30;
pub const SQL_MAX_CURSOR_NAME_LEN: u32 = 31;
pub const SQL_MAXIMUM_CURSOR_NAME_LENGTH: u32 = 31;
pub const SQL_MAX_TABLE_NAME_LEN: u32 = 35;
pub const SQL_SCROLL_CONCURRENCY: u32 = 43;
pub const SQL_TXN_CAPABLE: u32 = 46;
pub const SQL_TRANSACTION_CAPABLE: u32 = 46;
pub const SQL_USER_NAME: u32 = 47;
pub const SQL_TXN_ISOLATION_OPTION: u32 = 72;
pub const SQL_TRANSACTION_ISOLATION_OPTION: u32 = 72;
pub const SQL_GETDATA_EXTENSIONS: u32 = 81;
pub const SQL_NULL_COLLATION: u32 = 85;
pub const SQL_ALTER_TABLE: u32 = 86;
pub const SQL_ORDER_BY_COLUMNS_IN_SELECT: u32 = 90;
pub const SQL_SPECIAL_CHARACTERS: u32 = 94;
pub const SQL_MAX_COLUMNS_IN_GROUP_BY: u32 = 97;
pub const SQL_MAXIMUM_COLUMNS_IN_GROUP_BY: u32 = 97;
pub const SQL_MAX_COLUMNS_IN_INDEX: u32 = 98;
pub const SQL_MAXIMUM_COLUMNS_IN_INDEX: u32 = 98;
pub const SQL_MAX_COLUMNS_IN_ORDER_BY: u32 = 99;
pub const SQL_MAXIMUM_COLUMNS_IN_ORDER_BY: u32 = 99;
pub const SQL_MAX_COLUMNS_IN_SELECT: u32 = 100;
pub const SQL_MAXIMUM_COLUMNS_IN_SELECT: u32 = 100;
pub const SQL_MAX_COLUMNS_IN_TABLE: u32 = 101;
pub const SQL_MAX_INDEX_SIZE: u32 = 102;
pub const SQL_MAXIMUM_INDEX_SIZE: u32 = 102;
pub const SQL_MAX_ROW_SIZE: u32 = 104;
pub const SQL_MAXIMUM_ROW_SIZE: u32 = 104;
pub const SQL_MAX_STATEMENT_LEN: u32 = 105;
pub const SQL_MAXIMUM_STATEMENT_LENGTH: u32 = 105;
pub const SQL_MAX_TABLES_IN_SELECT: u32 = 106;
pub const SQL_MAXIMUM_TABLES_IN_SELECT: u32 = 106;
pub const SQL_MAX_USER_NAME_LEN: u32 = 107;
pub const SQL_MAXIMUM_USER_NAME_LENGTH: u32 = 107;
pub const SQL_MAX_SCHEMA_NAME_LEN: u32 = 32;
pub const SQL_MAXIMUM_SCHEMA_NAME_LENGTH: u32 = 32;
pub const SQL_MAX_CATALOG_NAME_LEN: u32 = 34;
pub const SQL_MAXIMUM_CATALOG_NAME_LENGTH: u32 = 34;
pub const SQL_OJ_CAPABILITIES: u32 = 115;
pub const SQL_CONFIG_KEYWORDS: u32 = 174;
pub const SQL_OUTER_JOIN_CAPABILITIES: u32 = 115;
pub const SQL_XOPEN_CLI_YEAR: u32 = 10000;
pub const SQL_CURSOR_SENSITIVITY: u32 = 10001;
pub const SQL_DESCRIBE_PARAMETER: u32 = 10002;
pub const SQL_CATALOG_NAME: u32 = 10003;
pub const SQL_COLLATION_SEQ: u32 = 10004;
pub const SQL_MAX_IDENTIFIER_LEN: u32 = 10005;
pub const SQL_MAXIMUM_IDENTIFIER_LENGTH: u32 = 10005;
pub const SQL_INTEGRITY: u32 = 73;
pub const SQL_DATABASE_CODEPAGE: u32 = 2519;
pub const SQL_APPLICATION_CODEPAGE: u32 = 2520;
pub const SQL_CONNECT_CODEPAGE: u32 = 2521;
pub const SQL_ATTR_DB2_APPLICATION_ID: u32 = 2532;
pub const SQL_ATTR_DB2_APPLICATION_HANDLE: u32 = 2533;
pub const SQL_ATTR_HANDLE_XA_ASSOCIATED: u32 = 2535;
pub const SQL_DB2_DRIVER_VER: u32 = 2550;
pub const SQL_ATTR_XML_DECLARATION: u32 = 2552;
pub const SQL_ATTR_CURRENT_IMPLICIT_XMLPARSE_OPTION: u32 = 2553;
pub const SQL_ATTR_XQUERY_STATEMENT: u32 = 2557;
pub const SQL_DB2_DRIVER_TYPE: u32 = 2567;
pub const SQL_INPUT_CHAR_CONVFACTOR: u32 = 2581;
pub const SQL_OUTPUT_CHAR_CONVFACTOR: u32 = 2582;
pub const SQL_ATTR_REPLACE_QUOTED_LITERALS: u32 = 2586;
pub const SQL_ATTR_REPORT_TIMESTAMP_TRUNC_AS_WARN: u32 = 2587;
pub const SQL_ATTR_CLIENT_ENCALG: u32 = 2589;
pub const SQL_ATTR_CONCURRENT_ACCESS_RESOLUTION: u32 = 2595;
pub const SQL_ATTR_REPORT_SEAMLESSFAILOVER_WARNING: u32 = 2605;
pub const SQL_CONCURRENT_ACCESS_RESOLUTION_UNSET: u32 = 0;
pub const SQL_USE_CURRENTLY_COMMITTED: u32 = 1;
pub const SQL_WAIT_FOR_OUTCOME: u32 = 2;
pub const SQL_SKIP_LOCKED_DATA: u32 = 3;
pub const SQL_DBMS_FUNCTIONLVL: u32 = 203;
pub const SQL_CLI_STMT_UNDEFINED: u32 = 0;
pub const SQL_CLI_STMT_ALTER_TABLE: u32 = 1;
pub const SQL_CLI_STMT_CREATE_INDEX: u32 = 5;
pub const SQL_CLI_STMT_CREATE_TABLE: u32 = 6;
pub const SQL_CLI_STMT_CREATE_VIEW: u32 = 7;
pub const SQL_CLI_STMT_DELETE_SEARCHED: u32 = 8;
pub const SQL_CLI_STMT_DELETE_POSITIONED: u32 = 9;
pub const SQL_CLI_STMT_DROP_PACKAGE: u32 = 10;
pub const SQL_CLI_STMT_DROP_INDEX: u32 = 11;
pub const SQL_CLI_STMT_DROP_TABLE: u32 = 12;
pub const SQL_CLI_STMT_DROP_VIEW: u32 = 13;
pub const SQL_CLI_STMT_GRANT: u32 = 14;
pub const SQL_CLI_STMT_INSERT: u32 = 15;
pub const SQL_CLI_STMT_REVOKE: u32 = 16;
pub const SQL_CLI_STMT_SELECT: u32 = 18;
pub const SQL_CLI_STMT_UPDATE_SEARCHED: u32 = 19;
pub const SQL_CLI_STMT_UPDATE_POSITIONED: u32 = 20;
pub const SQL_CLI_STMT_CALL: u32 = 24;
pub const SQL_CLI_STMT_SELECT_FOR_UPDATE: u32 = 29;
pub const SQL_CLI_STMT_WITH: u32 = 30;
pub const SQL_CLI_STMT_SELECT_FOR_FETCH: u32 = 31;
pub const SQL_CLI_STMT_VALUES: u32 = 32;
pub const SQL_CLI_STMT_CREATE_TRIGGER: u32 = 34;
pub const SQL_CLI_STMT_SELECT_OPTIMIZE_FOR_NROWS: u32 = 39;
pub const SQL_CLI_STMT_SELECT_INTO: u32 = 40;
pub const SQL_CLI_STMT_CREATE_PROCEDURE: u32 = 41;
pub const SQL_CLI_STMT_CREATE_FUNCTION: u32 = 42;
pub const SQL_CLI_STMT_INSERT_VALUES: u32 = 45;
pub const SQL_CLI_STMT_SET_CURRENT_QUERY_OPT: u32 = 46;
pub const SQL_CLI_STMT_MERGE: u32 = 56;
pub const SQL_CLI_STMT_XQUERY: u32 = 59;
pub const SQL_CLI_STMT_SET: u32 = 62;
pub const SQL_CLI_STMT_ALTER_PROCEDURE: u32 = 63;
pub const SQL_CLI_STMT_CLOSE_DATABASE: u32 = 64;
pub const SQL_CLI_STMT_CREATE_DATABASE: u32 = 65;
pub const SQL_CLI_STMT_DROP_DATABASE: u32 = 66;
pub const SQL_CLI_STMT_ANONYMOUS_BLOCK: u32 = 72;
pub const SQL_IBM_ALTERTABLEVARCHAR: u32 = 1000;
pub const SQL_AT_ADD_COLUMN: u32 = 1;
pub const SQL_AT_DROP_COLUMN: u32 = 2;
pub const SQL_AT_ADD_CONSTRAINT: u32 = 8;
pub const SQL_CB_DELETE: u32 = 0;
pub const SQL_CB_CLOSE: u32 = 1;
pub const SQL_CB_PRESERVE: u32 = 2;
pub const SQL_IC_UPPER: u32 = 1;
pub const SQL_IC_LOWER: u32 = 2;
pub const SQL_IC_SENSITIVE: u32 = 3;
pub const SQL_IC_MIXED: u32 = 4;
pub const SQL_TC_NONE: u32 = 0;
pub const SQL_TC_DML: u32 = 1;
pub const SQL_TC_ALL: u32 = 2;
pub const SQL_TC_DDL_COMMIT: u32 = 3;
pub const SQL_TC_DDL_IGNORE: u32 = 4;
pub const SQL_SCCO_READ_ONLY: u32 = 1;
pub const SQL_SCCO_LOCK: u32 = 2;
pub const SQL_SCCO_OPT_ROWVER: u32 = 4;
pub const SQL_SCCO_OPT_VALUES: u32 = 8;
pub const SQL_FD_FETCH_NEXT: u32 = 1;
pub const SQL_FD_FETCH_FIRST: u32 = 2;
pub const SQL_FD_FETCH_LAST: u32 = 4;
pub const SQL_FD_FETCH_PRIOR: u32 = 8;
pub const SQL_FD_FETCH_ABSOLUTE: u32 = 16;
pub const SQL_FD_FETCH_RELATIVE: u32 = 32;
pub const SQL_FD_FETCH_RESUME: u32 = 64;
pub const SQL_TXN_READ_UNCOMMITTED: u32 = 1;
pub const SQL_TRANSACTION_READ_UNCOMMITTED: u32 = 1;
pub const SQL_TXN_READ_COMMITTED: u32 = 2;
pub const SQL_TRANSACTION_READ_COMMITTED: u32 = 2;
pub const SQL_TXN_REPEATABLE_READ: u32 = 4;
pub const SQL_TRANSACTION_REPEATABLE_READ: u32 = 4;
pub const SQL_TXN_SERIALIZABLE: u32 = 8;
pub const SQL_TRANSACTION_SERIALIZABLE: u32 = 8;
pub const SQL_TXN_NOCOMMIT: u32 = 32;
pub const SQL_TRANSACTION_NOCOMMIT: u32 = 32;
pub const SQL_TXN_IDS_CURSOR_STABILITY: u32 = 64;
pub const SQL_TRANSACTION_IDS_CURSOR_STABILITY: u32 = 64;
pub const SQL_TXN_IDS_LAST_COMMITTED: u32 = 128;
pub const SQL_TRANSACTION_IDS_LAST_COMMITTED: u32 = 128;
pub const SQL_GD_ANY_COLUMN: u32 = 1;
pub const SQL_GD_ANY_ORDER: u32 = 2;
pub const SQL_OJ_LEFT: u32 = 1;
pub const SQL_OJ_RIGHT: u32 = 2;
pub const SQL_OJ_FULL: u32 = 4;
pub const SQL_OJ_NESTED: u32 = 8;
pub const SQL_OJ_NOT_ORDERED: u32 = 16;
pub const SQL_OJ_INNER: u32 = 32;
pub const SQL_OJ_ALL_COMPARISON_OPS: u32 = 64;
pub const SQL_CLI_DRIVER_TYPE_UNDEFINED: u32 = 0;
pub const SQL_CLI_DRIVER_RUNTIME_CLIENT: u32 = 1;
pub const SQL_CLI_DRIVER_CLI_DRIVER: u32 = 2;
pub const SQL_ALL_TYPES: u32 = 0;
pub const SQL_ATTR_AUTO_IPD: u32 = 10001;
pub const SQL_ATTR_APP_ROW_DESC: u32 = 10010;
pub const SQL_ATTR_APP_PARAM_DESC: u32 = 10011;
pub const SQL_ATTR_IMP_ROW_DESC: u32 = 10012;
pub const SQL_ATTR_IMP_PARAM_DESC: u32 = 10013;
pub const SQL_ATTR_METADATA_ID: u32 = 10014;
pub const SQL_ATTR_CURSOR_SCROLLABLE: i32 = -1;
pub const SQL_ATTR_CURSOR_SENSITIVITY: i32 = -2;
pub const SQL_NONSCROLLABLE: u32 = 0;
pub const SQL_SCROLLABLE: u32 = 1;
pub const SQL_CURSOR_HOLD: u32 = 1250;
pub const SQL_ATTR_CURSOR_HOLD: u32 = 1250;
pub const SQL_NODESCRIBE_OUTPUT: u32 = 1251;
pub const SQL_ATTR_NODESCRIBE_OUTPUT: u32 = 1251;
pub const SQL_NODESCRIBE_INPUT: u32 = 1264;
pub const SQL_ATTR_NODESCRIBE_INPUT: u32 = 1264;
pub const SQL_NODESCRIBE: u32 = 1251;
pub const SQL_ATTR_NODESCRIBE: u32 = 1251;
pub const SQL_CLOSE_BEHAVIOR: u32 = 1257;
pub const SQL_ATTR_CLOSE_BEHAVIOR: u32 = 1257;
pub const SQL_ATTR_CLOSEOPEN: u32 = 1265;
pub const SQL_ATTR_CURRENT_PACKAGE_SET: u32 = 1276;
pub const SQL_ATTR_DEFERRED_PREPARE: u32 = 1277;
pub const SQL_ATTR_EARLYCLOSE: u32 = 1268;
pub const SQL_ATTR_PROCESSCTL: u32 = 1278;
pub const SQL_ATTR_PREFETCH: u32 = 1285;
pub const SQL_ATTR_ENABLE_IPD_SETTING: u32 = 1286;
pub const SQL_ATTR_RETRYONERROR: u32 = 121;
pub const SQL_DESC_DESCRIPTOR_TYPE: u32 = 1287;
pub const SQL_ATTR_OPTIMIZE_SQLCOLUMNS: u32 = 1288;
pub const SQL_ATTR_MEM_DEBUG_DUMP: u32 = 1289;
pub const SQL_ATTR_CONNECT_NODE: u32 = 1290;
pub const SQL_ATTR_CONNECT_WITH_XA: u32 = 1291;
pub const SQL_ATTR_GET_XA_RESOURCE: u32 = 1292;
pub const SQL_ATTR_DB2_SQLERRP: u32 = 2451;
pub const SQL_ATTR_SERVER_MSGTXT_SP: u32 = 2452;
pub const SQL_ATTR_OPTIMIZE_FOR_NROWS: u32 = 2450;
pub const SQL_ATTR_QUERY_OPTIMIZATION_LEVEL: u32 = 1293;
pub const SQL_ATTR_USE_LIGHT_OUTPUT_SQLDA: u32 = 1298;
pub const SQL_ATTR_CURSOR_BLOCK_NUM_ROWS: u32 = 2453;
pub const SQL_ATTR_CURSOR_BLOCK_EARLY_CLOSE: u32 = 2454;
pub const SQL_ATTR_SERVER_MSGTXT_MASK: u32 = 2455;
pub const SQL_ATTR_USE_LIGHT_INPUT_SQLDA: u32 = 2458;
pub const SQL_ATTR_BLOCK_FOR_NROWS: u32 = 2459;
pub const SQL_ATTR_OPTIMIZE_ROWS_FOR_BLOCKING: u32 = 2460;
pub const SQL_ATTR_STATICMODE: u32 = 2467;
pub const SQL_ATTR_DB2_MESSAGE_PREFIX: u32 = 2468;
pub const SQL_ATTR_CALL_RETVAL_AS_PARM: u32 = 2469;
pub const SQL_ATTR_CALL_RETURN: u32 = 2470;
pub const SQL_ATTR_RETURN_USER_DEFINED_TYPES: u32 = 2471;
pub const SQL_ATTR_ENABLE_EXTENDED_PARAMDATA: u32 = 2472;
pub const SQL_ATTR_APP_TYPE: u32 = 2473;
pub const SQL_ATTR_TRANSFORM_GROUP: u32 = 2474;
pub const SQL_ATTR_DESCRIBE_CALL: u32 = 2476;
pub const SQL_ATTR_AUTOCOMMCLEANUP: u32 = 2477;
pub const SQL_ATTR_USEMALLOC: u32 = 2478;
pub const SQL_ATTR_PRESERVE_LOCALE: u32 = 2479;
pub const SQL_ATTR_MAPGRAPHIC: u32 = 2480;
pub const SQL_ATTR_INSERT_BUFFERING: u32 = 2481;
pub const SQL_ATTR_USE_LOAD_API: u32 = 2482;
pub const SQL_ATTR_LOAD_RECOVERABLE: u32 = 2483;
pub const SQL_ATTR_LOAD_COPY_LOCATION: u32 = 2484;
pub const SQL_ATTR_LOAD_MESSAGE_FILE: u32 = 2485;
pub const SQL_ATTR_LOAD_SAVECOUNT: u32 = 2486;
pub const SQL_ATTR_LOAD_CPU_PARALLELISM: u32 = 2487;
pub const SQL_ATTR_LOAD_DISK_PARALLELISM: u32 = 2488;
pub const SQL_ATTR_LOAD_INDEXING_MODE: u32 = 2489;
pub const SQL_ATTR_LOAD_STATS_MODE: u32 = 2490;
pub const SQL_ATTR_LOAD_TEMP_FILES_PATH: u32 = 2491;
pub const SQL_ATTR_LOAD_DATA_BUFFER_SIZE: u32 = 2492;
pub const SQL_ATTR_LOAD_MODIFIED_BY: u32 = 2493;
pub const SQL_ATTR_DB2_RESERVED_2494: u32 = 2494;
pub const SQL_ATTR_DESCRIBE_BEHAVIOR: u32 = 2495;
pub const SQL_ATTR_FETCH_SENSITIVITY: u32 = 2496;
pub const SQL_ATTR_DB2_RESERVED_2497: u32 = 2497;
pub const SQL_ATTR_CLIENT_LOB_BUFFERING: u32 = 2498;
pub const SQL_ATTR_SKIP_TRACE: u32 = 2499;
pub const SQL_ATTR_LOAD_INFO: u32 = 2501;
pub const SQL_ATTR_DESCRIBE_INPUT_ON_PREPARE: u32 = 2505;
pub const SQL_ATTR_DESCRIBE_OUTPUT_LEVEL: u32 = 2506;
pub const SQL_ATTR_CURRENT_PACKAGE_PATH: u32 = 2509;
pub const SQL_ATTR_INFO_PROGRAMID: u32 = 2511;
pub const SQL_ATTR_INFO_PROGRAMNAME: u32 = 2516;
pub const SQL_ATTR_FREE_LOCATORS_ON_FETCH: u32 = 2518;
pub const SQL_ATTR_KEEP_DYNAMIC: u32 = 2522;
pub const SQL_ATTR_LOAD_ROWS_READ_PTR: u32 = 2524;
pub const SQL_ATTR_LOAD_ROWS_SKIPPED_PTR: u32 = 2525;
pub const SQL_ATTR_LOAD_ROWS_COMMITTED_PTR: u32 = 2526;
pub const SQL_ATTR_LOAD_ROWS_LOADED_PTR: u32 = 2527;
pub const SQL_ATTR_LOAD_ROWS_REJECTED_PTR: u32 = 2528;
pub const SQL_ATTR_LOAD_ROWS_DELETED_PTR: u32 = 2529;
pub const SQL_ATTR_LOAD_INFO_VER: u32 = 2530;
pub const SQL_ATTR_SET_SSA: u32 = 2531;
pub const SQL_ATTR_BLOCK_LOBS: u32 = 2534;
pub const SQL_ATTR_LOAD_ACCESS_LEVEL: u32 = 2536;
pub const SQL_ATTR_MAPCHAR: u32 = 2546;
pub const SQL_ATTR_ARM_CORRELATOR: u32 = 2554;
pub const SQL_ATTR_CLIENT_DEBUGINFO: u32 = 2556;
pub const SQL_ATTR_GET_GENERATED_VALUE: u32 = 2583;
pub const SQL_ATTR_GET_SERIAL_VALUE: u32 = 2584;
pub const SQL_ATTR_INTERLEAVED_PUTDATA: u32 = 2591;
pub const SQL_ATTR_FORCE_ROLLBACK: u32 = 2596;
pub const SQL_ATTR_STMT_CONCENTRATOR: u32 = 2597;
pub const SQL_ATTR_LOAD_REPLACE_OPTION: u32 = 3036;
pub const SQL_ATTR_SESSION_GLOBAL_VAR: u32 = 3044;
pub const SQL_ATTR_SPECIAL_REGISTER: u32 = 3049;
pub const SQL_STMT_CONCENTRATOR_OFF: u32 = 1;
pub const SQL_STMT_CONCENTRATOR_WITH_LITERALS: u32 = 2;
pub const SQL_INFO_LAST: u32 = 174;
pub const SQL_INFO_DRIVER_START: u32 = 1000;
pub const SQL_FORCE_ROLLBACK_ON: u32 = 1;
pub const SQL_FORCE_ROLLBACK_OFF: u32 = 0;
pub const SQL_FORCE_ROLLBACK_DEFAULT: u32 = 0;
pub const SQL_DESCRIBE_NONE: u32 = 0;
pub const SQL_DESCRIBE_LIGHT: u32 = 1;
pub const SQL_DESCRIBE_REGULAR: u32 = 2;
pub const SQL_DESCRIBE_EXTENDED: u32 = 3;
pub const SQL_USE_LOAD_OFF: u32 = 0;
pub const SQL_USE_LOAD_INSERT: u32 = 1;
pub const SQL_USE_LOAD_REPLACE: u32 = 2;
pub const SQL_USE_LOAD_RESTART: u32 = 3;
pub const SQL_USE_LOAD_TERMINATE: u32 = 4;
pub const SQL_LOAD_REPLACE_DEFAULT: u32 = 0;
pub const SQL_LOAD_KEEPDICTIONARY: u32 = 1;
pub const SQL_LOAD_RESETDICTIONARY: u32 = 2;
pub const SQL_LOAD_RESETDICTIONARYONLY: u32 = 3;
pub const SQL_PREFETCH_ON: u32 = 1;
pub const SQL_PREFETCH_OFF: u32 = 0;
pub const SQL_PREFETCH_DEFAULT: u32 = 0;
pub const SQL_CC_NO_RELEASE: u32 = 0;
pub const SQL_CC_RELEASE: u32 = 1;
pub const SQL_CC_DEFAULT: u32 = 0;
pub const SQL_RETRYONERROR_OFF: u32 = 0;
pub const SQL_RETRYONERROR_ON: u32 = 1;
pub const SQL_RETRYONERROR_DEFAULT: u32 = 1;
pub const SQL_RETRYBINDONERROR_OFF: u32 = 0;
pub const SQL_RETRYBINDONERROR_ON: u32 = 1;
pub const SQL_RETRYBINDONERROR_DEFAULT: u32 = 1;
pub const SQL_ALLOW_INTERLEAVED_GETDATA_OFF: u32 = 0;
pub const SQL_ALLOW_INTERLEAVED_GETDATA_ON: u32 = 1;
pub const SQL_ALLOW_INTERLEAVED_GETDATA_DEFAULT: u32 = 0;
pub const SQL_INTERLEAVED_STREAM_PUTDATA_OFF: u32 = 0;
pub const SQL_INTERLEAVED_STREAM_PUTDATA_ON: u32 = 1;
pub const SQL_OVERRIDE_CODEPAGE_ON: u32 = 1;
pub const SQL_OVERRIDE_CODEPAGE_OFF: u32 = 0;
pub const SQL_DEFERRED_PREPARE_ON: u32 = 1;
pub const SQL_DEFERRED_PREPARE_OFF: u32 = 0;
pub const SQL_DEFERRED_PREPARE_DEFAULT: u32 = 1;
pub const SQL_EARLYCLOSE_ON: u32 = 1;
pub const SQL_EARLYCLOSE_OFF: u32 = 0;
pub const SQL_EARLYCLOSE_SERVER: u32 = 2;
pub const SQL_EARLYCLOSE_DEFAULT: u32 = 1;
pub const SQL_APP_TYPE_ODBC: u32 = 1;
pub const SQL_APP_TYPE_OLEDB: u32 = 2;
pub const SQL_APP_TYPE_JDBC: u32 = 3;
pub const SQL_APP_TYPE_ADONET: u32 = 4;
pub const SQL_APP_TYPE_DRDAWRAPPER: u32 = 5;
pub const SQL_APP_TYPE_OCI: u32 = 6;
pub const SQL_APP_TYPE_DEFAULT: u32 = 1;
pub const SQL_PROCESSCTL_NOTHREAD: u32 = 1;
pub const SQL_PROCESSCTL_NOFORK: u32 = 2;
pub const SQL_PROCESSCTL_SHARESTMTDESC: u32 = 4;
pub const SQL_PROCESSCTL_MULTICONNECT3: u32 = 8;
pub const SQL_FALSE: u32 = 0;
pub const SQL_TRUE: u32 = 1;
pub const SQL_CURSOR_HOLD_ON: u32 = 1;
pub const SQL_CURSOR_HOLD_OFF: u32 = 0;
pub const SQL_CURSOR_HOLD_DEFAULT: u32 = 1;
pub const SQL_NODESCRIBE_ON: u32 = 1;
pub const SQL_NODESCRIBE_OFF: u32 = 0;
pub const SQL_NODESCRIBE_DEFAULT: u32 = 0;
pub const SQL_DESCRIBE_CALL_NEVER: u32 = 0;
pub const SQL_DESCRIBE_CALL_BEFORE: u32 = 1;
pub const SQL_DESCRIBE_CALL_ON_ERROR: u32 = 2;
pub const SQL_DESCRIBE_CALL_DEFAULT: i32 = -1;
pub const SQL_CLIENTLOB_USE_LOCATORS: u32 = 0;
pub const SQL_CLIENTLOB_BUFFER_UNBOUND_LOBS: u32 = 1;
pub const SQL_CLIENTLOB_DEFAULT: u32 = 0;
pub const SQL_CLIENT_ENCALG_NOT_SET: u32 = 0;
pub const SQL_CLIENT_ENCALG_ANY: u32 = 1;
pub const SQL_CLIENT_ENCALG_AES_ONLY: u32 = 2;
pub const SQL_COMMITONEOF_OFF: u32 = 0;
pub const SQL_COMMITONEOF_ON: u32 = 1;
pub const SQL_WCHARTYPE: u32 = 1252;
pub const SQL_LONGDATA_COMPAT: u32 = 1253;
pub const SQL_CURRENT_SCHEMA: u32 = 1254;
pub const SQL_DB2EXPLAIN: u32 = 1258;
pub const SQL_DB2ESTIMATE: u32 = 1259;
pub const SQL_PARAMOPT_ATOMIC: u32 = 1260;
pub const SQL_STMTTXN_ISOLATION: u32 = 1261;
pub const SQL_MAXCONN: u32 = 1262;
pub const SQL_ATTR_CLISCHEMA: u32 = 1280;
pub const SQL_ATTR_INFO_USERID: u32 = 1281;
pub const SQL_ATTR_INFO_WRKSTNNAME: u32 = 1282;
pub const SQL_ATTR_INFO_APPLNAME: u32 = 1283;
pub const SQL_ATTR_INFO_ACCTSTR: u32 = 1284;
pub const SQL_ATTR_AUTOCOMMIT_NOCOMMIT: u32 = 2462;
pub const SQL_ATTR_QUERY_PATROLLER: u32 = 2466;
pub const SQL_ATTR_CHAINING_BEGIN: u32 = 2464;
pub const SQL_ATTR_CHAINING_END: u32 = 2465;
pub const SQL_ATTR_EXTENDEDBIND: u32 = 2475;
pub const SQL_ATTR_GRAPHIC_UNICODESERVER: u32 = 2503;
pub const SQL_ATTR_RETURN_CHAR_AS_WCHAR_OLEDB: u32 = 2517;
pub const SQL_ATTR_GATEWAY_CONNECTED: u32 = 2537;
pub const SQL_ATTR_SQLCOLUMNS_SORT_BY_ORDINAL_OLEDB: u32 = 2542;
pub const SQL_ATTR_REPORT_ISLONG_FOR_LONGTYPES_OLEDB: u32 = 2543;
pub const SQL_ATTR_PING_DB: u32 = 2545;
pub const SQL_ATTR_RECEIVE_TIMEOUT: u32 = 2547;
pub const SQL_ATTR_REOPT: u32 = 2548;
pub const SQL_ATTR_LOB_CACHE_SIZE: u32 = 2555;
pub const SQL_ATTR_STREAM_GETDATA: u32 = 2558;
pub const SQL_ATTR_APP_USES_LOB_LOCATOR: u32 = 2559;
pub const SQL_ATTR_MAX_LOB_BLOCK_SIZE: u32 = 2560;
pub const SQL_ATTR_USE_TRUSTED_CONTEXT: u32 = 2561;
pub const SQL_ATTR_TRUSTED_CONTEXT_USERID: u32 = 2562;
pub const SQL_ATTR_TRUSTED_CONTEXT_PASSWORD: u32 = 2563;
pub const SQL_ATTR_USER_REGISTRY_NAME: u32 = 2564;
pub const SQL_ATTR_DECFLOAT_ROUNDING_MODE: u32 = 2565;
pub const SQL_ATTR_APPEND_FOR_FETCH_ONLY: u32 = 2573;
pub const SQL_ATTR_ONLY_USE_BIG_PACKAGES: u32 = 2577;
pub const SQL_ATTR_NONATMOIC_BUFFER_INSERT: u32 = 2588;
pub const SQL_ATTR_ROWCOUNT_PREFETCH: u32 = 2592;
pub const SQL_ATTR_PING_REQUEST_PACKET_SIZE: u32 = 2593;
pub const SQL_ATTR_PING_NTIMES: u32 = 2594;
pub const SQL_ATTR_ALLOW_INTERLEAVED_GETDATA: u32 = 2599;
pub const SQL_ATTR_INTERLEAVED_STREAM_PUTDATA: u32 = 3000;
pub const SQL_ATTR_FET_BUF_SIZE: u32 = 3001;
pub const SQL_ATTR_CLIENT_CODEPAGE: u32 = 3002;
pub const SQL_ATTR_EXTENDED_INDICATORS: u32 = 3003;
pub const SQL_ATTR_SESSION_TIME_ZONE: u32 = 3004;
pub const SQL_ATTR_CLIENT_TIME_ZONE: u32 = 3005;
pub const SQL_ATTR_NETWORK_STATISTICS: u32 = 3006;
pub const SQL_ATTR_OVERRIDE_CHARACTER_CODEPAGE: u32 = 3007;
pub const SQL_ATTR_GET_LATEST_MEMBER: u32 = 3008;
pub const SQL_ATTR_CO_CAPTUREONPREPARE: u32 = 3009;
pub const SQL_ATTR_RETRYBINDONERROR: u32 = 3010;
pub const SQL_ATTR_COMMITONEOF: u32 = 3011;
pub const SQL_ATTR_PARC_BATCH: u32 = 3012;
pub const SQL_ATTR_COLUMNWISE_MRI: u32 = 3013;
pub const SQL_ATTR_OVERRIDE_CODEPAGE: u32 = 3014;
pub const SQL_ATTR_SQLCODEMAP: u32 = 3015;
pub const SQL_ATTR_ISREADONLYSQL: u32 = 3016;
pub const SQL_ATTR_DBC_SYS_NAMING: u32 = 3017;
pub const SQL_ATTR_FREE_MEMORY_ON_STMTCLOSE: u32 = 3018;
pub const SQL_ATTR_OVERRIDE_PRIMARY_AFFINITY: u32 = 3020;
pub const SQL_ATTR_STREAM_OUTPUTLOB_ON_CALL: u32 = 3021;
pub const SQL_ATTR_CACHE_USRLIBL: u32 = 3022;
pub const SQL_ATTR_GET_LATEST_MEMBER_NAME: u32 = 3023;
pub const SQL_ATTR_INFO_CRRTKN: u32 = 3024;
pub const SQL_ATTR_DATE_FMT: u32 = 3025;
pub const SQL_ATTR_DATE_SEP: u32 = 3026;
pub const SQL_ATTR_TIME_FMT: u32 = 3027;
pub const SQL_ATTR_TIME_SEP: u32 = 3028;
pub const SQL_ATTR_DECIMAL_SEP: u32 = 3029;
pub const SQL_ATTR_READ_ONLY_CONNECTION: u32 = 3030;
pub const SQL_ATTR_CONFIG_KEYWORDS_ARRAY_SIZE: u32 = 3031;
pub const SQL_ATTR_CONFIG_KEYWORDS_MAXLEN: u32 = 3032;
pub const SQL_ATTR_RETRY_ON_MERGE: u32 = 3033;
pub const SQL_ATTR_DETECT_READ_ONLY_TXN: u32 = 3034;
pub const SQL_ATTR_IGNORE_SERVER_LIST: u32 = 3035;
pub const SQL_ATTR_DB2ZLOAD_LOADSTMT: u32 = 3037;
pub const SQL_ATTR_DB2ZLOAD_RECDELIM: u32 = 3038;
pub const SQL_ATTR_DB2ZLOAD_BEGIN: u32 = 3039;
pub const SQL_ATTR_DB2ZLOAD_END: u32 = 3040;
pub const SQL_ATTR_DB2ZLOAD_FILETYPE: u32 = 3041;
pub const SQL_ATTR_DB2ZLOAD_MSGFILE: u32 = 3042;
pub const SQL_ATTR_DB2ZLOAD_UTILITYID: u32 = 3043;
pub const SQL_ATTR_CONNECT_PASSIVE: u32 = 3045;
pub const SQL_ATTR_CLIENT_APPLCOMPAT: u32 = 3046;
pub const SQL_ATTR_DB2ZLOAD_LOADFILE: u32 = 3047;
pub const SQL_ATTR_PREFETCH_NROWS: u32 = 3048;
pub const SQL_ATTR_CLIENT_USERID: u32 = 1281;
pub const SQL_ATTR_CLIENT_WRKSTNNAME: u32 = 1282;
pub const SQL_ATTR_CLIENT_APPLNAME: u32 = 1283;
pub const SQL_ATTR_CLIENT_ACCTSTR: u32 = 1284;
pub const SQL_ATTR_CLIENT_PROGINFO: u32 = 2516;
pub const SQL_DM_DROP_MODULE: u32 = 1;
pub const SQL_DM_RESTRICT: u32 = 2;
pub const SQL_MU_PROCEDURE_INVOCATION: u32 = 1;
pub const SQL_CM_CREATE_MODULE: u32 = 1;
pub const SQL_CM_AUTHORIZATION: u32 = 2;
pub const SQL_ATTR_WCHARTYPE: u32 = 1252;
pub const SQL_ATTR_LONGDATA_COMPAT: u32 = 1253;
pub const SQL_ATTR_CURRENT_SCHEMA: u32 = 1254;
pub const SQL_ATTR_DB2EXPLAIN: u32 = 1258;
pub const SQL_ATTR_DB2ESTIMATE: u32 = 1259;
pub const SQL_ATTR_PARAMOPT_ATOMIC: u32 = 1260;
pub const SQL_ATTR_STMTTXN_ISOLATION: u32 = 1261;
pub const SQL_ATTR_MAXCONN: u32 = 1262;
pub const SQL_CONNECTTYPE: u32 = 1255;
pub const SQL_SYNC_POINT: u32 = 1256;
pub const SQL_MINMEMORY_USAGE: u32 = 1263;
pub const SQL_CONN_CONTEXT: u32 = 1269;
pub const SQL_ATTR_INHERIT_NULL_CONNECT: u32 = 1270;
pub const SQL_ATTR_FORCE_CONVERSION_ON_CLIENT: u32 = 1275;
pub const SQL_ATTR_INFO_KEYWORDLIST: u32 = 2500;
pub const SQL_ATTR_DISABLE_SYSPLEX: u32 = 2590;
pub const SQL_ATTR_CONNECTTYPE: u32 = 1255;
pub const SQL_ATTR_SYNC_POINT: u32 = 1256;
pub const SQL_ATTR_MINMEMORY_USAGE: u32 = 1263;
pub const SQL_ATTR_CONN_CONTEXT: u32 = 1269;
pub const SQL_LD_COMPAT_YES: u32 = 1;
pub const SQL_LD_COMPAT_NO: u32 = 0;
pub const SQL_LD_COMPAT_DEFAULT: u32 = 0;
pub const SQL_ATTR_EXTENDEDBIND_COPY: u32 = 1;
pub const SQL_ATTR_EXTENDEDBIND_NOCOPY: u32 = 0;
pub const SQL_ATTR_EXTENDEDBIND_DEFAULT: u32 = 0;
pub const SQL_NC_HIGH: u32 = 0;
pub const SQL_NC_LOW: u32 = 1;
pub const SQL_PARC_BATCH_ENABLE: u32 = 1;
pub const SQL_PARC_BATCH_DISABLE: u32 = 0;
pub const SQL_SQLCODEMAP_NOMAP: u32 = 1;
pub const SQL_SQLCODEMAP_MAP: u32 = 2;
pub const SQL_CONNECT_PASSIVE_YES: u32 = 1;
pub const SQL_CONNECT_PASSIVE_NO: u32 = 0;
pub const SQL_CONNECT_PASSIVE_DEFAULT: u32 = 0;
pub const CLI_MAX_LONGVARCHAR: u32 = 1250;
pub const CLI_MAX_VARCHAR: u32 = 1251;
pub const CLI_MAX_CHAR: u32 = 1252;
pub const CLI_MAX_LONGVARGRAPHIC: u32 = 1253;
pub const CLI_MAX_VARGRAPHIC: u32 = 1254;
pub const CLI_MAX_GRAPHIC: u32 = 1255;
pub const SQL_DIAG_MESSAGE_TEXT_PTR: u32 = 2456;
pub const SQL_DIAG_LINE_NUMBER: u32 = 2461;
pub const SQL_DIAG_ERRMC: u32 = 2467;
pub const SQL_DIAG_SQLCA: u32 = 3037;
pub const SQL_DIAG_BYTES_PROCESSED: u32 = 2477;
pub const SQL_DIAG_RELATIVE_COST_ESTIMATE: u32 = 2504;
pub const SQL_DIAG_ROW_COUNT_ESTIMATE: u32 = 2507;
pub const SQL_DIAG_ELAPSED_SERVER_TIME: u32 = 2538;
pub const SQL_DIAG_ELAPSED_NETWORK_TIME: u32 = 2539;
pub const SQL_DIAG_ACCUMULATED_SERVER_TIME: u32 = 2540;
pub const SQL_DIAG_ACCUMULATED_NETWORK_TIME: u32 = 2541;
pub const SQL_DIAG_QUIESCE: u32 = 2549;
pub const SQL_DIAG_TOLERATED_ERROR: u32 = 2559;
pub const SQL_DIAG_NETWORK_STATISTICS: u32 = 2560;
pub const SQL_DIAG_QUIESCE_NO: u32 = 0;
pub const SQL_DIAG_QUIESCE_DATABASE: u32 = 1;
pub const SQL_DIAG_QUIESCE_INSTANCE: u32 = 2;
pub const SQL_ATTR_LITTLE_ENDIAN_UNICODE: u32 = 2457;
pub const SQL_ATTR_DIAGLEVEL: u32 = 2574;
pub const SQL_ATTR_NOTIFYLEVEL: u32 = 2575;
pub const SQL_ATTR_DIAGPATH: u32 = 2576;
pub const SQL_ATTR_MESSAGE_LINE_LENGTH: u32 = 2580;
pub const SQL_ATTR_ENABLE_IFXENV: u32 = 2585;
pub const SQL_ATTR_TRACENOHEADER: u32 = 2598;
pub const SQL_ATTR_DB2TRC_STARTUP_SIZE: u32 = 3019;
pub const SQL_ATOMIC_YES: u32 = 1;
pub const SQL_ATOMIC_NO: u32 = 0;
pub const SQL_ATOMIC_DEFAULT: u32 = 1;
pub const SQL_CONCURRENT_TRANS: u32 = 1;
pub const SQL_COORDINATED_TRANS: u32 = 2;
pub const SQL_CONNECTTYPE_DEFAULT: u32 = 1;
pub const SQL_ONEPHASE: u32 = 1;
pub const SQL_TWOPHASE: u32 = 2;
pub const SQL_SYNCPOINT_DEFAULT: u32 = 1;
pub const SQL_DB2ESTIMATE_ON: u32 = 1;
pub const SQL_DB2ESTIMATE_OFF: u32 = 0;
pub const SQL_DB2ESTIMATE_DEFAULT: u32 = 0;
pub const SQL_DB2EXPLAIN_OFF: u32 = 0;
pub const SQL_DB2EXPLAIN_SNAPSHOT_ON: u32 = 1;
pub const SQL_DB2EXPLAIN_MODE_ON: u32 = 2;
pub const SQL_DB2EXPLAIN_SNAPSHOT_MODE_ON: u32 = 3;
pub const SQL_DB2EXPLAIN_ON: u32 = 1;
pub const SQL_DB2EXPLAIN_DEFAULT: u32 = 0;
pub const SQL_WCHARTYPE_NOCONVERT: u32 = 0;
pub const SQL_WCHARTYPE_DEFAULT: u32 = 0;
pub const SQL_OPTIMIZE_SQLCOLUMNS_OFF: u32 = 0;
pub const SQL_OPTIMIZE_SQLCOLUMNS_ON: u32 = 1;
pub const SQL_OPTIMIZE_SQLCOLUMNS_DEFAULT: u32 = 0;
pub const SQL_CONNECT_WITH_XA_OFF: u32 = 0;
pub const SQL_CONNECT_WITH_XA_ON: u32 = 1;
pub const SQL_CONNECT_WITH_XA_DEFAULT: u32 = 0;
pub const SQL_ATTR_SERVER_MSGTXT_MASK_LOCAL_FIRST: u32 = 0;
pub const SQL_ATTR_SERVER_MSGTXT_MASK_WARNINGS: u32 = 1;
pub const SQL_ATTR_SERVER_MSGTXT_MASK_ERRORS: u32 = 4294967294;
pub const SQL_ATTR_SERVER_MSGTXT_MASK_ALL: u32 = 4294967295;
pub const SQL_ATTR_SERVER_MSGTXT_MASK_DEFAULT: u32 = 0;
pub const SQL_ATTR_QUERY_PATROLLER_DISABLE: u32 = 1;
pub const SQL_ATTR_QUERY_PATROLLER_ENABLE: u32 = 2;
pub const SQL_ATTR_QUERY_PATROLLER_BYPASS: u32 = 3;
pub const SQL_STATICMODE_DISABLED: u32 = 0;
pub const SQL_STATICMODE_CAPTURE: u32 = 1;
pub const SQL_STATICMODE_MATCH: u32 = 2;
pub const SQL_ATTR_DB2_MESSAGE_PREFIX_OFF: u32 = 0;
pub const SQL_ATTR_DB2_MESSAGE_PREFIX_ON: u32 = 1;
pub const SQL_ATTR_DB2_MESSAGE_PREFIX_DEFAULT: u32 = 1;
pub const SQL_ATTR_INSERT_BUFFERING_OFF: u32 = 0;
pub const SQL_ATTR_INSERT_BUFFERING_ON: u32 = 1;
pub const SQL_ATTR_INSERT_BUFFERING_IGD: u32 = 2;
pub const SQL_ROWCOUNT_PREFETCH_OFF: u32 = 0;
pub const SQL_ROWCOUNT_PREFETCH_ON: u32 = 1;
pub const SQL_SCOPE_CURROW: u32 = 0;
pub const SQL_SCOPE_TRANSACTION: u32 = 1;
pub const SQL_SCOPE_SESSION: u32 = 2;
pub const SQL_INDEX_UNIQUE: u32 = 0;
pub const SQL_INDEX_ALL: u32 = 1;
pub const SQL_INDEX_CLUSTERED: u32 = 1;
pub const SQL_INDEX_HASHED: u32 = 2;
pub const SQL_INDEX_OTHER: u32 = 3;
pub const SQL_PC_UNKNOWN: u32 = 0;
pub const SQL_PC_NON_PSEUDO: u32 = 1;
pub const SQL_PC_PSEUDO: u32 = 2;
pub const SQL_ROW_IDENTIFIER: u32 = 1;
pub const SQL_MAPGRAPHIC_DEFAULT: i32 = -1;
pub const SQL_MAPGRAPHIC_GRAPHIC: u32 = 0;
pub const SQL_MAPGRAPHIC_WCHAR: u32 = 1;
pub const SQL_MAPCHAR_DEFAULT: u32 = 0;
pub const SQL_MAPCHAR_WCHAR: u32 = 1;
pub const SQL_FETCH_NEXT: u32 = 1;
pub const SQL_FETCH_FIRST: u32 = 2;
pub const SQL_FETCH_LAST: u32 = 3;
pub const SQL_FETCH_PRIOR: u32 = 4;
pub const SQL_FETCH_ABSOLUTE: u32 = 5;
pub const SQL_FETCH_RELATIVE: u32 = 6;
pub const SQL_EXTENDED_INDICATOR_NOT_SET: u32 = 0;
pub const SQL_EXTENDED_INDICATOR_ENABLE: u32 = 1;
pub const SQL_EXTENDED_INDICATOR_DISABLE: u32 = 2;
pub const SQL_COLUMNWISE_MRI_ON: u32 = 1;
pub const SQL_COLUMNWISE_MRI_OFF: u32 = 0;
pub const SQL_ISREADONLYSQL_YES: u32 = 1;
pub const SQL_ISREADONLYSQL_NO: u32 = 0;
pub const SQL_FREE_MEMORY_ON_STMTCLOSE_YES: u32 = 1;
pub const SQL_FREE_MEMORY_ON_STMTCLOSE_NO: u32 = 0;
pub const SQL_ATTR_CACHE_USRLIBL_YES: u32 = 0;
pub const SQL_ATTR_CACHE_USRLIBL_NO: u32 = 1;
pub const SQL_ATTR_CACHE_USRLIBL_REFRESH: u32 = 2;
pub const SQL_IBMi_FMT_ISO: u32 = 1;
pub const SQL_IBMi_FMT_USA: u32 = 2;
pub const SQL_IBMi_FMT_EUR: u32 = 3;
pub const SQL_IBMi_FMT_JIS: u32 = 4;
pub const SQL_IBMi_FMT_MDY: u32 = 5;
pub const SQL_IBMi_FMT_DMY: u32 = 6;
pub const SQL_IBMi_FMT_YMD: u32 = 7;
pub const SQL_IBMi_FMT_JUL: u32 = 8;
pub const SQL_IBMi_FMT_HMS: u32 = 9;
pub const SQL_IBMi_FMT_JOB: u32 = 10;
pub const SQL_SEP_SLASH: u32 = 1;
pub const SQL_SEP_DASH: u32 = 2;
pub const SQL_SEP_PERIOD: u32 = 3;
pub const SQL_SEP_COMMA: u32 = 4;
pub const SQL_SEP_BLANK: u32 = 5;
pub const SQL_SEP_COLON: u32 = 6;
pub const SQL_SEP_JOB: u32 = 7;
pub const SQL_XML_DECLARATION_NONE: u32 = 0;
pub const SQL_XML_DECLARATION_BOM: u32 = 1;
pub const SQL_XML_DECLARATION_BASE: u32 = 2;
pub const SQL_XML_DECLARATION_ENCATTR: u32 = 4;
pub const SQL_DB2ZLOAD_RECDELIM_NONE: u32 = 0;
pub const SQL_DB2ZLOAD_RECDELIM_ALF: u32 = 1;
pub const SQL_DB2ZLOAD_RECDELIM_ENL: u32 = 2;
pub const SQL_DB2ZLOAD_RECDELIM_CRLF: u32 = 3;
pub const SQL_DB2ZLOAD_FILETYPE_DEL: u32 = 1;
pub const SQL_DB2ZLOAD_FILETYPE_INT: u32 = 2;
pub const SQL_DB2ZLOAD_FILETYPE_SPANNED: u32 = 3;
pub const DSD_ACR_AFFINITY: u32 = 1;
pub const SQL_ATTR_OUTPUT_NTS: u32 = 10001;
pub const SQL_FILE_READ: u32 = 2;
pub const SQL_FILE_CREATE: u32 = 8;
pub const SQL_FILE_OVERWRITE: u32 = 16;
pub const SQL_FILE_APPEND: u32 = 32;
pub const SQL_FROM_LOCATOR: u32 = 2;
pub const SQL_FROM_LITERAL: u32 = 3;
pub const SQL_ROUND_HALF_EVEN: u32 = 0;
pub const SQL_ROUND_HALF_UP: u32 = 1;
pub const SQL_ROUND_DOWN: u32 = 2;
pub const SQL_ROUND_CEILING: u32 = 3;
pub const SQL_ROUND_FLOOR: u32 = 4;
pub const SQL_NETWORK_STATISTICS_ON_SKIP_NOSERVER: u32 = 2;
pub const SQL_NETWORK_STATISTICS_ON: u32 = 1;
pub const SQL_NETWORK_STATISTICS_OFF: u32 = 0;
pub const SQL_NETWORK_STATISTICS_DEFAULT: u32 = 0;
pub const SQL_READ_ONLY_CONNECTION_ON: u32 = 1;
pub const SQL_READ_ONLY_CONNECTION_OFF: u32 = 0;
pub const SQL_READ_ONLY_CONNECTION_DEFAULT: u32 = 0;
pub const SQL_UNASSIGNED: i32 = -7;
pub const SQL_DETECT_READ_ONLY_TXN_ENABLE: u32 = 1;
pub const SQL_DETECT_READ_ONLY_TXN_DISABLE: u32 = 0;
pub const SQL_C_WCHAR: i32 = -8;
pub const SQL_C_TCHAR: u32 = 1;
#[doc = " Define fixed size integer types."]
pub type sqlint8 = ::std::os::raw::c_char;
pub type sqluint8 = ::std::os::raw::c_uchar;
pub type sqlint16 = ::std::os::raw::c_short;
pub type sqluint16 = ::std::os::raw::c_ushort;
pub type sqlint32 = ::std::os::raw::c_int;
pub type sqluint32 = ::std::os::raw::c_uint;
pub type sqlint64 = ::std::os::raw::c_long;
pub type sqluint64 = ::std::os::raw::c_ulong;
pub type sqlintptr = sqlint64;
pub type sqluintptr = sqluint64;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sqlca {
    pub sqlcaid: [::std::os::raw::c_char; 8usize],
    pub sqlcabc: sqlint32,
    pub sqlcode: sqlint32,
    pub sqlerrml: ::std::os::raw::c_short,
    pub sqlerrmc: [::std::os::raw::c_char; 70usize],
    pub sqlerrp: [::std::os::raw::c_char; 8usize],
    pub sqlerrd: [sqlint32; 6usize],
    pub sqlwarn: [::std::os::raw::c_char; 11usize],
    pub sqlstate: [::std::os::raw::c_char; 5usize],
}
#[test]
fn bindgen_test_layout_sqlca() {
    assert_eq!(
        ::std::mem::size_of::<sqlca>(),
        136usize,
        concat!("Size of: ", stringify!(sqlca))
    );
    assert_eq!(
        ::std::mem::align_of::<sqlca>(),
        4usize,
        concat!("Alignment of ", stringify!(sqlca))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<sqlca>())).sqlcaid as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(sqlca),
            "::",
            stringify!(sqlcaid)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<sqlca>())).sqlcabc as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(sqlca),
            "::",
            stringify!(sqlcabc)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<sqlca>())).sqlcode as *const _ as usize },
        12usize,
        concat!(
            "Offset of field: ",
            stringify!(sqlca),
            "::",
            stringify!(sqlcode)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<sqlca>())).sqlerrml as *const _ as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(sqlca),
            "::",
            stringify!(sqlerrml)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<sqlca>())).sqlerrmc as *const _ as usize },
        18usize,
        concat!(
            "Offset of field: ",
            stringify!(sqlca),
            "::",
            stringify!(sqlerrmc)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<sqlca>())).sqlerrp as *const _ as usize },
        88usize,
        concat!(
            "Offset of field: ",
            stringify!(sqlca),
            "::",
            stringify!(sqlerrp)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<sqlca>())).sqlerrd as *const _ as usize },
        96usize,
        concat!(
            "Offset of field: ",
            stringify!(sqlca),
            "::",
            stringify!(sqlerrd)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<sqlca>())).sqlwarn as *const _ as usize },
        120usize,
        concat!(
            "Offset of field: ",
            stringify!(sqlca),
            "::",
            stringify!(sqlwarn)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<sqlca>())).sqlstate as *const _ as usize },
        131usize,
        concat!(
            "Offset of field: ",
            stringify!(sqlca),
            "::",
            stringify!(sqlstate)
        )
    );
}
pub type __int8_t = ::std::os::raw::c_schar;
pub type __uint8_t = ::std::os::raw::c_uchar;
pub type __int16_t = ::std::os::raw::c_short;
pub type __uint16_t = ::std::os::raw::c_ushort;
pub type __int32_t = ::std::os::raw::c_int;
pub type __uint32_t = ::std::os::raw::c_uint;
pub type __int64_t = ::std::os::raw::c_longlong;
pub type __uint64_t = ::std::os::raw::c_ulonglong;
pub type __darwin_intptr_t = ::std::os::raw::c_long;
pub type __darwin_natural_t = ::std::os::raw::c_uint;
pub type __darwin_ct_rune_t = ::std::os::raw::c_int;
#[repr(C)]
#[derive(Copy, Clone)]
pub union __mbstate_t {
    pub __mbstate8: [::std::os::raw::c_char; 128usize],
    pub _mbstateL: ::std::os::raw::c_longlong,
    _bindgen_union_align: [u64; 16usize],
}
#[test]
fn bindgen_test_layout___mbstate_t() {
    assert_eq!(
        ::std::mem::size_of::<__mbstate_t>(),
        128usize,
        concat!("Size of: ", stringify!(__mbstate_t))
    );
    assert_eq!(
        ::std::mem::align_of::<__mbstate_t>(),
        8usize,
        concat!("Alignment of ", stringify!(__mbstate_t))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<__mbstate_t>())).__mbstate8 as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(__mbstate_t),
            "::",
            stringify!(__mbstate8)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<__mbstate_t>()))._mbstateL as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(__mbstate_t),
            "::",
            stringify!(_mbstateL)
        )
    );
}
pub type __darwin_mbstate_t = __mbstate_t;
pub type __darwin_ptrdiff_t = ::std::os::raw::c_long;
pub type __darwin_size_t = ::std::os::raw::c_ulong;
pub type __darwin_va_list = __builtin_va_list;
pub type __darwin_wchar_t = ::std::os::raw::c_int;
pub type __darwin_rune_t = __darwin_wchar_t;
pub type __darwin_wint_t = ::std::os::raw::c_int;
pub type __darwin_clock_t = ::std::os::raw::c_ulong;
pub type __darwin_socklen_t = __uint32_t;
pub type __darwin_ssize_t = ::std::os::raw::c_long;
pub type __darwin_time_t = ::std::os::raw::c_long;
pub type __darwin_blkcnt_t = __int64_t;
pub type __darwin_blksize_t = __int32_t;
pub type __darwin_dev_t = __int32_t;
pub type __darwin_fsblkcnt_t = ::std::os::raw::c_uint;
pub type __darwin_fsfilcnt_t = ::std::os::raw::c_uint;
pub type __darwin_gid_t = __uint32_t;
pub type __darwin_id_t = __uint32_t;
pub type __darwin_ino64_t = __uint64_t;
pub type __darwin_ino_t = __darwin_ino64_t;
pub type __darwin_mach_port_name_t = __darwin_natural_t;
pub type __darwin_mach_port_t = __darwin_mach_port_name_t;
pub type __darwin_mode_t = __uint16_t;
pub type __darwin_off_t = __int64_t;
pub type __darwin_pid_t = __int32_t;
pub type __darwin_sigset_t = __uint32_t;
pub type __darwin_suseconds_t = __int32_t;
pub type __darwin_uid_t = __uint32_t;
pub type __darwin_useconds_t = __uint32_t;
pub type __darwin_uuid_t = [::std::os::raw::c_uchar; 16usize];
pub type __darwin_uuid_string_t = [::std::os::raw::c_char; 37usize];
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __darwin_pthread_handler_rec {
    pub __routine: ::std::option::Option<unsafe extern "C" fn(arg1: *mut ::std::os::raw::c_void)>,
    pub __arg: *mut ::std::os::raw::c_void,
    pub __next: *mut __darwin_pthread_handler_rec,
}
#[test]
fn bindgen_test_layout___darwin_pthread_handler_rec() {
    assert_eq!(
        ::std::mem::size_of::<__darwin_pthread_handler_rec>(),
        24usize,
        concat!("Size of: ", stringify!(__darwin_pthread_handler_rec))
    );
    assert_eq!(
        ::std::mem::align_of::<__darwin_pthread_handler_rec>(),
        8usize,
        concat!("Alignment of ", stringify!(__darwin_pthread_handler_rec))
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<__darwin_pthread_handler_rec>())).__routine as *const _ as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(__darwin_pthread_handler_rec),
            "::",
            stringify!(__routine)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<__darwin_pthread_handler_rec>())).__arg as *const _ as usize
        },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(__darwin_pthread_handler_rec),
            "::",
            stringify!(__arg)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<__darwin_pthread_handler_rec>())).__next as *const _ as usize
        },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(__darwin_pthread_handler_rec),
            "::",
            stringify!(__next)
        )
    );
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct _opaque_pthread_attr_t {
    pub __sig: ::std::os::raw::c_long,
    pub __opaque: [::std::os::raw::c_char; 56usize],
}
#[test]
fn bindgen_test_layout__opaque_pthread_attr_t() {
    assert_eq!(
        ::std::mem::size_of::<_opaque_pthread_attr_t>(),
        64usize,
        concat!("Size of: ", stringify!(_opaque_pthread_attr_t))
    );
    assert_eq!(
        ::std::mem::align_of::<_opaque_pthread_attr_t>(),
        8usize,
        concat!("Alignment of ", stringify!(_opaque_pthread_attr_t))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<_opaque_pthread_attr_t>())).__sig as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(_opaque_pthread_attr_t),
            "::",
            stringify!(__sig)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<_opaque_pthread_attr_t>())).__opaque as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(_opaque_pthread_attr_t),
            "::",
            stringify!(__opaque)
        )
    );
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct _opaque_pthread_cond_t {
    pub __sig: ::std::os::raw::c_long,
    pub __opaque: [::std::os::raw::c_char; 40usize],
}
#[test]
fn bindgen_test_layout__opaque_pthread_cond_t() {
    assert_eq!(
        ::std::mem::size_of::<_opaque_pthread_cond_t>(),
        48usize,
        concat!("Size of: ", stringify!(_opaque_pthread_cond_t))
    );
    assert_eq!(
        ::std::mem::align_of::<_opaque_pthread_cond_t>(),
        8usize,
        concat!("Alignment of ", stringify!(_opaque_pthread_cond_t))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<_opaque_pthread_cond_t>())).__sig as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(_opaque_pthread_cond_t),
            "::",
            stringify!(__sig)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<_opaque_pthread_cond_t>())).__opaque as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(_opaque_pthread_cond_t),
            "::",
            stringify!(__opaque)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _opaque_pthread_condattr_t {
    pub __sig: ::std::os::raw::c_long,
    pub __opaque: [::std::os::raw::c_char; 8usize],
}
#[test]
fn bindgen_test_layout__opaque_pthread_condattr_t() {
    assert_eq!(
        ::std::mem::size_of::<_opaque_pthread_condattr_t>(),
        16usize,
        concat!("Size of: ", stringify!(_opaque_pthread_condattr_t))
    );
    assert_eq!(
        ::std::mem::align_of::<_opaque_pthread_condattr_t>(),
        8usize,
        concat!("Alignment of ", stringify!(_opaque_pthread_condattr_t))
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<_opaque_pthread_condattr_t>())).__sig as *const _ as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(_opaque_pthread_condattr_t),
            "::",
            stringify!(__sig)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<_opaque_pthread_condattr_t>())).__opaque as *const _ as usize
        },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(_opaque_pthread_condattr_t),
            "::",
            stringify!(__opaque)
        )
    );
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct _opaque_pthread_mutex_t {
    pub __sig: ::std::os::raw::c_long,
    pub __opaque: [::std::os::raw::c_char; 56usize],
}
#[test]
fn bindgen_test_layout__opaque_pthread_mutex_t() {
    assert_eq!(
        ::std::mem::size_of::<_opaque_pthread_mutex_t>(),
        64usize,
        concat!("Size of: ", stringify!(_opaque_pthread_mutex_t))
    );
    assert_eq!(
        ::std::mem::align_of::<_opaque_pthread_mutex_t>(),
        8usize,
        concat!("Alignment of ", stringify!(_opaque_pthread_mutex_t))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<_opaque_pthread_mutex_t>())).__sig as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(_opaque_pthread_mutex_t),
            "::",
            stringify!(__sig)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<_opaque_pthread_mutex_t>())).__opaque as *const _ as usize
        },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(_opaque_pthread_mutex_t),
            "::",
            stringify!(__opaque)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _opaque_pthread_mutexattr_t {
    pub __sig: ::std::os::raw::c_long,
    pub __opaque: [::std::os::raw::c_char; 8usize],
}
#[test]
fn bindgen_test_layout__opaque_pthread_mutexattr_t() {
    assert_eq!(
        ::std::mem::size_of::<_opaque_pthread_mutexattr_t>(),
        16usize,
        concat!("Size of: ", stringify!(_opaque_pthread_mutexattr_t))
    );
    assert_eq!(
        ::std::mem::align_of::<_opaque_pthread_mutexattr_t>(),
        8usize,
        concat!("Alignment of ", stringify!(_opaque_pthread_mutexattr_t))
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<_opaque_pthread_mutexattr_t>())).__sig as *const _ as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(_opaque_pthread_mutexattr_t),
            "::",
            stringify!(__sig)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<_opaque_pthread_mutexattr_t>())).__opaque as *const _ as usize
        },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(_opaque_pthread_mutexattr_t),
            "::",
            stringify!(__opaque)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _opaque_pthread_once_t {
    pub __sig: ::std::os::raw::c_long,
    pub __opaque: [::std::os::raw::c_char; 8usize],
}
#[test]
fn bindgen_test_layout__opaque_pthread_once_t() {
    assert_eq!(
        ::std::mem::size_of::<_opaque_pthread_once_t>(),
        16usize,
        concat!("Size of: ", stringify!(_opaque_pthread_once_t))
    );
    assert_eq!(
        ::std::mem::align_of::<_opaque_pthread_once_t>(),
        8usize,
        concat!("Alignment of ", stringify!(_opaque_pthread_once_t))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<_opaque_pthread_once_t>())).__sig as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(_opaque_pthread_once_t),
            "::",
            stringify!(__sig)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<_opaque_pthread_once_t>())).__opaque as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(_opaque_pthread_once_t),
            "::",
            stringify!(__opaque)
        )
    );
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct _opaque_pthread_rwlock_t {
    pub __sig: ::std::os::raw::c_long,
    pub __opaque: [::std::os::raw::c_char; 192usize],
}
#[test]
fn bindgen_test_layout__opaque_pthread_rwlock_t() {
    assert_eq!(
        ::std::mem::size_of::<_opaque_pthread_rwlock_t>(),
        200usize,
        concat!("Size of: ", stringify!(_opaque_pthread_rwlock_t))
    );
    assert_eq!(
        ::std::mem::align_of::<_opaque_pthread_rwlock_t>(),
        8usize,
        concat!("Alignment of ", stringify!(_opaque_pthread_rwlock_t))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<_opaque_pthread_rwlock_t>())).__sig as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(_opaque_pthread_rwlock_t),
            "::",
            stringify!(__sig)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<_opaque_pthread_rwlock_t>())).__opaque as *const _ as usize
        },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(_opaque_pthread_rwlock_t),
            "::",
            stringify!(__opaque)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _opaque_pthread_rwlockattr_t {
    pub __sig: ::std::os::raw::c_long,
    pub __opaque: [::std::os::raw::c_char; 16usize],
}
#[test]
fn bindgen_test_layout__opaque_pthread_rwlockattr_t() {
    assert_eq!(
        ::std::mem::size_of::<_opaque_pthread_rwlockattr_t>(),
        24usize,
        concat!("Size of: ", stringify!(_opaque_pthread_rwlockattr_t))
    );
    assert_eq!(
        ::std::mem::align_of::<_opaque_pthread_rwlockattr_t>(),
        8usize,
        concat!("Alignment of ", stringify!(_opaque_pthread_rwlockattr_t))
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<_opaque_pthread_rwlockattr_t>())).__sig as *const _ as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(_opaque_pthread_rwlockattr_t),
            "::",
            stringify!(__sig)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<_opaque_pthread_rwlockattr_t>())).__opaque as *const _ as usize
        },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(_opaque_pthread_rwlockattr_t),
            "::",
            stringify!(__opaque)
        )
    );
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct _opaque_pthread_t {
    pub __sig: ::std::os::raw::c_long,
    pub __cleanup_stack: *mut __darwin_pthread_handler_rec,
    pub __opaque: [::std::os::raw::c_char; 8176usize],
}
#[test]
fn bindgen_test_layout__opaque_pthread_t() {
    assert_eq!(
        ::std::mem::size_of::<_opaque_pthread_t>(),
        8192usize,
        concat!("Size of: ", stringify!(_opaque_pthread_t))
    );
    assert_eq!(
        ::std::mem::align_of::<_opaque_pthread_t>(),
        8usize,
        concat!("Alignment of ", stringify!(_opaque_pthread_t))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<_opaque_pthread_t>())).__sig as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(_opaque_pthread_t),
            "::",
            stringify!(__sig)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<_opaque_pthread_t>())).__cleanup_stack as *const _ as usize
        },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(_opaque_pthread_t),
            "::",
            stringify!(__cleanup_stack)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<_opaque_pthread_t>())).__opaque as *const _ as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(_opaque_pthread_t),
            "::",
            stringify!(__opaque)
        )
    );
}
pub type __darwin_pthread_attr_t = _opaque_pthread_attr_t;
pub type __darwin_pthread_cond_t = _opaque_pthread_cond_t;
pub type __darwin_pthread_condattr_t = _opaque_pthread_condattr_t;
pub type __darwin_pthread_key_t = ::std::os::raw::c_ulong;
pub type __darwin_pthread_mutex_t = _opaque_pthread_mutex_t;
pub type __darwin_pthread_mutexattr_t = _opaque_pthread_mutexattr_t;
pub type __darwin_pthread_once_t = _opaque_pthread_once_t;
pub type __darwin_pthread_rwlock_t = _opaque_pthread_rwlock_t;
pub type __darwin_pthread_rwlockattr_t = _opaque_pthread_rwlockattr_t;
pub type __darwin_pthread_t = *mut _opaque_pthread_t;
pub type __darwin_nl_item = ::std::os::raw::c_int;
pub type __darwin_wctrans_t = ::std::os::raw::c_int;
pub type __darwin_wctype_t = __uint32_t;
pub const idtype_t_P_ALL: idtype_t = 0;
pub const idtype_t_P_PID: idtype_t = 1;
pub const idtype_t_P_PGID: idtype_t = 2;
pub type idtype_t = ::std::os::raw::c_uint;
pub type pid_t = __darwin_pid_t;
pub type id_t = __darwin_id_t;
pub type sig_atomic_t = ::std::os::raw::c_int;
pub type u_int8_t = ::std::os::raw::c_uchar;
pub type u_int16_t = ::std::os::raw::c_ushort;
pub type u_int32_t = ::std::os::raw::c_uint;
pub type u_int64_t = ::std::os::raw::c_ulonglong;
pub type register_t = i64;
pub type user_addr_t = u_int64_t;
pub type user_size_t = u_int64_t;
pub type user_ssize_t = i64;
pub type user_long_t = i64;
pub type user_ulong_t = u_int64_t;
pub type user_time_t = i64;
pub type user_off_t = i64;
pub type syscall_arg_t = u_int64_t;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __darwin_i386_thread_state {
    pub __eax: ::std::os::raw::c_uint,
    pub __ebx: ::std::os::raw::c_uint,
    pub __ecx: ::std::os::raw::c_uint,
    pub __edx: ::std::os::raw::c_uint,
    pub __edi: ::std::os::raw::c_uint,
    pub __esi: ::std::os::raw::c_uint,
    pub __ebp: ::std::os::raw::c_uint,
    pub __esp: ::std::os::raw::c_uint,
    pub __ss: ::std::os::raw::c_uint,
    pub __eflags: ::std::os::raw::c_uint,
    pub __eip: ::std::os::raw::c_uint,
    pub __cs: ::std::os::raw::c_uint,
    pub __ds: ::std::os::raw::c_uint,
    pub __es: ::std::os::raw::c_uint,
    pub __fs: ::std::os::raw::c_uint,
    pub __gs: ::std::os::raw::c_uint,
}
#[test]
fn bindgen_test_layout___darwin_i386_thread_state() {
    assert_eq!(
        ::std::mem::size_of::<__darwin_i386_thread_state>(),
        64usize,
        concat!("Size of: ", stringify!(__darwin_i386_thread_state))
    );
    assert_eq!(
        ::std::mem::align_of::<__darwin_i386_thread_state>(),
        4usize,
        concat!("Alignment of ", stringify!(__darwin_i386_thread_state))
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<__darwin_i386_thread_state>())).__eax as *const _ as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(__darwin_i386_thread_state),
            "::",
            stringify!(__eax)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<__darwin_i386_thread_state>())).__ebx as *const _ as usize
        },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(__darwin_i386_thread_state),
            "::",
            stringify!(__ebx)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<__darwin_i386_thread_state>())).__ecx as *const _ as usize
        },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(__darwin_i386_thread_state),
            "::",
            stringify!(__ecx)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<__darwin_i386_thread_state>())).__edx as *const _ as usize
        },
        12usize,
        concat!(
            "Offset of field: ",
            stringify!(__darwin_i386_thread_state),
            "::",
            stringify!(__edx)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<__darwin_i386_thread_state>())).__edi as *const _ as usize
        },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(__darwin_i386_thread_state),
            "::",
            stringify!(__edi)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<__darwin_i386_thread_state>())).__esi as *const _ as usize
        },
        20usize,
        concat!(
            "Offset of field: ",
            stringify!(__darwin_i386_thread_state),
            "::",
            stringify!(__esi)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<__darwin_i386_thread_state>())).__ebp as *const _ as usize
        },
        24usize,
        concat!(
            "Offset of field: ",
            stringify!(__darwin_i386_thread_state),
            "::",
            stringify!(__ebp)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<__darwin_i386_thread_state>())).__esp as *const _ as usize
        },
        28usize,
        concat!(
            "Offset of field: ",
            stringify!(__darwin_i386_thread_state),
            "::",
            stringify!(__esp)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<__darwin_i386_thread_state>())).__ss as *const _ as usize },
        32usize,
        concat!(
            "Offset of field: ",
            stringify!(__darwin_i386_thread_state),
            "::",
            stringify!(__ss)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<__darwin_i386_thread_state>())).__eflags as *const _ as usize
        },
        36usize,
        concat!(
            "Offset of field: ",
            stringify!(__darwin_i386_thread_state),
            "::",
            stringify!(__eflags)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<__darwin_i386_thread_state>())).__eip as *const _ as usize
        },
        40usize,
        concat!(
            "Offset of field: ",
            stringify!(__darwin_i386_thread_state),
            "::",
            stringify!(__eip)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<__darwin_i386_thread_state>())).__cs as *const _ as usize },
        44usize,
        concat!(
            "Offset of field: ",
            stringify!(__darwin_i386_thread_state),
            "::",
            stringify!(__cs)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<__darwin_i386_thread_state>())).__ds as *const _ as usize },
        48usize,
        concat!(
            "Offset of field: ",
            stringify!(__darwin_i386_thread_state),
            "::",
            stringify!(__ds)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<__darwin_i386_thread_state>())).__es as *const _ as usize },
        52usize,
        concat!(
            "Offset of field: ",
            stringify!(__darwin_i386_thread_state),
            "::",
            stringify!(__es)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<__darwin_i386_thread_state>())).__fs as *const _ as usize },
        56usize,
        concat!(
            "Offset of field: ",
            stringify!(__darwin_i386_thread_state),
            "::",
            stringify!(__fs)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<__darwin_i386_thread_state>())).__gs as *const _ as usize },
        60usize,
        concat!(
            "Offset of field: ",
            stringify!(__darwin_i386_thread_state),
            "::",
            stringify!(__gs)
        )
    );
}
#[repr(C)]
#[repr(align(2))]
#[derive(Debug, Copy, Clone)]
pub struct __darwin_fp_control {
    pub _bitfield_1: __BindgenBitfieldUnit<[u8; 2usize], u8>,
}
#[test]
fn bindgen_test_layout___darwin_fp_control() {
    assert_eq!(
        ::std::mem::size_of::<__darwin_fp_control>(),
        2usize,
        concat!("Size of: ", stringify!(__darwin_fp_control))
    );
    assert_eq!(
        ::std::mem::align_of::<__darwin_fp_control>(),
        2usize,
        concat!("Alignment of ", stringify!(__darwin_fp_control))
    );
}
impl __darwin_fp_control {
    #[inline]
    pub fn __invalid(&self) -> ::std::os::raw::c_ushort {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(0usize, 1u8) as u16) }
    }
    #[inline]
    pub fn set___invalid(&mut self, val: ::std::os::raw::c_ushort) {
        unsafe {
            let val: u16 = ::std::mem::transmute(val);
            self._bitfield_1.set(0usize, 1u8, val as u64)
        }
    }
    #[inline]
    pub fn __denorm(&self) -> ::std::os::raw::c_ushort {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(1usize, 1u8) as u16) }
    }
    #[inline]
    pub fn set___denorm(&mut self, val: ::std::os::raw::c_ushort) {
        unsafe {
            let val: u16 = ::std::mem::transmute(val);
            self._bitfield_1.set(1usize, 1u8, val as u64)
        }
    }
    #[inline]
    pub fn __zdiv(&self) -> ::std::os::raw::c_ushort {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(2usize, 1u8) as u16) }
    }
    #[inline]
    pub fn set___zdiv(&mut self, val: ::std::os::raw::c_ushort) {
        unsafe {
            let val: u16 = ::std::mem::transmute(val);
            self._bitfield_1.set(2usize, 1u8, val as u64)
        }
    }
    #[inline]
    pub fn __ovrfl(&self) -> ::std::os::raw::c_ushort {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(3usize, 1u8) as u16) }
    }
    #[inline]
    pub fn set___ovrfl(&mut self, val: ::std::os::raw::c_ushort) {
        unsafe {
            let val: u16 = ::std::mem::transmute(val);
            self._bitfield_1.set(3usize, 1u8, val as u64)
        }
    }
    #[inline]
    pub fn __undfl(&self) -> ::std::os::raw::c_ushort {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(4usize, 1u8) as u16) }
    }
    #[inline]
    pub fn set___undfl(&mut self, val: ::std::os::raw::c_ushort) {
        unsafe {
            let val: u16 = ::std::mem::transmute(val);
            self._bitfield_1.set(4usize, 1u8, val as u64)
        }
    }
    #[inline]
    pub fn __precis(&self) -> ::std::os::raw::c_ushort {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(5usize, 1u8) as u16) }
    }
    #[inline]
    pub fn set___precis(&mut self, val: ::std::os::raw::c_ushort) {
        unsafe {
            let val: u16 = ::std::mem::transmute(val);
            self._bitfield_1.set(5usize, 1u8, val as u64)
        }
    }
    #[inline]
    pub fn __pc(&self) -> ::std::os::raw::c_ushort {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(8usize, 2u8) as u16) }
    }
    #[inline]
    pub fn set___pc(&mut self, val: ::std::os::raw::c_ushort) {
        unsafe {
            let val: u16 = ::std::mem::transmute(val);
            self._bitfield_1.set(8usize, 2u8, val as u64)
        }
    }
    #[inline]
    pub fn __rc(&self) -> ::std::os::raw::c_ushort {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(10usize, 2u8) as u16) }
    }
    #[inline]
    pub fn set___rc(&mut self, val: ::std::os::raw::c_ushort) {
        unsafe {
            let val: u16 = ::std::mem::transmute(val);
            self._bitfield_1.set(10usize, 2u8, val as u64)
        }
    }
    #[inline]
    pub fn new_bitfield_1(
        __invalid: ::std::os::raw::c_ushort,
        __denorm: ::std::os::raw::c_ushort,
        __zdiv: ::std::os::raw::c_ushort,
        __ovrfl: ::std::os::raw::c_ushort,
        __undfl: ::std::os::raw::c_ushort,
        __precis: ::std::os::raw::c_ushort,
        __pc: ::std::os::raw::c_ushort,
        __rc: ::std::os::raw::c_ushort,
    ) -> __BindgenBitfieldUnit<[u8; 2usize], u8> {
        let mut __bindgen_bitfield_unit: __BindgenBitfieldUnit<[u8; 2usize], u8> =
            Default::default();
        __bindgen_bitfield_unit.set(0usize, 1u8, {
            let __invalid: u16 = unsafe { ::std::mem::transmute(__invalid) };
            __invalid as u64
        });
        __bindgen_bitfield_unit.set(1usize, 1u8, {
            let __denorm: u16 = unsafe { ::std::mem::transmute(__denorm) };
            __denorm as u64
        });
        __bindgen_bitfield_unit.set(2usize, 1u8, {
            let __zdiv: u16 = unsafe { ::std::mem::transmute(__zdiv) };
            __zdiv as u64
        });
        __bindgen_bitfield_unit.set(3usize, 1u8, {
            let __ovrfl: u16 = unsafe { ::std::mem::transmute(__ovrfl) };
            __ovrfl as u64
        });
        __bindgen_bitfield_unit.set(4usize, 1u8, {
            let __undfl: u16 = unsafe { ::std::mem::transmute(__undfl) };
            __undfl as u64
        });
        __bindgen_bitfield_unit.set(5usize, 1u8, {
            let __precis: u16 = unsafe { ::std::mem::transmute(__precis) };
            __precis as u64
        });
        __bindgen_bitfield_unit.set(8usize, 2u8, {
            let __pc: u16 = unsafe { ::std::mem::transmute(__pc) };
            __pc as u64
        });
        __bindgen_bitfield_unit.set(10usize, 2u8, {
            let __rc: u16 = unsafe { ::std::mem::transmute(__rc) };
            __rc as u64
        });
        __bindgen_bitfield_unit
    }
}
pub type __darwin_fp_control_t = __darwin_fp_control;
#[repr(C)]
#[repr(align(2))]
#[derive(Debug, Copy, Clone)]
pub struct __darwin_fp_status {
    pub _bitfield_1: __BindgenBitfieldUnit<[u8; 2usize], u8>,
}
#[test]
fn bindgen_test_layout___darwin_fp_status() {
    assert_eq!(
        ::std::mem::size_of::<__darwin_fp_status>(),
        2usize,
        concat!("Size of: ", stringify!(__darwin_fp_status))
    );
    assert_eq!(
        ::std::mem::align_of::<__darwin_fp_status>(),
        2usize,
        concat!("Alignment of ", stringify!(__darwin_fp_status))
    );
}
impl __darwin_fp_status {
    #[inline]
    pub fn __invalid(&self) -> ::std::os::raw::c_ushort {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(0usize, 1u8) as u16) }
    }
    #[inline]
    pub fn set___invalid(&mut self, val: ::std::os::raw::c_ushort) {
        unsafe {
            let val: u16 = ::std::mem::transmute(val);
            self._bitfield_1.set(0usize, 1u8, val as u64)
        }
    }
    #[inline]
    pub fn __denorm(&self) -> ::std::os::raw::c_ushort {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(1usize, 1u8) as u16) }
    }
    #[inline]
    pub fn set___denorm(&mut self, val: ::std::os::raw::c_ushort) {
        unsafe {
            let val: u16 = ::std::mem::transmute(val);
            self._bitfield_1.set(1usize, 1u8, val as u64)
        }
    }
    #[inline]
    pub fn __zdiv(&self) -> ::std::os::raw::c_ushort {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(2usize, 1u8) as u16) }
    }
    #[inline]
    pub fn set___zdiv(&mut self, val: ::std::os::raw::c_ushort) {
        unsafe {
            let val: u16 = ::std::mem::transmute(val);
            self._bitfield_1.set(2usize, 1u8, val as u64)
        }
    }
    #[inline]
    pub fn __ovrfl(&self) -> ::std::os::raw::c_ushort {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(3usize, 1u8) as u16) }
    }
    #[inline]
    pub fn set___ovrfl(&mut self, val: ::std::os::raw::c_ushort) {
        unsafe {
            let val: u16 = ::std::mem::transmute(val);
            self._bitfield_1.set(3usize, 1u8, val as u64)
        }
    }
    #[inline]
    pub fn __undfl(&self) -> ::std::os::raw::c_ushort {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(4usize, 1u8) as u16) }
    }
    #[inline]
    pub fn set___undfl(&mut self, val: ::std::os::raw::c_ushort) {
        unsafe {
            let val: u16 = ::std::mem::transmute(val);
            self._bitfield_1.set(4usize, 1u8, val as u64)
        }
    }
    #[inline]
    pub fn __precis(&self) -> ::std::os::raw::c_ushort {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(5usize, 1u8) as u16) }
    }
    #[inline]
    pub fn set___precis(&mut self, val: ::std::os::raw::c_ushort) {
        unsafe {
            let val: u16 = ::std::mem::transmute(val);
            self._bitfield_1.set(5usize, 1u8, val as u64)
        }
    }
    #[inline]
    pub fn __stkflt(&self) -> ::std::os::raw::c_ushort {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(6usize, 1u8) as u16) }
    }
    #[inline]
    pub fn set___stkflt(&mut self, val: ::std::os::raw::c_ushort) {
        unsafe {
            let val: u16 = ::std::mem::transmute(val);
            self._bitfield_1.set(6usize, 1u8, val as u64)
        }
    }
    #[inline]
    pub fn __errsumm(&self) -> ::std::os::raw::c_ushort {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(7usize, 1u8) as u16) }
    }
    #[inline]
    pub fn set___errsumm(&mut self, val: ::std::os::raw::c_ushort) {
        unsafe {
            let val: u16 = ::std::mem::transmute(val);
            self._bitfield_1.set(7usize, 1u8, val as u64)
        }
    }
    #[inline]
    pub fn __c0(&self) -> ::std::os::raw::c_ushort {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(8usize, 1u8) as u16) }
    }
    #[inline]
    pub fn set___c0(&mut self, val: ::std::os::raw::c_ushort) {
        unsafe {
            let val: u16 = ::std::mem::transmute(val);
            self._bitfield_1.set(8usize, 1u8, val as u64)
        }
    }
    #[inline]
    pub fn __c1(&self) -> ::std::os::raw::c_ushort {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(9usize, 1u8) as u16) }
    }
    #[inline]
    pub fn set___c1(&mut self, val: ::std::os::raw::c_ushort) {
        unsafe {
            let val: u16 = ::std::mem::transmute(val);
            self._bitfield_1.set(9usize, 1u8, val as u64)
        }
    }
    #[inline]
    pub fn __c2(&self) -> ::std::os::raw::c_ushort {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(10usize, 1u8) as u16) }
    }
    #[inline]
    pub fn set___c2(&mut self, val: ::std::os::raw::c_ushort) {
        unsafe {
            let val: u16 = ::std::mem::transmute(val);
            self._bitfield_1.set(10usize, 1u8, val as u64)
        }
    }
    #[inline]
    pub fn __tos(&self) -> ::std::os::raw::c_ushort {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(11usize, 3u8) as u16) }
    }
    #[inline]
    pub fn set___tos(&mut self, val: ::std::os::raw::c_ushort) {
        unsafe {
            let val: u16 = ::std::mem::transmute(val);
            self._bitfield_1.set(11usize, 3u8, val as u64)
        }
    }
    #[inline]
    pub fn __c3(&self) -> ::std::os::raw::c_ushort {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(14usize, 1u8) as u16) }
    }
    #[inline]
    pub fn set___c3(&mut self, val: ::std::os::raw::c_ushort) {
        unsafe {
            let val: u16 = ::std::mem::transmute(val);
            self._bitfield_1.set(14usize, 1u8, val as u64)
        }
    }
    #[inline]
    pub fn __busy(&self) -> ::std::os::raw::c_ushort {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(15usize, 1u8) as u16) }
    }
    #[inline]
    pub fn set___busy(&mut self, val: ::std::os::raw::c_ushort) {
        unsafe {
            let val: u16 = ::std::mem::transmute(val);
            self._bitfield_1.set(15usize, 1u8, val as u64)
        }
    }
    #[inline]
    pub fn new_bitfield_1(
        __invalid: ::std::os::raw::c_ushort,
        __denorm: ::std::os::raw::c_ushort,
        __zdiv: ::std::os::raw::c_ushort,
        __ovrfl: ::std::os::raw::c_ushort,
        __undfl: ::std::os::raw::c_ushort,
        __precis: ::std::os::raw::c_ushort,
        __stkflt: ::std::os::raw::c_ushort,
        __errsumm: ::std::os::raw::c_ushort,
        __c0: ::std::os::raw::c_ushort,
        __c1: ::std::os::raw::c_ushort,
        __c2: ::std::os::raw::c_ushort,
        __tos: ::std::os::raw::c_ushort,
        __c3: ::std::os::raw::c_ushort,
        __busy: ::std::os::raw::c_ushort,
    ) -> __BindgenBitfieldUnit<[u8; 2usize], u8> {
        let mut __bindgen_bitfield_unit: __BindgenBitfieldUnit<[u8; 2usize], u8> =
            Default::default();
        __bindgen_bitfield_unit.set(0usize, 1u8, {
            let __invalid: u16 = unsafe { ::std::mem::transmute(__invalid) };
            __invalid as u64
        });
        __bindgen_bitfield_unit.set(1usize, 1u8, {
            let __denorm: u16 = unsafe { ::std::mem::transmute(__denorm) };
            __denorm as u64
        });
        __bindgen_bitfield_unit.set(2usize, 1u8, {
            let __zdiv: u16 = unsafe { ::std::mem::transmute(__zdiv) };
            __zdiv as u64
        });
        __bindgen_bitfield_unit.set(3usize, 1u8, {
            let __ovrfl: u16 = unsafe { ::std::mem::transmute(__ovrfl) };
            __ovrfl as u64
        });
        __bindgen_bitfield_unit.set(4usize, 1u8, {
            let __undfl: u16 = unsafe { ::std::mem::transmute(__undfl) };
            __undfl as u64
        });
        __bindgen_bitfield_unit.set(5usize, 1u8, {
            let __precis: u16 = unsafe { ::std::mem::transmute(__precis) };
            __precis as u64
        });
        __bindgen_bitfield_unit.set(6usize, 1u8, {
            let __stkflt: u16 = unsafe { ::std::mem::transmute(__stkflt) };
            __stkflt as u64
        });
        __bindgen_bitfield_unit.set(7usize, 1u8, {
            let __errsumm: u16 = unsafe { ::std::mem::transmute(__errsumm) };
            __errsumm as u64
        });
        __bindgen_bitfield_unit.set(8usize, 1u8, {
            let __c0: u16 = unsafe { ::std::mem::transmute(__c0) };
            __c0 as u64
        });
        __bindgen_bitfield_unit.set(9usize, 1u8, {
            let __c1: u16 = unsafe { ::std::mem::transmute(__c1) };
            __c1 as u64
        });
        __bindgen_bitfield_unit.set(10usize, 1u8, {
            let __c2: u16 = unsafe { ::std::mem::transmute(__c2) };
            __c2 as u64
        });
        __bindgen_bitfield_unit.set(11usize, 3u8, {
            let __tos: u16 = unsafe { ::std::mem::transmute(__tos) };
            __tos as u64
        });
        __bindgen_bitfield_unit.set(14usize, 1u8, {
            let __c3: u16 = unsafe { ::std::mem::transmute(__c3) };
            __c3 as u64
        });
        __bindgen_bitfield_unit.set(15usize, 1u8, {
            let __busy: u16 = unsafe { ::std::mem::transmute(__busy) };
            __busy as u64
        });
        __bindgen_bitfield_unit
    }
}
pub type __darwin_fp_status_t = __darwin_fp_status;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __darwin_mmst_reg {
    pub __mmst_reg: [::std::os::raw::c_char; 10usize],
    pub __mmst_rsrv: [::std::os::raw::c_char; 6usize],
}
#[test]
fn bindgen_test_layout___darwin_mmst_reg() {
    assert_eq!(
        ::std::mem::size_of::<__darwin_mmst_reg>(),
        16usize,
        concat!("Size of: ", stringify!(__darwin_mmst_reg))
    );
    assert_eq!(
        ::std::mem::align_of::<__darwin_mmst_reg>(),
        1usize,
        concat!("Alignment of ", stringify!(__darwin_mmst_reg))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<__darwin_mmst_reg>())).__mmst_reg as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(__darwin_mmst_reg),
            "::",
            stringify!(__mmst_reg)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<__darwin_mmst_reg>())).__mmst_rsrv as *const _ as usize },
        10usize,
        concat!(
            "Offset of field: ",
            stringify!(__darwin_mmst_reg),
            "::",
            stringify!(__mmst_rsrv)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __darwin_xmm_reg {
    pub __xmm_reg: [::std::os::raw::c_char; 16usize],
}
#[test]
fn bindgen_test_layout___darwin_xmm_reg() {
    assert_eq!(
        ::std::mem::size_of::<__darwin_xmm_reg>(),
        16usize,
        concat!("Size of: ", stringify!(__darwin_xmm_reg))
    );
    assert_eq!(
        ::std::mem::align_of::<__darwin_xmm_reg>(),
        1usize,
        concat!("Alignment of ", stringify!(__darwin_xmm_reg))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<__darwin_xmm_reg>())).__xmm_reg as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(__darwin_xmm_reg),
            "::",
            stringify!(__xmm_reg)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __darwin_ymm_reg {
    pub __ymm_reg: [::std::os::raw::c_char; 32usize],
}
#[test]
fn bindgen_test_layout___darwin_ymm_reg() {
    assert_eq!(
        ::std::mem::size_of::<__darwin_ymm_reg>(),
        32usize,
        concat!("Size of: ", stringify!(__darwin_ymm_reg))
    );
    assert_eq!(
        ::std::mem::align_of::<__darwin_ymm_reg>(),
        1usize,
        concat!("Alignment of ", stringify!(__darwin_ymm_reg))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<__darwin_ymm_reg>())).__ymm_reg as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(__darwin_ymm_reg),
            "::",
            stringify!(__ymm_reg)
        )
    );
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct __darwin_zmm_reg {
    pub __zmm_reg: [::std::os::raw::c_char; 64usize],
}
#[test]
fn bindgen_test_layout___darwin_zmm_reg() {
    assert_eq!(
        ::std::mem::size_of::<__darwin_zmm_reg>(),
        64usize,
        concat!("Size of: ", stringify!(__darwin_zmm_reg))
    );
    assert_eq!(
        ::std::mem::align_of::<__darwin_zmm_reg>(),
        1usize,
        concat!("Alignment of ", stringify!(__darwin_zmm_reg))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<__darwin_zmm_reg>())).__zmm_reg as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(__darwin_zmm_reg),
            "::",
            stringify!(__zmm_reg)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __darwin_opmask_reg {
    pub __opmask_reg: [::std::os::raw::c_char; 8usize],
}
#[test]
fn bindgen_test_layout___darwin_opmask_reg() {
    assert_eq!(
        ::std::mem::size_of::<__darwin_opmask_reg>(),
        8usize,
        concat!("Size of: ", stringify!(__darwin_opmask_reg))
    );
    assert_eq!(
        ::std::mem::align_of::<__darwin_opmask_reg>(),
        1usize,
        concat!("Alignment of ", stringify!(__darwin_opmask_reg))
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<__darwin_opmask_reg>())).__opmask_reg as *const _ as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(__darwin_opmask_reg),
            "::",
            stringify!(__opmask_reg)
        )
    );
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct __darwin_i386_float_state {
    pub __fpu_reserved: [::std::os::raw::c_int; 2usize],
    pub __fpu_fcw: __darwin_fp_control,
    pub __fpu_fsw: __darwin_fp_status,
    pub __fpu_ftw: __uint8_t,
    pub __fpu_rsrv1: __uint8_t,
    pub __fpu_fop: __uint16_t,
    pub __fpu_ip: __uint32_t,
    pub __fpu_cs: __uint16_t,
    pub __fpu_rsrv2: __uint16_t,
    pub __fpu_dp: __uint32_t,
    pub __fpu_ds: __uint16_t,
    pub __fpu_rsrv3: __uint16_t,
    pub __fpu_mxcsr: __uint32_t,
    pub __fpu_mxcsrmask: __uint32_t,
    pub __fpu_stmm0: __darwin_mmst_reg,
    pub __fpu_stmm1: __darwin_mmst_reg,
    pub __fpu_stmm2: __darwin_mmst_reg,
    pub __fpu_stmm3: __darwin_mmst_reg,
    pub __fpu_stmm4: __darwin_mmst_reg,
    pub __fpu_stmm5: __darwin_mmst_reg,
    pub __fpu_stmm6: __darwin_mmst_reg,
    pub __fpu_stmm7: __darwin_mmst_reg,
    pub __fpu_xmm0: __darwin_xmm_reg,
    pub __fpu_xmm1: __darwin_xmm_reg,
    pub __fpu_xmm2: __darwin_xmm_reg,
    pub __fpu_xmm3: __darwin_xmm_reg,
    pub __fpu_xmm4: __darwin_xmm_reg,
    pub __fpu_xmm5: __darwin_xmm_reg,
    pub __fpu_xmm6: __darwin_xmm_reg,
    pub __fpu_xmm7: __darwin_xmm_reg,
    pub __fpu_rsrv4: [::std::os::raw::c_char; 224usize],
    pub __fpu_reserved1: ::std::os::raw::c_int,
}
#[test]
fn bindgen_test_layout___darwin_i386_float_state() {
    assert_eq!(
        ::std::mem::size_of::<__darwin_i386_float_state>(),
        524usize,
        concat!("Size of: ", stringify!(__darwin_i386_float_state))
    );
    assert_eq!(
        ::std::mem::align_of::<__darwin_i386_float_state>(),
        4usize,
        concat!("Alignment of ", stringify!(__darwin_i386_float_state))
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<__darwin_i386_float_state>())).__fpu_reserved as *const _
                as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(__darwin_i386_float_state),
            "::",
            stringify!(__fpu_reserved)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<__darwin_i386_float_state>())).__fpu_fcw as *const _ as usize
        },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(__darwin_i386_float_state),
            "::",
            stringify!(__fpu_fcw)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<__darwin_i386_float_state>())).__fpu_fsw as *const _ as usize
        },
        10usize,
        concat!(
            "Offset of field: ",
            stringify!(__darwin_i386_float_state),
            "::",
            stringify!(__fpu_fsw)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<__darwin_i386_float_state>())).__fpu_ftw as *const _ as usize
        },
        12usize,
        concat!(
            "Offset of field: ",
            stringify!(__darwin_i386_float_state),
            "::",
            stringify!(__fpu_ftw)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<__darwin_i386_float_state>())).__fpu_rsrv1 as *const _ as usize
        },
        13usize,
        concat!(
            "Offset of field: ",
            stringify!(__darwin_i386_float_state),
            "::",
            stringify!(__fpu_rsrv1)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<__darwin_i386_float_state>())).__fpu_fop as *const _ as usize
        },
        14usize,
        concat!(
            "Offset of field: ",
            stringify!(__darwin_i386_float_state),
            "::",
            stringify!(__fpu_fop)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<__darwin_i386_float_state>())).__fpu_ip as *const _ as usize
        },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(__darwin_i386_float_state),
            "::",
            stringify!(__fpu_ip)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<__darwin_i386_float_state>())).__fpu_cs as *const _ as usize
        },
        20usize,
        concat!(
            "Offset of field: ",
            stringify!(__darwin_i386_float_state),
            "::",
            stringify!(__fpu_cs)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<__darwin_i386_float_state>())).__fpu_rsrv2 as *const _ as usize
        },
        22usize,
        concat!(
            "Offset of field: ",
            stringify!(__darwin_i386_float_state),
            "::",
            stringify!(__fpu_rsrv2)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<__darwin_i386_float_state>())).__fpu_dp as *const _ as usize
        },
        24usize,
        concat!(
            "Offset of field: ",
            stringify!(__darwin_i386_float_state),
            "::",
            stringify!(__fpu_dp)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<__darwin_i386_float_state>())).__fpu_ds as *const _ as usize
        },
        28usize,
        concat!(
            "Offset of field: ",
            stringify!(__darwin_i386_float_state),
            "::",
            stringify!(__fpu_ds)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<__darwin_i386_float_state>())).__fpu_rsrv3 as *const _ as usize
        },
        30usize,
        concat!(
            "Offset of field: ",
            stringify!(__darwin_i386_float_state),
            "::",
            stringify!(__fpu_rsrv3)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<__darwin_i386_float_state>())).__fpu_mxcsr as *const _ as usize
        },
        32usize,
        concat!(
            "Offset of field: ",
            stringify!(__darwin_i386_float_state),
            "::",
            stringify!(__fpu_mxcsr)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<__darwin_i386_float_state>())).__fpu_mxcsrmask as *const _
                as usize
        },
        36usize,
        concat!(
            "Offset of field: ",
            stringify!(__darwin_i386_float_state),
            "::",
            stringify!(__fpu_mxcsrmask)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<__darwin_i386_float_state>())).__fpu_stmm0 as *const _ as usize
        },
        40usize,
        concat!(
            "Offset of field: ",
            stringify!(__darwin_i386_float_state),
            "::",
            stringify!(__fpu_stmm0)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<__darwin_i386_float_state>())).__fpu_stmm1 as *const _ as usize
        },
        56usize,
        concat!(
            "Offset of field: ",
            stringify!(__darwin_i386_float_state),
            "::",
            stringify!(__fpu_stmm1)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<__darwin_i386_float_state>())).__fpu_stmm2 as *const _ as usize
        },
        72usize,
        concat!(
            "Offset of field: ",
            stringify!(__darwin_i386_float_state),
            "::",
            stringify!(__fpu_stmm2)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<__darwin_i386_float_state>())).__fpu_stmm3 as *const _ as usize
        },
        88usize,
        concat!(
            "Offset of field: ",
            stringify!(__darwin_i386_float_state),
            "::",
            stringify!(__fpu_stmm3)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<__darwin_i386_float_state>())).__fpu_stmm4 as *const _ as usize
        },
        104usize,
        concat!(
            "Offset of field: ",
            stringify!(__darwin_i386_float_state),
            "::",
            stringify!(__fpu_stmm4)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<__darwin_i386_float_state>())).__fpu_stmm5 as *const _ as usize
        },
        120usize,
        concat!(
            "Offset of field: ",
            stringify!(__darwin_i386_float_state),
            "::",
            stringify!(__fpu_stmm5)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<__darwin_i386_float_state>())).__fpu_stmm6 as *const _ as usize
        },
        136usize,
        concat!(
            "Offset of field: ",
            stringify!(__darwin_i386_float_state),
            "::",
            stringify!(__fpu_stmm6)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<__darwin_i386_float_state>())).__fpu_stmm7 as *const _ as usize
        },
        152usize,
        concat!(
            "Offset of field: ",
            stringify!(__darwin_i386_float_state),
            "::",
            stringify!(__fpu_stmm7)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<__darwin_i386_float_state>())).__fpu_xmm0 as *const _ as usize
        },
        168usize,
        concat!(
            "Offset of field: ",
            stringify!(__darwin_i386_float_state),
            "::",
            stringify!(__fpu_xmm0)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<__darwin_i386_float_state>())).__fpu_xmm1 as *const _ as usize
        },
        184usize,
        concat!(
            "Offset of field: ",
            stringify!(__darwin_i386_float_state),
            "::",
            stringify!(__fpu_xmm1)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<__darwin_i386_float_state>())).__fpu_xmm2 as *const _ as usize
        },
        200usize,
        concat!(
            "Offset of field: ",
            stringify!(__darwin_i386_float_state),
            "::",
            stringify!(__fpu_xmm2)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<__darwin_i386_float_state>())).__fpu_xmm3 as *const _ as usize
        },
        216usize,
        concat!(
            "Offset of field: ",
            stringify!(__darwin_i386_float_state),
            "::",
            stringify!(__fpu_xmm3)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<__darwin_i386_float_state>())).__fpu_xmm4 as *const _ as usize
        },
        232usize,
        concat!(
            "Offset of field: ",
            stringify!(__darwin_i386_float_state),
            "::",
            stringify!(__fpu_xmm4)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<__darwin_i386_float_state>())).__fpu_xmm5 as *const _ as usize
        },
        248usize,
        concat!(
            "Offset of field: ",
            stringify!(__darwin_i386_float_state),
            "::",
            stringify!(__fpu_xmm5)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<__darwin_i386_float_state>())).__fpu_xmm6 as *const _ as usize
        },
        264usize,
        concat!(
            "Offset of field: ",
            stringify!(__darwin_i386_float_state),
            "::",
            stringify!(__fpu_xmm6)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<__darwin_i386_float_state>())).__fpu_xmm7 as *const _ as usize
        },
        280usize,
        concat!(
            "Offset of field: ",
            stringify!(__darwin_i386_float_state),
            "::",
            stringify!(__fpu_xmm7)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<__darwin_i386_float_state>())).__fpu_rsrv4 as *const _ as usize
        },
        296usize,
        concat!(
            "Offset of field: ",
            stringify!(__darwin_i386_float_state),
            "::",
            stringify!(__fpu_rsrv4)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<__darwin_i386_float_state>())).__fpu_reserved1 as *const _
                as usize
        },
        520usize,
        concat!(
            "Offset of field: ",
            stringify!(__darwin_i386_float_state),
            "::",
            stringify!(__fpu_reserved1)
        )
    );
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct __darwin_i386_avx_state {
    pub __fpu_reserved: [::std::os::raw::c_int; 2usize],
    pub __fpu_fcw: __darwin_fp_control,
    pub __fpu_fsw: __darwin_fp_status,
    pub __fpu_ftw: __uint8_t,
    pub __fpu_rsrv1: __uint8_t,
    pub __fpu_fop: __uint16_t,
    pub __fpu_ip: __uint32_t,
    pub __fpu_cs: __uint16_t,
    pub __fpu_rsrv2: __uint16_t,
    pub __fpu_dp: __uint32_t,
    pub __fpu_ds: __uint16_t,
    pub __fpu_rsrv3: __uint16_t,
    pub __fpu_mxcsr: __uint32_t,
    pub __fpu_mxcsrmask: __uint32_t,
    pub __fpu_stmm0: __darwin_mmst_reg,
    pub __fpu_stmm1: __darwin_mmst_reg,
    pub __fpu_stmm2: __darwin_mmst_reg,
    pub __fpu_stmm3: __darwin_mmst_reg,
    pub __fpu_stmm4: __darwin_mmst_reg,
    pub __fpu_stmm5: __darwin_mmst_reg,
    pub __fpu_stmm6: __darwin_mmst_reg,
    pub __fpu_stmm7: __darwin_mmst_reg,
    pub __fpu_xmm0: __darwin_xmm_reg,
    pub __fpu_xmm1: __darwin_xmm_reg,
    pub __fpu_xmm2: __darwin_xmm_reg,
    pub __fpu_xmm3: __darwin_xmm_reg,
    pub __fpu_xmm4: __darwin_xmm_reg,
    pub __fpu_xmm5: __darwin_xmm_reg,
    pub __fpu_xmm6: __darwin_xmm_reg,
    pub __fpu_xmm7: __darwin_xmm_reg,
    pub __fpu_rsrv4: [::std::os::raw::c_char; 224usize],
    pub __fpu_reserved1: ::std::os::raw::c_int,
    pub __avx_reserved1: [::std::os::raw::c_char; 64usize],
    pub __fpu_ymmh0: __darwin_xmm_reg,
    pub __fpu_ymmh1: __darwin_xmm_reg,
    pub __fpu_ymmh2: __darwin_xmm_reg,
    pub __fpu_ymmh3: __darwin_xmm_reg,
    pub __fpu_ymmh4: __darwin_xmm_reg,
    pub __fpu_ymmh5: __darwin_xmm_reg,
    pub __fpu_ymmh6: __darwin_xmm_reg,
    pub __fpu_ymmh7: __darwin_xmm_reg,
}
#[test]
fn bindgen_test_layout___darwin_i386_avx_state() {
    assert_eq!(
        ::std::mem::size_of::<__darwin_i386_avx_state>(),
        716usize,
        concat!("Size of: ", stringify!(__darwin_i386_avx_state))
    );
    assert_eq!(
        ::std::mem::align_of::<__darwin_i386_avx_state>(),
        4usize,
        concat!("Alignment of ", stringify!(__darwin_i386_avx_state))
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<__darwin_i386_avx_state>())).__fpu_reserved as *const _ as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(__darwin_i386_avx_state),
            "::",
            stringify!(__fpu_reserved)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<__darwin_i386_avx_state>())).__fpu_fcw as *const _ as usize
        },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(__darwin_i386_avx_state),
            "::",
            stringify!(__fpu_fcw)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<__darwin_i386_avx_state>())).__fpu_fsw as *const _ as usize
        },
        10usize,
        concat!(
            "Offset of field: ",
            stringify!(__darwin_i386_avx_state),
            "::",
            stringify!(__fpu_fsw)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<__darwin_i386_avx_state>())).__fpu_ftw as *const _ as usize
        },
        12usize,
        concat!(
            "Offset of field: ",
            stringify!(__darwin_i386_avx_state),
            "::",
            stringify!(__fpu_ftw)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<__darwin_i386_avx_state>())).__fpu_rsrv1 as *const _ as usize
        },
        13usize,
        concat!(
            "Offset of field: ",
            stringify!(__darwin_i386_avx_state),
            "::",
            stringify!(__fpu_rsrv1)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<__darwin_i386_avx_state>())).__fpu_fop as *const _ as usize
        },
        14usize,
        concat!(
            "Offset of field: ",
            stringify!(__darwin_i386_avx_state),
            "::",
            stringify!(__fpu_fop)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<__darwin_i386_avx_state>())).__fpu_ip as *const _ as usize
        },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(__darwin_i386_avx_state),
            "::",
            stringify!(__fpu_ip)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<__darwin_i386_avx_state>())).__fpu_cs as *const _ as usize
        },
        20usize,
        concat!(
            "Offset of field: ",
            stringify!(__darwin_i386_avx_state),
            "::",
            stringify!(__fpu_cs)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<__darwin_i386_avx_state>())).__fpu_rsrv2 as *const _ as usize
        },
        22usize,
        concat!(
            "Offset of field: ",
            stringify!(__darwin_i386_avx_state),
            "::",
            stringify!(__fpu_rsrv2)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<__darwin_i386_avx_state>())).__fpu_dp as *const _ as usize
        },
        24usize,
        concat!(
            "Offset of field: ",
            stringify!(__darwin_i386_avx_state),
            "::",
            stringify!(__fpu_dp)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<__darwin_i386_avx_state>())).__fpu_ds as *const _ as usize
        },
        28usize,
        concat!(
            "Offset of field: ",
            stringify!(__darwin_i386_avx_state),
            "::",
            stringify!(__fpu_ds)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<__darwin_i386_avx_state>())).__fpu_rsrv3 as *const _ as usize
        },
        30usize,
        concat!(
            "Offset of field: ",
            stringify!(__darwin_i386_avx_state),
            "::",
            stringify!(__fpu_rsrv3)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<__darwin_i386_avx_state>())).__fpu_mxcsr as *const _ as usize
        },
        32usize,
        concat!(
            "Offset of field: ",
            stringify!(__darwin_i386_avx_state),
            "::",
            stringify!(__fpu_mxcsr)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<__darwin_i386_avx_state>())).__fpu_mxcsrmask as *const _ as usize
        },
        36usize,
        concat!(
            "Offset of field: ",
            stringify!(__darwin_i386_avx_state),
            "::",
            stringify!(__fpu_mxcsrmask)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<__darwin_i386_avx_state>())).__fpu_stmm0 as *const _ as usize
        },
        40usize,
        concat!(
            "Offset of field: ",
            stringify!(__darwin_i386_avx_state),
            "::",
            stringify!(__fpu_stmm0)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<__darwin_i386_avx_state>())).__fpu_stmm1 as *const _ as usize
        },
        56usize,
        concat!(
            "Offset of field: ",
            stringify!(__darwin_i386_avx_state),
            "::",
            stringify!(__fpu_stmm1)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<__darwin_i386_avx_state>())).__fpu_stmm2 as *const _ as usize
        },
        72usize,
        concat!(
            "Offset of field: ",
            stringify!(__darwin_i386_avx_state),
            "::",
            stringify!(__fpu_stmm2)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<__darwin_i386_avx_state>())).__fpu_stmm3 as *const _ as usize
        },
        88usize,
        concat!(
            "Offset of field: ",
            stringify!(__darwin_i386_avx_state),
            "::",
            stringify!(__fpu_stmm3)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<__darwin_i386_avx_state>())).__fpu_stmm4 as *const _ as usize
        },
        104usize,
        concat!(
            "Offset of field: ",
            stringify!(__darwin_i386_avx_state),
            "::",
            stringify!(__fpu_stmm4)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<__darwin_i386_avx_state>())).__fpu_stmm5 as *const _ as usize
        },
        120usize,
        concat!(
            "Offset of field: ",
            stringify!(__darwin_i386_avx_state),
            "::",
            stringify!(__fpu_stmm5)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<__darwin_i386_avx_state>())).__fpu_stmm6 as *const _ as usize
        },
        136usize,
        concat!(
            "Offset of field: ",
            stringify!(__darwin_i386_avx_state),
            "::",
            stringify!(__fpu_stmm6)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<__darwin_i386_avx_state>())).__fpu_stmm7 as *const _ as usize
        },
        152usize,
        concat!(
            "Offset of field: ",
            stringify!(__darwin_i386_avx_state),
            "::",
            stringify!(__fpu_stmm7)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<__darwin_i386_avx_state>())).__fpu_xmm0 as *const _ as usize
        },
        168usize,
        concat!(
            "Offset of field: ",
            stringify!(__darwin_i386_avx_state),
            "::",
            stringify!(__fpu_xmm0)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<__darwin_i386_avx_state>())).__fpu_xmm1 as *const _ as usize
        },
        184usize,
        concat!(
            "Offset of field: ",
            stringify!(__darwin_i386_avx_state),
            "::",
            stringify!(__fpu_xmm1)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<__darwin_i386_avx_state>())).__fpu_xmm2 as *const _ as usize
        },
        200usize,
        concat!(
            "Offset of field: ",
            stringify!(__darwin_i386_avx_state),
            "::",
            stringify!(__fpu_xmm2)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<__darwin_i386_avx_state>())).__fpu_xmm3 as *const _ as usize
        },
        216usize,
        concat!(
            "Offset of field: ",
            stringify!(__darwin_i386_avx_state),
            "::",
            stringify!(__fpu_xmm3)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<__darwin_i386_avx_state>())).__fpu_xmm4 as *const _ as usize
        },
        232usize,
        concat!(
            "Offset of field: ",
            stringify!(__darwin_i386_avx_state),
            "::",
            stringify!(__fpu_xmm4)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<__darwin_i386_avx_state>())).__fpu_xmm5 as *const _ as usize
        },
        248usize,
        concat!(
            "Offset of field: ",
            stringify!(__darwin_i386_avx_state),
            "::",
            stringify!(__fpu_xmm5)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<__darwin_i386_avx_state>())).__fpu_xmm6 as *const _ as usize
        },
        264usize,
        concat!(
            "Offset of field: ",
            stringify!(__darwin_i386_avx_state),
            "::",
            stringify!(__fpu_xmm6)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<__darwin_i386_avx_state>())).__fpu_xmm7 as *const _ as usize
        },
        280usize,
        concat!(
            "Offset of field: ",
            stringify!(__darwin_i386_avx_state),
            "::",
            stringify!(__fpu_xmm7)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<__darwin_i386_avx_state>())).__fpu_rsrv4 as *const _ as usize
        },
        296usize,
        concat!(
            "Offset of field: ",
            stringify!(__darwin_i386_avx_state),
            "::",
            stringify!(__fpu_rsrv4)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<__darwin_i386_avx_state>())).__fpu_reserved1 as *const _ as usize
        },
        520usize,
        concat!(
            "Offset of field: ",
            stringify!(__darwin_i386_avx_state),
            "::",
            stringify!(__fpu_reserved1)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<__darwin_i386_avx_state>())).__avx_reserved1 as *const _ as usize
        },
        524usize,
        concat!(
            "Offset of field: ",
            stringify!(__darwin_i386_avx_state),
            "::",
            stringify!(__avx_reserved1)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<__darwin_i386_avx_state>())).__fpu_ymmh0 as *const _ as usize
        },
        588usize,
        concat!(
            "Offset of field: ",
            stringify!(__darwin_i386_avx_state),
            "::",
            stringify!(__fpu_ymmh0)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<__darwin_i386_avx_state>())).__fpu_ymmh1 as *const _ as usize
        },
        604usize,
        concat!(
            "Offset of field: ",
            stringify!(__darwin_i386_avx_state),
            "::",
            stringify!(__fpu_ymmh1)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<__darwin_i386_avx_state>())).__fpu_ymmh2 as *const _ as usize
        },
        620usize,
        concat!(
            "Offset of field: ",
            stringify!(__darwin_i386_avx_state),
            "::",
            stringify!(__fpu_ymmh2)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<__darwin_i386_avx_state>())).__fpu_ymmh3 as *const _ as usize
        },
        636usize,
        concat!(
            "Offset of field: ",
            stringify!(__darwin_i386_avx_state),
            "::",
            stringify!(__fpu_ymmh3)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<__darwin_i386_avx_state>())).__fpu_ymmh4 as *const _ as usize
        },
        652usize,
        concat!(
            "Offset of field: ",
            stringify!(__darwin_i386_avx_state),
            "::",
            stringify!(__fpu_ymmh4)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<__darwin_i386_avx_state>())).__fpu_ymmh5 as *const _ as usize
        },
        668usize,
        concat!(
            "Offset of field: ",
            stringify!(__darwin_i386_avx_state),
            "::",
            stringify!(__fpu_ymmh5)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<__darwin_i386_avx_state>())).__fpu_ymmh6 as *const _ as usize
        },
        684usize,
        concat!(
            "Offset of field: ",
            stringify!(__darwin_i386_avx_state),
            "::",
            stringify!(__fpu_ymmh6)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<__darwin_i386_avx_state>())).__fpu_ymmh7 as *const _ as usize
        },
        700usize,
        concat!(
            "Offset of field: ",
            stringify!(__darwin_i386_avx_state),
            "::",
            stringify!(__fpu_ymmh7)
        )
    );
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct __darwin_i386_avx512_state {
    pub __fpu_reserved: [::std::os::raw::c_int; 2usize],
    pub __fpu_fcw: __darwin_fp_control,
    pub __fpu_fsw: __darwin_fp_status,
    pub __fpu_ftw: __uint8_t,
    pub __fpu_rsrv1: __uint8_t,
    pub __fpu_fop: __uint16_t,
    pub __fpu_ip: __uint32_t,
    pub __fpu_cs: __uint16_t,
    pub __fpu_rsrv2: __uint16_t,
    pub __fpu_dp: __uint32_t,
    pub __fpu_ds: __uint16_t,
    pub __fpu_rsrv3: __uint16_t,
    pub __fpu_mxcsr: __uint32_t,
    pub __fpu_mxcsrmask: __uint32_t,
    pub __fpu_stmm0: __darwin_mmst_reg,
    pub __fpu_stmm1: __darwin_mmst_reg,
    pub __fpu_stmm2: __darwin_mmst_reg,
    pub __fpu_stmm3: __darwin_mmst_reg,
    pub __fpu_stmm4: __darwin_mmst_reg,
    pub __fpu_stmm5: __darwin_mmst_reg,
    pub __fpu_stmm6: __darwin_mmst_reg,
    pub __fpu_stmm7: __darwin_mmst_reg,
    pub __fpu_xmm0: __darwin_xmm_reg,
    pub __fpu_xmm1: __darwin_xmm_reg,
    pub __fpu_xmm2: __darwin_xmm_reg,
    pub __fpu_xmm3: __darwin_xmm_reg,
    pub __fpu_xmm4: __darwin_xmm_reg,
    pub __fpu_xmm5: __darwin_xmm_reg,
    pub __fpu_xmm6: __darwin_xmm_reg,
    pub __fpu_xmm7: __darwin_xmm_reg,
    pub __fpu_rsrv4: [::std::os::raw::c_char; 224usize],
    pub __fpu_reserved1: ::std::os::raw::c_int,
    pub __avx_reserved1: [::std::os::raw::c_char; 64usize],
    pub __fpu_ymmh0: __darwin_xmm_reg,
    pub __fpu_ymmh1: __darwin_xmm_reg,
    pub __fpu_ymmh2: __darwin_xmm_reg,
    pub __fpu_ymmh3: __darwin_xmm_reg,
    pub __fpu_ymmh4: __darwin_xmm_reg,
    pub __fpu_ymmh5: __darwin_xmm_reg,
    pub __fpu_ymmh6: __darwin_xmm_reg,
    pub __fpu_ymmh7: __darwin_xmm_reg,
    pub __fpu_k0: __darwin_opmask_reg,
    pub __fpu_k1: __darwin_opmask_reg,
    pub __fpu_k2: __darwin_opmask_reg,
    pub __fpu_k3: __darwin_opmask_reg,
    pub __fpu_k4: __darwin_opmask_reg,
    pub __fpu_k5: __darwin_opmask_reg,
    pub __fpu_k6: __darwin_opmask_reg,
    pub __fpu_k7: __darwin_opmask_reg,
    pub __fpu_zmmh0: __darwin_ymm_reg,
    pub __fpu_zmmh1: __darwin_ymm_reg,
    pub __fpu_zmmh2: __darwin_ymm_reg,
    pub __fpu_zmmh3: __darwin_ymm_reg,
    pub __fpu_zmmh4: __darwin_ymm_reg,
    pub __fpu_zmmh5: __darwin_ymm_reg,
    pub __fpu_zmmh6: __darwin_ymm_reg,
    pub __fpu_zmmh7: __darwin_ymm_reg,
}
#[test]
fn bindgen_test_layout___darwin_i386_avx512_state() {
    assert_eq!(
        ::std::mem::size_of::<__darwin_i386_avx512_state>(),
        1036usize,
        concat!("Size of: ", stringify!(__darwin_i386_avx512_state))
    );
    assert_eq!(
        ::std::mem::align_of::<__darwin_i386_avx512_state>(),
        4usize,
        concat!("Alignment of ", stringify!(__darwin_i386_avx512_state))
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<__darwin_i386_avx512_state>())).__fpu_reserved as *const _
                as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(__darwin_i386_avx512_state),
            "::",
            stringify!(__fpu_reserved)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<__darwin_i386_avx512_state>())).__fpu_fcw as *const _ as usize
        },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(__darwin_i386_avx512_state),
            "::",
            stringify!(__fpu_fcw)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<__darwin_i386_avx512_state>())).__fpu_fsw as *const _ as usize
        },
        10usize,
        concat!(
            "Offset of field: ",
            stringify!(__darwin_i386_avx512_state),
            "::",
            stringify!(__fpu_fsw)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<__darwin_i386_avx512_state>())).__fpu_ftw as *const _ as usize
        },
        12usize,
        concat!(
            "Offset of field: ",
            stringify!(__darwin_i386_avx512_state),
            "::",
            stringify!(__fpu_ftw)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<__darwin_i386_avx512_state>())).__fpu_rsrv1 as *const _ as usize
        },
        13usize,
        concat!(
            "Offset of field: ",
            stringify!(__darwin_i386_avx512_state),
            "::",
            stringify!(__fpu_rsrv1)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<__darwin_i386_avx512_state>())).__fpu_fop as *const _ as usize
        },
        14usize,
        concat!(
            "Offset of field: ",
            stringify!(__darwin_i386_avx512_state),
            "::",
            stringify!(__fpu_fop)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<__darwin_i386_avx512_state>())).__fpu_ip as *const _ as usize
        },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(__darwin_i386_avx512_state),
            "::",
            stringify!(__fpu_ip)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<__darwin_i386_avx512_state>())).__fpu_cs as *const _ as usize
        },
        20usize,
        concat!(
            "Offset of field: ",
            stringify!(__darwin_i386_avx512_state),
            "::",
            stringify!(__fpu_cs)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<__darwin_i386_avx512_state>())).__fpu_rsrv2 as *const _ as usize
        },
        22usize,
        concat!(
            "Offset of field: ",
            stringify!(__darwin_i386_avx512_state),
            "::",
            stringify!(__fpu_rsrv2)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<__darwin_i386_avx512_state>())).__fpu_dp as *const _ as usize
        },
        24usize,
        concat!(
            "Offset of field: ",
            stringify!(__darwin_i386_avx512_state),
            "::",
            stringify!(__fpu_dp)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<__darwin_i386_avx512_state>())).__fpu_ds as *const _ as usize
        },
        28usize,
        concat!(
            "Offset of field: ",
            stringify!(__darwin_i386_avx512_state),
            "::",
            stringify!(__fpu_ds)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<__darwin_i386_avx512_state>())).__fpu_rsrv3 as *const _ as usize
        },
        30usize,
        concat!(
            "Offset of field: ",
            stringify!(__darwin_i386_avx512_state),
            "::",
            stringify!(__fpu_rsrv3)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<__darwin_i386_avx512_state>())).__fpu_mxcsr as *const _ as usize
        },
        32usize,
        concat!(
            "Offset of field: ",
            stringify!(__darwin_i386_avx512_state),
            "::",
            stringify!(__fpu_mxcsr)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<__darwin_i386_avx512_state>())).__fpu_mxcsrmask as *const _
                as usize
        },
        36usize,
        concat!(
            "Offset of field: ",
            stringify!(__darwin_i386_avx512_state),
            "::",
            stringify!(__fpu_mxcsrmask)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<__darwin_i386_avx512_state>())).__fpu_stmm0 as *const _ as usize
        },
        40usize,
        concat!(
            "Offset of field: ",
            stringify!(__darwin_i386_avx512_state),
            "::",
            stringify!(__fpu_stmm0)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<__darwin_i386_avx512_state>())).__fpu_stmm1 as *const _ as usize
        },
        56usize,
        concat!(
            "Offset of field: ",
            stringify!(__darwin_i386_avx512_state),
            "::",
            stringify!(__fpu_stmm1)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<__darwin_i386_avx512_state>())).__fpu_stmm2 as *const _ as usize
        },
        72usize,
        concat!(
            "Offset of field: ",
            stringify!(__darwin_i386_avx512_state),
            "::",
            stringify!(__fpu_stmm2)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<__darwin_i386_avx512_state>())).__fpu_stmm3 as *const _ as usize
        },
        88usize,
        concat!(
            "Offset of field: ",
            stringify!(__darwin_i386_avx512_state),
            "::",
            stringify!(__fpu_stmm3)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<__darwin_i386_avx512_state>())).__fpu_stmm4 as *const _ as usize
        },
        104usize,
        concat!(
            "Offset of field: ",
            stringify!(__darwin_i386_avx512_state),
            "::",
            stringify!(__fpu_stmm4)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<__darwin_i386_avx512_state>())).__fpu_stmm5 as *const _ as usize
        },
        120usize,
        concat!(
            "Offset of field: ",
            stringify!(__darwin_i386_avx512_state),
            "::",
            stringify!(__fpu_stmm5)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<__darwin_i386_avx512_state>())).__fpu_stmm6 as *const _ as usize
        },
        136usize,
        concat!(
            "Offset of field: ",
            stringify!(__darwin_i386_avx512_state),
            "::",
            stringify!(__fpu_stmm6)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<__darwin_i386_avx512_state>())).__fpu_stmm7 as *const _ as usize
        },
        152usize,
        concat!(
            "Offset of field: ",
            stringify!(__darwin_i386_avx512_state),
            "::",
            stringify!(__fpu_stmm7)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<__darwin_i386_avx512_state>())).__fpu_xmm0 as *const _ as usize
        },
        168usize,
        concat!(
            "Offset of field: ",
            stringify!(__darwin_i386_avx512_state),
            "::",
            stringify!(__fpu_xmm0)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<__darwin_i386_avx512_state>())).__fpu_xmm1 as *const _ as usize
        },
        184usize,
        concat!(
            "Offset of field: ",
            stringify!(__darwin_i386_avx512_state),
            "::",
            stringify!(__fpu_xmm1)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<__darwin_i386_avx512_state>())).__fpu_xmm2 as *const _ as usize
        },
        200usize,
        concat!(
            "Offset of field: ",
            stringify!(__darwin_i386_avx512_state),
            "::",
            stringify!(__fpu_xmm2)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<__darwin_i386_avx512_state>())).__fpu_xmm3 as *const _ as usize
        },
        216usize,
        concat!(
            "Offset of field: ",
            stringify!(__darwin_i386_avx512_state),
            "::",
            stringify!(__fpu_xmm3)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<__darwin_i386_avx512_state>())).__fpu_xmm4 as *const _ as usize
        },
        232usize,
        concat!(
            "Offset of field: ",
            stringify!(__darwin_i386_avx512_state),
            "::",
            stringify!(__fpu_xmm4)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<__darwin_i386_avx512_state>())).__fpu_xmm5 as *const _ as usize
        },
        248usize,
        concat!(
            "Offset of field: ",
            stringify!(__darwin_i386_avx512_state),
            "::",
            stringify!(__fpu_xmm5)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<__darwin_i386_avx512_state>())).__fpu_xmm6 as *const _ as usize
        },
        264usize,
        concat!(
            "Offset of field: ",
            stringify!(__darwin_i386_avx512_state),
            "::",
            stringify!(__fpu_xmm6)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<__darwin_i386_avx512_state>())).__fpu_xmm7 as *const _ as usize
        },
        280usize,
        concat!(
            "Offset of field: ",
            stringify!(__darwin_i386_avx512_state),
            "::",
            stringify!(__fpu_xmm7)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<__darwin_i386_avx512_state>())).__fpu_rsrv4 as *const _ as usize
        },
        296usize,
        concat!(
            "Offset of field: ",
            stringify!(__darwin_i386_avx512_state),
            "::",
            stringify!(__fpu_rsrv4)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<__darwin_i386_avx512_state>())).__fpu_reserved1 as *const _
                as usize
        },
        520usize,
        concat!(
            "Offset of field: ",
            stringify!(__darwin_i386_avx512_state),
            "::",
            stringify!(__fpu_reserved1)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<__darwin_i386_avx512_state>())).__avx_reserved1 as *const _
                as usize
        },
        524usize,
        concat!(
            "Offset of field: ",
            stringify!(__darwin_i386_avx512_state),
            "::",
            stringify!(__avx_reserved1)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<__darwin_i386_avx512_state>())).__fpu_ymmh0 as *const _ as usize
        },
        588usize,
        concat!(
            "Offset of field: ",
            stringify!(__darwin_i386_avx512_state),
            "::",
            stringify!(__fpu_ymmh0)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<__darwin_i386_avx512_state>())).__fpu_ymmh1 as *const _ as usize
        },
        604usize,
        concat!(
            "Offset of field: ",
            stringify!(__darwin_i386_avx512_state),
            "::",
            stringify!(__fpu_ymmh1)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<__darwin_i386_avx512_state>())).__fpu_ymmh2 as *const _ as usize
        },
        620usize,
        concat!(
            "Offset of field: ",
            stringify!(__darwin_i386_avx512_state),
            "::",
            stringify!(__fpu_ymmh2)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<__darwin_i386_avx512_state>())).__fpu_ymmh3 as *const _ as usize
        },
        636usize,
        concat!(
            "Offset of field: ",
            stringify!(__darwin_i386_avx512_state),
            "::",
            stringify!(__fpu_ymmh3)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<__darwin_i386_avx512_state>())).__fpu_ymmh4 as *const _ as usize
        },
        652usize,
        concat!(
            "Offset of field: ",
            stringify!(__darwin_i386_avx512_state),
            "::",
            stringify!(__fpu_ymmh4)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<__darwin_i386_avx512_state>())).__fpu_ymmh5 as *const _ as usize
        },
        668usize,
        concat!(
            "Offset of field: ",
            stringify!(__darwin_i386_avx512_state),
            "::",
            stringify!(__fpu_ymmh5)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<__darwin_i386_avx512_state>())).__fpu_ymmh6 as *const _ as usize
        },
        684usize,
        concat!(
            "Offset of field: ",
            stringify!(__darwin_i386_avx512_state),
            "::",
            stringify!(__fpu_ymmh6)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<__darwin_i386_avx512_state>())).__fpu_ymmh7 as *const _ as usize
        },
        700usize,
        concat!(
            "Offset of field: ",
            stringify!(__darwin_i386_avx512_state),
            "::",
            stringify!(__fpu_ymmh7)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<__darwin_i386_avx512_state>())).__fpu_k0 as *const _ as usize
        },
        716usize,
        concat!(
            "Offset of field: ",
            stringify!(__darwin_i386_avx512_state),
            "::",
            stringify!(__fpu_k0)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<__darwin_i386_avx512_state>())).__fpu_k1 as *const _ as usize
        },
        724usize,
        concat!(
            "Offset of field: ",
            stringify!(__darwin_i386_avx512_state),
            "::",
            stringify!(__fpu_k1)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<__darwin_i386_avx512_state>())).__fpu_k2 as *const _ as usize
        },
        732usize,
        concat!(
            "Offset of field: ",
            stringify!(__darwin_i386_avx512_state),
            "::",
            stringify!(__fpu_k2)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<__darwin_i386_avx512_state>())).__fpu_k3 as *const _ as usize
        },
        740usize,
        concat!(
            "Offset of field: ",
            stringify!(__darwin_i386_avx512_state),
            "::",
            stringify!(__fpu_k3)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<__darwin_i386_avx512_state>())).__fpu_k4 as *const _ as usize
        },
        748usize,
        concat!(
            "Offset of field: ",
            stringify!(__darwin_i386_avx512_state),
            "::",
            stringify!(__fpu_k4)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<__darwin_i386_avx512_state>())).__fpu_k5 as *const _ as usize
        },
        756usize,
        concat!(
            "Offset of field: ",
            stringify!(__darwin_i386_avx512_state),
            "::",
            stringify!(__fpu_k5)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<__darwin_i386_avx512_state>())).__fpu_k6 as *const _ as usize
        },
        764usize,
        concat!(
            "Offset of field: ",
            stringify!(__darwin_i386_avx512_state),
            "::",
            stringify!(__fpu_k6)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<__darwin_i386_avx512_state>())).__fpu_k7 as *const _ as usize
        },
        772usize,
        concat!(
            "Offset of field: ",
            stringify!(__darwin_i386_avx512_state),
            "::",
            stringify!(__fpu_k7)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<__darwin_i386_avx512_state>())).__fpu_zmmh0 as *const _ as usize
        },
        780usize,
        concat!(
            "Offset of field: ",
            stringify!(__darwin_i386_avx512_state),
            "::",
            stringify!(__fpu_zmmh0)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<__darwin_i386_avx512_state>())).__fpu_zmmh1 as *const _ as usize
        },
        812usize,
        concat!(
            "Offset of field: ",
            stringify!(__darwin_i386_avx512_state),
            "::",
            stringify!(__fpu_zmmh1)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<__darwin_i386_avx512_state>())).__fpu_zmmh2 as *const _ as usize
        },
        844usize,
        concat!(
            "Offset of field: ",
            stringify!(__darwin_i386_avx512_state),
            "::",
            stringify!(__fpu_zmmh2)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<__darwin_i386_avx512_state>())).__fpu_zmmh3 as *const _ as usize
        },
        876usize,
        concat!(
            "Offset of field: ",
            stringify!(__darwin_i386_avx512_state),
            "::",
            stringify!(__fpu_zmmh3)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<__darwin_i386_avx512_state>())).__fpu_zmmh4 as *const _ as usize
        },
        908usize,
        concat!(
            "Offset of field: ",
            stringify!(__darwin_i386_avx512_state),
            "::",
            stringify!(__fpu_zmmh4)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<__darwin_i386_avx512_state>())).__fpu_zmmh5 as *const _ as usize
        },
        940usize,
        concat!(
            "Offset of field: ",
            stringify!(__darwin_i386_avx512_state),
            "::",
            stringify!(__fpu_zmmh5)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<__darwin_i386_avx512_state>())).__fpu_zmmh6 as *const _ as usize
        },
        972usize,
        concat!(
            "Offset of field: ",
            stringify!(__darwin_i386_avx512_state),
            "::",
            stringify!(__fpu_zmmh6)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<__darwin_i386_avx512_state>())).__fpu_zmmh7 as *const _ as usize
        },
        1004usize,
        concat!(
            "Offset of field: ",
            stringify!(__darwin_i386_avx512_state),
            "::",
            stringify!(__fpu_zmmh7)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __darwin_i386_exception_state {
    pub __trapno: __uint16_t,
    pub __cpu: __uint16_t,
    pub __err: __uint32_t,
    pub __faultvaddr: __uint32_t,
}
#[test]
fn bindgen_test_layout___darwin_i386_exception_state() {
    assert_eq!(
        ::std::mem::size_of::<__darwin_i386_exception_state>(),
        12usize,
        concat!("Size of: ", stringify!(__darwin_i386_exception_state))
    );
    assert_eq!(
        ::std::mem::align_of::<__darwin_i386_exception_state>(),
        4usize,
        concat!("Alignment of ", stringify!(__darwin_i386_exception_state))
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<__darwin_i386_exception_state>())).__trapno as *const _ as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(__darwin_i386_exception_state),
            "::",
            stringify!(__trapno)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<__darwin_i386_exception_state>())).__cpu as *const _ as usize
        },
        2usize,
        concat!(
            "Offset of field: ",
            stringify!(__darwin_i386_exception_state),
            "::",
            stringify!(__cpu)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<__darwin_i386_exception_state>())).__err as *const _ as usize
        },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(__darwin_i386_exception_state),
            "::",
            stringify!(__err)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<__darwin_i386_exception_state>())).__faultvaddr as *const _
                as usize
        },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(__darwin_i386_exception_state),
            "::",
            stringify!(__faultvaddr)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __darwin_x86_debug_state32 {
    pub __dr0: ::std::os::raw::c_uint,
    pub __dr1: ::std::os::raw::c_uint,
    pub __dr2: ::std::os::raw::c_uint,
    pub __dr3: ::std::os::raw::c_uint,
    pub __dr4: ::std::os::raw::c_uint,
    pub __dr5: ::std::os::raw::c_uint,
    pub __dr6: ::std::os::raw::c_uint,
    pub __dr7: ::std::os::raw::c_uint,
}
#[test]
fn bindgen_test_layout___darwin_x86_debug_state32() {
    assert_eq!(
        ::std::mem::size_of::<__darwin_x86_debug_state32>(),
        32usize,
        concat!("Size of: ", stringify!(__darwin_x86_debug_state32))
    );
    assert_eq!(
        ::std::mem::align_of::<__darwin_x86_debug_state32>(),
        4usize,
        concat!("Alignment of ", stringify!(__darwin_x86_debug_state32))
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<__darwin_x86_debug_state32>())).__dr0 as *const _ as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(__darwin_x86_debug_state32),
            "::",
            stringify!(__dr0)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<__darwin_x86_debug_state32>())).__dr1 as *const _ as usize
        },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(__darwin_x86_debug_state32),
            "::",
            stringify!(__dr1)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<__darwin_x86_debug_state32>())).__dr2 as *const _ as usize
        },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(__darwin_x86_debug_state32),
            "::",
            stringify!(__dr2)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<__darwin_x86_debug_state32>())).__dr3 as *const _ as usize
        },
        12usize,
        concat!(
            "Offset of field: ",
            stringify!(__darwin_x86_debug_state32),
            "::",
            stringify!(__dr3)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<__darwin_x86_debug_state32>())).__dr4 as *const _ as usize
        },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(__darwin_x86_debug_state32),
            "::",
            stringify!(__dr4)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<__darwin_x86_debug_state32>())).__dr5 as *const _ as usize
        },
        20usize,
        concat!(
            "Offset of field: ",
            stringify!(__darwin_x86_debug_state32),
            "::",
            stringify!(__dr5)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<__darwin_x86_debug_state32>())).__dr6 as *const _ as usize
        },
        24usize,
        concat!(
            "Offset of field: ",
            stringify!(__darwin_x86_debug_state32),
            "::",
            stringify!(__dr6)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<__darwin_x86_debug_state32>())).__dr7 as *const _ as usize
        },
        28usize,
        concat!(
            "Offset of field: ",
            stringify!(__darwin_x86_debug_state32),
            "::",
            stringify!(__dr7)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __darwin_x86_thread_state64 {
    pub __rax: __uint64_t,
    pub __rbx: __uint64_t,
    pub __rcx: __uint64_t,
    pub __rdx: __uint64_t,
    pub __rdi: __uint64_t,
    pub __rsi: __uint64_t,
    pub __rbp: __uint64_t,
    pub __rsp: __uint64_t,
    pub __r8: __uint64_t,
    pub __r9: __uint64_t,
    pub __r10: __uint64_t,
    pub __r11: __uint64_t,
    pub __r12: __uint64_t,
    pub __r13: __uint64_t,
    pub __r14: __uint64_t,
    pub __r15: __uint64_t,
    pub __rip: __uint64_t,
    pub __rflags: __uint64_t,
    pub __cs: __uint64_t,
    pub __fs: __uint64_t,
    pub __gs: __uint64_t,
}
#[test]
fn bindgen_test_layout___darwin_x86_thread_state64() {
    assert_eq!(
        ::std::mem::size_of::<__darwin_x86_thread_state64>(),
        168usize,
        concat!("Size of: ", stringify!(__darwin_x86_thread_state64))
    );
    assert_eq!(
        ::std::mem::align_of::<__darwin_x86_thread_state64>(),
        8usize,
        concat!("Alignment of ", stringify!(__darwin_x86_thread_state64))
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<__darwin_x86_thread_state64>())).__rax as *const _ as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(__darwin_x86_thread_state64),
            "::",
            stringify!(__rax)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<__darwin_x86_thread_state64>())).__rbx as *const _ as usize
        },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(__darwin_x86_thread_state64),
            "::",
            stringify!(__rbx)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<__darwin_x86_thread_state64>())).__rcx as *const _ as usize
        },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(__darwin_x86_thread_state64),
            "::",
            stringify!(__rcx)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<__darwin_x86_thread_state64>())).__rdx as *const _ as usize
        },
        24usize,
        concat!(
            "Offset of field: ",
            stringify!(__darwin_x86_thread_state64),
            "::",
            stringify!(__rdx)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<__darwin_x86_thread_state64>())).__rdi as *const _ as usize
        },
        32usize,
        concat!(
            "Offset of field: ",
            stringify!(__darwin_x86_thread_state64),
            "::",
            stringify!(__rdi)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<__darwin_x86_thread_state64>())).__rsi as *const _ as usize
        },
        40usize,
        concat!(
            "Offset of field: ",
            stringify!(__darwin_x86_thread_state64),
            "::",
            stringify!(__rsi)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<__darwin_x86_thread_state64>())).__rbp as *const _ as usize
        },
        48usize,
        concat!(
            "Offset of field: ",
            stringify!(__darwin_x86_thread_state64),
            "::",
            stringify!(__rbp)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<__darwin_x86_thread_state64>())).__rsp as *const _ as usize
        },
        56usize,
        concat!(
            "Offset of field: ",
            stringify!(__darwin_x86_thread_state64),
            "::",
            stringify!(__rsp)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<__darwin_x86_thread_state64>())).__r8 as *const _ as usize
        },
        64usize,
        concat!(
            "Offset of field: ",
            stringify!(__darwin_x86_thread_state64),
            "::",
            stringify!(__r8)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<__darwin_x86_thread_state64>())).__r9 as *const _ as usize
        },
        72usize,
        concat!(
            "Offset of field: ",
            stringify!(__darwin_x86_thread_state64),
            "::",
            stringify!(__r9)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<__darwin_x86_thread_state64>())).__r10 as *const _ as usize
        },
        80usize,
        concat!(
            "Offset of field: ",
            stringify!(__darwin_x86_thread_state64),
            "::",
            stringify!(__r10)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<__darwin_x86_thread_state64>())).__r11 as *const _ as usize
        },
        88usize,
        concat!(
            "Offset of field: ",
            stringify!(__darwin_x86_thread_state64),
            "::",
            stringify!(__r11)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<__darwin_x86_thread_state64>())).__r12 as *const _ as usize
        },
        96usize,
        concat!(
            "Offset of field: ",
            stringify!(__darwin_x86_thread_state64),
            "::",
            stringify!(__r12)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<__darwin_x86_thread_state64>())).__r13 as *const _ as usize
        },
        104usize,
        concat!(
            "Offset of field: ",
            stringify!(__darwin_x86_thread_state64),
            "::",
            stringify!(__r13)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<__darwin_x86_thread_state64>())).__r14 as *const _ as usize
        },
        112usize,
        concat!(
            "Offset of field: ",
            stringify!(__darwin_x86_thread_state64),
            "::",
            stringify!(__r14)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<__darwin_x86_thread_state64>())).__r15 as *const _ as usize
        },
        120usize,
        concat!(
            "Offset of field: ",
            stringify!(__darwin_x86_thread_state64),
            "::",
            stringify!(__r15)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<__darwin_x86_thread_state64>())).__rip as *const _ as usize
        },
        128usize,
        concat!(
            "Offset of field: ",
            stringify!(__darwin_x86_thread_state64),
            "::",
            stringify!(__rip)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<__darwin_x86_thread_state64>())).__rflags as *const _ as usize
        },
        136usize,
        concat!(
            "Offset of field: ",
            stringify!(__darwin_x86_thread_state64),
            "::",
            stringify!(__rflags)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<__darwin_x86_thread_state64>())).__cs as *const _ as usize
        },
        144usize,
        concat!(
            "Offset of field: ",
            stringify!(__darwin_x86_thread_state64),
            "::",
            stringify!(__cs)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<__darwin_x86_thread_state64>())).__fs as *const _ as usize
        },
        152usize,
        concat!(
            "Offset of field: ",
            stringify!(__darwin_x86_thread_state64),
            "::",
            stringify!(__fs)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<__darwin_x86_thread_state64>())).__gs as *const _ as usize
        },
        160usize,
        concat!(
            "Offset of field: ",
            stringify!(__darwin_x86_thread_state64),
            "::",
            stringify!(__gs)
        )
    );
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct __darwin_x86_float_state64 {
    pub __fpu_reserved: [::std::os::raw::c_int; 2usize],
    pub __fpu_fcw: __darwin_fp_control,
    pub __fpu_fsw: __darwin_fp_status,
    pub __fpu_ftw: __uint8_t,
    pub __fpu_rsrv1: __uint8_t,
    pub __fpu_fop: __uint16_t,
    pub __fpu_ip: __uint32_t,
    pub __fpu_cs: __uint16_t,
    pub __fpu_rsrv2: __uint16_t,
    pub __fpu_dp: __uint32_t,
    pub __fpu_ds: __uint16_t,
    pub __fpu_rsrv3: __uint16_t,
    pub __fpu_mxcsr: __uint32_t,
    pub __fpu_mxcsrmask: __uint32_t,
    pub __fpu_stmm0: __darwin_mmst_reg,
    pub __fpu_stmm1: __darwin_mmst_reg,
    pub __fpu_stmm2: __darwin_mmst_reg,
    pub __fpu_stmm3: __darwin_mmst_reg,
    pub __fpu_stmm4: __darwin_mmst_reg,
    pub __fpu_stmm5: __darwin_mmst_reg,
    pub __fpu_stmm6: __darwin_mmst_reg,
    pub __fpu_stmm7: __darwin_mmst_reg,
    pub __fpu_xmm0: __darwin_xmm_reg,
    pub __fpu_xmm1: __darwin_xmm_reg,
    pub __fpu_xmm2: __darwin_xmm_reg,
    pub __fpu_xmm3: __darwin_xmm_reg,
    pub __fpu_xmm4: __darwin_xmm_reg,
    pub __fpu_xmm5: __darwin_xmm_reg,
    pub __fpu_xmm6: __darwin_xmm_reg,
    pub __fpu_xmm7: __darwin_xmm_reg,
    pub __fpu_xmm8: __darwin_xmm_reg,
    pub __fpu_xmm9: __darwin_xmm_reg,
    pub __fpu_xmm10: __darwin_xmm_reg,
    pub __fpu_xmm11: __darwin_xmm_reg,
    pub __fpu_xmm12: __darwin_xmm_reg,
    pub __fpu_xmm13: __darwin_xmm_reg,
    pub __fpu_xmm14: __darwin_xmm_reg,
    pub __fpu_xmm15: __darwin_xmm_reg,
    pub __fpu_rsrv4: [::std::os::raw::c_char; 96usize],
    pub __fpu_reserved1: ::std::os::raw::c_int,
}
#[test]
fn bindgen_test_layout___darwin_x86_float_state64() {
    assert_eq!(
        ::std::mem::size_of::<__darwin_x86_float_state64>(),
        524usize,
        concat!("Size of: ", stringify!(__darwin_x86_float_state64))
    );
    assert_eq!(
        ::std::mem::align_of::<__darwin_x86_float_state64>(),
        4usize,
        concat!("Alignment of ", stringify!(__darwin_x86_float_state64))
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<__darwin_x86_float_state64>())).__fpu_reserved as *const _
                as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(__darwin_x86_float_state64),
            "::",
            stringify!(__fpu_reserved)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<__darwin_x86_float_state64>())).__fpu_fcw as *const _ as usize
        },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(__darwin_x86_float_state64),
            "::",
            stringify!(__fpu_fcw)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<__darwin_x86_float_state64>())).__fpu_fsw as *const _ as usize
        },
        10usize,
        concat!(
            "Offset of field: ",
            stringify!(__darwin_x86_float_state64),
            "::",
            stringify!(__fpu_fsw)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<__darwin_x86_float_state64>())).__fpu_ftw as *const _ as usize
        },
        12usize,
        concat!(
            "Offset of field: ",
            stringify!(__darwin_x86_float_state64),
            "::",
            stringify!(__fpu_ftw)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<__darwin_x86_float_state64>())).__fpu_rsrv1 as *const _ as usize
        },
        13usize,
        concat!(
            "Offset of field: ",
            stringify!(__darwin_x86_float_state64),
            "::",
            stringify!(__fpu_rsrv1)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<__darwin_x86_float_state64>())).__fpu_fop as *const _ as usize
        },
        14usize,
        concat!(
            "Offset of field: ",
            stringify!(__darwin_x86_float_state64),
            "::",
            stringify!(__fpu_fop)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<__darwin_x86_float_state64>())).__fpu_ip as *const _ as usize
        },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(__darwin_x86_float_state64),
            "::",
            stringify!(__fpu_ip)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<__darwin_x86_float_state64>())).__fpu_cs as *const _ as usize
        },
        20usize,
        concat!(
            "Offset of field: ",
            stringify!(__darwin_x86_float_state64),
            "::",
            stringify!(__fpu_cs)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<__darwin_x86_float_state64>())).__fpu_rsrv2 as *const _ as usize
        },
        22usize,
        concat!(
            "Offset of field: ",
            stringify!(__darwin_x86_float_state64),
            "::",
            stringify!(__fpu_rsrv2)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<__darwin_x86_float_state64>())).__fpu_dp as *const _ as usize
        },
        24usize,
        concat!(
            "Offset of field: ",
            stringify!(__darwin_x86_float_state64),
            "::",
            stringify!(__fpu_dp)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<__darwin_x86_float_state64>())).__fpu_ds as *const _ as usize
        },
        28usize,
        concat!(
            "Offset of field: ",
            stringify!(__darwin_x86_float_state64),
            "::",
            stringify!(__fpu_ds)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<__darwin_x86_float_state64>())).__fpu_rsrv3 as *const _ as usize
        },
        30usize,
        concat!(
            "Offset of field: ",
            stringify!(__darwin_x86_float_state64),
            "::",
            stringify!(__fpu_rsrv3)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<__darwin_x86_float_state64>())).__fpu_mxcsr as *const _ as usize
        },
        32usize,
        concat!(
            "Offset of field: ",
            stringify!(__darwin_x86_float_state64),
            "::",
            stringify!(__fpu_mxcsr)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<__darwin_x86_float_state64>())).__fpu_mxcsrmask as *const _
                as usize
        },
        36usize,
        concat!(
            "Offset of field: ",
            stringify!(__darwin_x86_float_state64),
            "::",
            stringify!(__fpu_mxcsrmask)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<__darwin_x86_float_state64>())).__fpu_stmm0 as *const _ as usize
        },
        40usize,
        concat!(
            "Offset of field: ",
            stringify!(__darwin_x86_float_state64),
            "::",
            stringify!(__fpu_stmm0)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<__darwin_x86_float_state64>())).__fpu_stmm1 as *const _ as usize
        },
        56usize,
        concat!(
            "Offset of field: ",
            stringify!(__darwin_x86_float_state64),
            "::",
            stringify!(__fpu_stmm1)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<__darwin_x86_float_state64>())).__fpu_stmm2 as *const _ as usize
        },
        72usize,
        concat!(
            "Offset of field: ",
            stringify!(__darwin_x86_float_state64),
            "::",
            stringify!(__fpu_stmm2)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<__darwin_x86_float_state64>())).__fpu_stmm3 as *const _ as usize
        },
        88usize,
        concat!(
            "Offset of field: ",
            stringify!(__darwin_x86_float_state64),
            "::",
            stringify!(__fpu_stmm3)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<__darwin_x86_float_state64>())).__fpu_stmm4 as *const _ as usize
        },
        104usize,
        concat!(
            "Offset of field: ",
            stringify!(__darwin_x86_float_state64),
            "::",
            stringify!(__fpu_stmm4)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<__darwin_x86_float_state64>())).__fpu_stmm5 as *const _ as usize
        },
        120usize,
        concat!(
            "Offset of field: ",
            stringify!(__darwin_x86_float_state64),
            "::",
            stringify!(__fpu_stmm5)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<__darwin_x86_float_state64>())).__fpu_stmm6 as *const _ as usize
        },
        136usize,
        concat!(
            "Offset of field: ",
            stringify!(__darwin_x86_float_state64),
            "::",
            stringify!(__fpu_stmm6)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<__darwin_x86_float_state64>())).__fpu_stmm7 as *const _ as usize
        },
        152usize,
        concat!(
            "Offset of field: ",
            stringify!(__darwin_x86_float_state64),
            "::",
            stringify!(__fpu_stmm7)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<__darwin_x86_float_state64>())).__fpu_xmm0 as *const _ as usize
        },
        168usize,
        concat!(
            "Offset of field: ",
            stringify!(__darwin_x86_float_state64),
            "::",
            stringify!(__fpu_xmm0)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<__darwin_x86_float_state64>())).__fpu_xmm1 as *const _ as usize
        },
        184usize,
        concat!(
            "Offset of field: ",
            stringify!(__darwin_x86_float_state64),
            "::",
            stringify!(__fpu_xmm1)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<__darwin_x86_float_state64>())).__fpu_xmm2 as *const _ as usize
        },
        200usize,
        concat!(
            "Offset of field: ",
            stringify!(__darwin_x86_float_state64),
            "::",
            stringify!(__fpu_xmm2)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<__darwin_x86_float_state64>())).__fpu_xmm3 as *const _ as usize
        },
        216usize,
        concat!(
            "Offset of field: ",
            stringify!(__darwin_x86_float_state64),
            "::",
            stringify!(__fpu_xmm3)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<__darwin_x86_float_state64>())).__fpu_xmm4 as *const _ as usize
        },
        232usize,
        concat!(
            "Offset of field: ",
            stringify!(__darwin_x86_float_state64),
            "::",
            stringify!(__fpu_xmm4)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<__darwin_x86_float_state64>())).__fpu_xmm5 as *const _ as usize
        },
        248usize,
        concat!(
            "Offset of field: ",
            stringify!(__darwin_x86_float_state64),
            "::",
            stringify!(__fpu_xmm5)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<__darwin_x86_float_state64>())).__fpu_xmm6 as *const _ as usize
        },
        264usize,
        concat!(
            "Offset of field: ",
            stringify!(__darwin_x86_float_state64),
            "::",
            stringify!(__fpu_xmm6)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<__darwin_x86_float_state64>())).__fpu_xmm7 as *const _ as usize
        },
        280usize,
        concat!(
            "Offset of field: ",
            stringify!(__darwin_x86_float_state64),
            "::",
            stringify!(__fpu_xmm7)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<__darwin_x86_float_state64>())).__fpu_xmm8 as *const _ as usize
        },
        296usize,
        concat!(
            "Offset of field: ",
            stringify!(__darwin_x86_float_state64),
            "::",
            stringify!(__fpu_xmm8)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<__darwin_x86_float_state64>())).__fpu_xmm9 as *const _ as usize
        },
        312usize,
        concat!(
            "Offset of field: ",
            stringify!(__darwin_x86_float_state64),
            "::",
            stringify!(__fpu_xmm9)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<__darwin_x86_float_state64>())).__fpu_xmm10 as *const _ as usize
        },
        328usize,
        concat!(
            "Offset of field: ",
            stringify!(__darwin_x86_float_state64),
            "::",
            stringify!(__fpu_xmm10)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<__darwin_x86_float_state64>())).__fpu_xmm11 as *const _ as usize
        },
        344usize,
        concat!(
            "Offset of field: ",
            stringify!(__darwin_x86_float_state64),
            "::",
            stringify!(__fpu_xmm11)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<__darwin_x86_float_state64>())).__fpu_xmm12 as *const _ as usize
        },
        360usize,
        concat!(
            "Offset of field: ",
            stringify!(__darwin_x86_float_state64),
            "::",
            stringify!(__fpu_xmm12)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<__darwin_x86_float_state64>())).__fpu_xmm13 as *const _ as usize
        },
        376usize,
        concat!(
            "Offset of field: ",
            stringify!(__darwin_x86_float_state64),
            "::",
            stringify!(__fpu_xmm13)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<__darwin_x86_float_state64>())).__fpu_xmm14 as *const _ as usize
        },
        392usize,
        concat!(
            "Offset of field: ",
            stringify!(__darwin_x86_float_state64),
            "::",
            stringify!(__fpu_xmm14)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<__darwin_x86_float_state64>())).__fpu_xmm15 as *const _ as usize
        },
        408usize,
        concat!(
            "Offset of field: ",
            stringify!(__darwin_x86_float_state64),
            "::",
            stringify!(__fpu_xmm15)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<__darwin_x86_float_state64>())).__fpu_rsrv4 as *const _ as usize
        },
        424usize,
        concat!(
            "Offset of field: ",
            stringify!(__darwin_x86_float_state64),
            "::",
            stringify!(__fpu_rsrv4)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<__darwin_x86_float_state64>())).__fpu_reserved1 as *const _
                as usize
        },
        520usize,
        concat!(
            "Offset of field: ",
            stringify!(__darwin_x86_float_state64),
            "::",
            stringify!(__fpu_reserved1)
        )
    );
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct __darwin_x86_avx_state64 {
    pub __fpu_reserved: [::std::os::raw::c_int; 2usize],
    pub __fpu_fcw: __darwin_fp_control,
    pub __fpu_fsw: __darwin_fp_status,
    pub __fpu_ftw: __uint8_t,
    pub __fpu_rsrv1: __uint8_t,
    pub __fpu_fop: __uint16_t,
    pub __fpu_ip: __uint32_t,
    pub __fpu_cs: __uint16_t,
    pub __fpu_rsrv2: __uint16_t,
    pub __fpu_dp: __uint32_t,
    pub __fpu_ds: __uint16_t,
    pub __fpu_rsrv3: __uint16_t,
    pub __fpu_mxcsr: __uint32_t,
    pub __fpu_mxcsrmask: __uint32_t,
    pub __fpu_stmm0: __darwin_mmst_reg,
    pub __fpu_stmm1: __darwin_mmst_reg,
    pub __fpu_stmm2: __darwin_mmst_reg,
    pub __fpu_stmm3: __darwin_mmst_reg,
    pub __fpu_stmm4: __darwin_mmst_reg,
    pub __fpu_stmm5: __darwin_mmst_reg,
    pub __fpu_stmm6: __darwin_mmst_reg,
    pub __fpu_stmm7: __darwin_mmst_reg,
    pub __fpu_xmm0: __darwin_xmm_reg,
    pub __fpu_xmm1: __darwin_xmm_reg,
    pub __fpu_xmm2: __darwin_xmm_reg,
    pub __fpu_xmm3: __darwin_xmm_reg,
    pub __fpu_xmm4: __darwin_xmm_reg,
    pub __fpu_xmm5: __darwin_xmm_reg,
    pub __fpu_xmm6: __darwin_xmm_reg,
    pub __fpu_xmm7: __darwin_xmm_reg,
    pub __fpu_xmm8: __darwin_xmm_reg,
    pub __fpu_xmm9: __darwin_xmm_reg,
    pub __fpu_xmm10: __darwin_xmm_reg,
    pub __fpu_xmm11: __darwin_xmm_reg,
    pub __fpu_xmm12: __darwin_xmm_reg,
    pub __fpu_xmm13: __darwin_xmm_reg,
    pub __fpu_xmm14: __darwin_xmm_reg,
    pub __fpu_xmm15: __darwin_xmm_reg,
    pub __fpu_rsrv4: [::std::os::raw::c_char; 96usize],
    pub __fpu_reserved1: ::std::os::raw::c_int,
    pub __avx_reserved1: [::std::os::raw::c_char; 64usize],
    pub __fpu_ymmh0: __darwin_xmm_reg,
    pub __fpu_ymmh1: __darwin_xmm_reg,
    pub __fpu_ymmh2: __darwin_xmm_reg,
    pub __fpu_ymmh3: __darwin_xmm_reg,
    pub __fpu_ymmh4: __darwin_xmm_reg,
    pub __fpu_ymmh5: __darwin_xmm_reg,
    pub __fpu_ymmh6: __darwin_xmm_reg,
    pub __fpu_ymmh7: __darwin_xmm_reg,
    pub __fpu_ymmh8: __darwin_xmm_reg,
    pub __fpu_ymmh9: __darwin_xmm_reg,
    pub __fpu_ymmh10: __darwin_xmm_reg,
    pub __fpu_ymmh11: __darwin_xmm_reg,
    pub __fpu_ymmh12: __darwin_xmm_reg,
    pub __fpu_ymmh13: __darwin_xmm_reg,
    pub __fpu_ymmh14: __darwin_xmm_reg,
    pub __fpu_ymmh15: __darwin_xmm_reg,
}
#[test]
fn bindgen_test_layout___darwin_x86_avx_state64() {
    assert_eq!(
        ::std::mem::size_of::<__darwin_x86_avx_state64>(),
        844usize,
        concat!("Size of: ", stringify!(__darwin_x86_avx_state64))
    );
    assert_eq!(
        ::std::mem::align_of::<__darwin_x86_avx_state64>(),
        4usize,
        concat!("Alignment of ", stringify!(__darwin_x86_avx_state64))
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<__darwin_x86_avx_state64>())).__fpu_reserved as *const _ as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(__darwin_x86_avx_state64),
            "::",
            stringify!(__fpu_reserved)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<__darwin_x86_avx_state64>())).__fpu_fcw as *const _ as usize
        },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(__darwin_x86_avx_state64),
            "::",
            stringify!(__fpu_fcw)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<__darwin_x86_avx_state64>())).__fpu_fsw as *const _ as usize
        },
        10usize,
        concat!(
            "Offset of field: ",
            stringify!(__darwin_x86_avx_state64),
            "::",
            stringify!(__fpu_fsw)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<__darwin_x86_avx_state64>())).__fpu_ftw as *const _ as usize
        },
        12usize,
        concat!(
            "Offset of field: ",
            stringify!(__darwin_x86_avx_state64),
            "::",
            stringify!(__fpu_ftw)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<__darwin_x86_avx_state64>())).__fpu_rsrv1 as *const _ as usize
        },
        13usize,
        concat!(
            "Offset of field: ",
            stringify!(__darwin_x86_avx_state64),
            "::",
            stringify!(__fpu_rsrv1)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<__darwin_x86_avx_state64>())).__fpu_fop as *const _ as usize
        },
        14usize,
        concat!(
            "Offset of field: ",
            stringify!(__darwin_x86_avx_state64),
            "::",
            stringify!(__fpu_fop)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<__darwin_x86_avx_state64>())).__fpu_ip as *const _ as usize
        },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(__darwin_x86_avx_state64),
            "::",
            stringify!(__fpu_ip)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<__darwin_x86_avx_state64>())).__fpu_cs as *const _ as usize
        },
        20usize,
        concat!(
            "Offset of field: ",
            stringify!(__darwin_x86_avx_state64),
            "::",
            stringify!(__fpu_cs)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<__darwin_x86_avx_state64>())).__fpu_rsrv2 as *const _ as usize
        },
        22usize,
        concat!(
            "Offset of field: ",
            stringify!(__darwin_x86_avx_state64),
            "::",
            stringify!(__fpu_rsrv2)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<__darwin_x86_avx_state64>())).__fpu_dp as *const _ as usize
        },
        24usize,
        concat!(
            "Offset of field: ",
            stringify!(__darwin_x86_avx_state64),
            "::",
            stringify!(__fpu_dp)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<__darwin_x86_avx_state64>())).__fpu_ds as *const _ as usize
        },
        28usize,
        concat!(
            "Offset of field: ",
            stringify!(__darwin_x86_avx_state64),
            "::",
            stringify!(__fpu_ds)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<__darwin_x86_avx_state64>())).__fpu_rsrv3 as *const _ as usize
        },
        30usize,
        concat!(
            "Offset of field: ",
            stringify!(__darwin_x86_avx_state64),
            "::",
            stringify!(__fpu_rsrv3)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<__darwin_x86_avx_state64>())).__fpu_mxcsr as *const _ as usize
        },
        32usize,
        concat!(
            "Offset of field: ",
            stringify!(__darwin_x86_avx_state64),
            "::",
            stringify!(__fpu_mxcsr)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<__darwin_x86_avx_state64>())).__fpu_mxcsrmask as *const _
                as usize
        },
        36usize,
        concat!(
            "Offset of field: ",
            stringify!(__darwin_x86_avx_state64),
            "::",
            stringify!(__fpu_mxcsrmask)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<__darwin_x86_avx_state64>())).__fpu_stmm0 as *const _ as usize
        },
        40usize,
        concat!(
            "Offset of field: ",
            stringify!(__darwin_x86_avx_state64),
            "::",
            stringify!(__fpu_stmm0)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<__darwin_x86_avx_state64>())).__fpu_stmm1 as *const _ as usize
        },
        56usize,
        concat!(
            "Offset of field: ",
            stringify!(__darwin_x86_avx_state64),
            "::",
            stringify!(__fpu_stmm1)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<__darwin_x86_avx_state64>())).__fpu_stmm2 as *const _ as usize
        },
        72usize,
        concat!(
            "Offset of field: ",
            stringify!(__darwin_x86_avx_state64),
            "::",
            stringify!(__fpu_stmm2)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<__darwin_x86_avx_state64>())).__fpu_stmm3 as *const _ as usize
        },
        88usize,
        concat!(
            "Offset of field: ",
            stringify!(__darwin_x86_avx_state64),
            "::",
            stringify!(__fpu_stmm3)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<__darwin_x86_avx_state64>())).__fpu_stmm4 as *const _ as usize
        },
        104usize,
        concat!(
            "Offset of field: ",
            stringify!(__darwin_x86_avx_state64),
            "::",
            stringify!(__fpu_stmm4)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<__darwin_x86_avx_state64>())).__fpu_stmm5 as *const _ as usize
        },
        120usize,
        concat!(
            "Offset of field: ",
            stringify!(__darwin_x86_avx_state64),
            "::",
            stringify!(__fpu_stmm5)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<__darwin_x86_avx_state64>())).__fpu_stmm6 as *const _ as usize
        },
        136usize,
        concat!(
            "Offset of field: ",
            stringify!(__darwin_x86_avx_state64),
            "::",
            stringify!(__fpu_stmm6)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<__darwin_x86_avx_state64>())).__fpu_stmm7 as *const _ as usize
        },
        152usize,
        concat!(
            "Offset of field: ",
            stringify!(__darwin_x86_avx_state64),
            "::",
            stringify!(__fpu_stmm7)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<__darwin_x86_avx_state64>())).__fpu_xmm0 as *const _ as usize
        },
        168usize,
        concat!(
            "Offset of field: ",
            stringify!(__darwin_x86_avx_state64),
            "::",
            stringify!(__fpu_xmm0)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<__darwin_x86_avx_state64>())).__fpu_xmm1 as *const _ as usize
        },
        184usize,
        concat!(
            "Offset of field: ",
            stringify!(__darwin_x86_avx_state64),
            "::",
            stringify!(__fpu_xmm1)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<__darwin_x86_avx_state64>())).__fpu_xmm2 as *const _ as usize
        },
        200usize,
        concat!(
            "Offset of field: ",
            stringify!(__darwin_x86_avx_state64),
            "::",
            stringify!(__fpu_xmm2)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<__darwin_x86_avx_state64>())).__fpu_xmm3 as *const _ as usize
        },
        216usize,
        concat!(
            "Offset of field: ",
            stringify!(__darwin_x86_avx_state64),
            "::",
            stringify!(__fpu_xmm3)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<__darwin_x86_avx_state64>())).__fpu_xmm4 as *const _ as usize
        },
        232usize,
        concat!(
            "Offset of field: ",
            stringify!(__darwin_x86_avx_state64),
            "::",
            stringify!(__fpu_xmm4)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<__darwin_x86_avx_state64>())).__fpu_xmm5 as *const _ as usize
        },
        248usize,
        concat!(
            "Offset of field: ",
            stringify!(__darwin_x86_avx_state64),
            "::",
            stringify!(__fpu_xmm5)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<__darwin_x86_avx_state64>())).__fpu_xmm6 as *const _ as usize
        },
        264usize,
        concat!(
            "Offset of field: ",
            stringify!(__darwin_x86_avx_state64),
            "::",
            stringify!(__fpu_xmm6)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<__darwin_x86_avx_state64>())).__fpu_xmm7 as *const _ as usize
        },
        280usize,
        concat!(
            "Offset of field: ",
            stringify!(__darwin_x86_avx_state64),
            "::",
            stringify!(__fpu_xmm7)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<__darwin_x86_avx_state64>())).__fpu_xmm8 as *const _ as usize
        },
        296usize,
        concat!(
            "Offset of field: ",
            stringify!(__darwin_x86_avx_state64),
            "::",
            stringify!(__fpu_xmm8)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<__darwin_x86_avx_state64>())).__fpu_xmm9 as *const _ as usize
        },
        312usize,
        concat!(
            "Offset of field: ",
            stringify!(__darwin_x86_avx_state64),
            "::",
            stringify!(__fpu_xmm9)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<__darwin_x86_avx_state64>())).__fpu_xmm10 as *const _ as usize
        },
        328usize,
        concat!(
            "Offset of field: ",
            stringify!(__darwin_x86_avx_state64),
            "::",
            stringify!(__fpu_xmm10)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<__darwin_x86_avx_state64>())).__fpu_xmm11 as *const _ as usize
        },
        344usize,
        concat!(
            "Offset of field: ",
            stringify!(__darwin_x86_avx_state64),
            "::",
            stringify!(__fpu_xmm11)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<__darwin_x86_avx_state64>())).__fpu_xmm12 as *const _ as usize
        },
        360usize,
        concat!(
            "Offset of field: ",
            stringify!(__darwin_x86_avx_state64),
            "::",
            stringify!(__fpu_xmm12)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<__darwin_x86_avx_state64>())).__fpu_xmm13 as *const _ as usize
        },
        376usize,
        concat!(
            "Offset of field: ",
            stringify!(__darwin_x86_avx_state64),
            "::",
            stringify!(__fpu_xmm13)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<__darwin_x86_avx_state64>())).__fpu_xmm14 as *const _ as usize
        },
        392usize,
        concat!(
            "Offset of field: ",
            stringify!(__darwin_x86_avx_state64),
            "::",
            stringify!(__fpu_xmm14)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<__darwin_x86_avx_state64>())).__fpu_xmm15 as *const _ as usize
        },
        408usize,
        concat!(
            "Offset of field: ",
            stringify!(__darwin_x86_avx_state64),
            "::",
            stringify!(__fpu_xmm15)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<__darwin_x86_avx_state64>())).__fpu_rsrv4 as *const _ as usize
        },
        424usize,
        concat!(
            "Offset of field: ",
            stringify!(__darwin_x86_avx_state64),
            "::",
            stringify!(__fpu_rsrv4)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<__darwin_x86_avx_state64>())).__fpu_reserved1 as *const _
                as usize
        },
        520usize,
        concat!(
            "Offset of field: ",
            stringify!(__darwin_x86_avx_state64),
            "::",
            stringify!(__fpu_reserved1)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<__darwin_x86_avx_state64>())).__avx_reserved1 as *const _
                as usize
        },
        524usize,
        concat!(
            "Offset of field: ",
            stringify!(__darwin_x86_avx_state64),
            "::",
            stringify!(__avx_reserved1)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<__darwin_x86_avx_state64>())).__fpu_ymmh0 as *const _ as usize
        },
        588usize,
        concat!(
            "Offset of field: ",
            stringify!(__darwin_x86_avx_state64),
            "::",
            stringify!(__fpu_ymmh0)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<__darwin_x86_avx_state64>())).__fpu_ymmh1 as *const _ as usize
        },
        604usize,
        concat!(
            "Offset of field: ",
            stringify!(__darwin_x86_avx_state64),
            "::",
            stringify!(__fpu_ymmh1)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<__darwin_x86_avx_state64>())).__fpu_ymmh2 as *const _ as usize
        },
        620usize,
        concat!(
            "Offset of field: ",
            stringify!(__darwin_x86_avx_state64),
            "::",
            stringify!(__fpu_ymmh2)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<__darwin_x86_avx_state64>())).__fpu_ymmh3 as *const _ as usize
        },
        636usize,
        concat!(
            "Offset of field: ",
            stringify!(__darwin_x86_avx_state64),
            "::",
            stringify!(__fpu_ymmh3)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<__darwin_x86_avx_state64>())).__fpu_ymmh4 as *const _ as usize
        },
        652usize,
        concat!(
            "Offset of field: ",
            stringify!(__darwin_x86_avx_state64),
            "::",
            stringify!(__fpu_ymmh4)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<__darwin_x86_avx_state64>())).__fpu_ymmh5 as *const _ as usize
        },
        668usize,
        concat!(
            "Offset of field: ",
            stringify!(__darwin_x86_avx_state64),
            "::",
            stringify!(__fpu_ymmh5)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<__darwin_x86_avx_state64>())).__fpu_ymmh6 as *const _ as usize
        },
        684usize,
        concat!(
            "Offset of field: ",
            stringify!(__darwin_x86_avx_state64),
            "::",
            stringify!(__fpu_ymmh6)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<__darwin_x86_avx_state64>())).__fpu_ymmh7 as *const _ as usize
        },
        700usize,
        concat!(
            "Offset of field: ",
            stringify!(__darwin_x86_avx_state64),
            "::",
            stringify!(__fpu_ymmh7)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<__darwin_x86_avx_state64>())).__fpu_ymmh8 as *const _ as usize
        },
        716usize,
        concat!(
            "Offset of field: ",
            stringify!(__darwin_x86_avx_state64),
            "::",
            stringify!(__fpu_ymmh8)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<__darwin_x86_avx_state64>())).__fpu_ymmh9 as *const _ as usize
        },
        732usize,
        concat!(
            "Offset of field: ",
            stringify!(__darwin_x86_avx_state64),
            "::",
            stringify!(__fpu_ymmh9)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<__darwin_x86_avx_state64>())).__fpu_ymmh10 as *const _ as usize
        },
        748usize,
        concat!(
            "Offset of field: ",
            stringify!(__darwin_x86_avx_state64),
            "::",
            stringify!(__fpu_ymmh10)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<__darwin_x86_avx_state64>())).__fpu_ymmh11 as *const _ as usize
        },
        764usize,
        concat!(
            "Offset of field: ",
            stringify!(__darwin_x86_avx_state64),
            "::",
            stringify!(__fpu_ymmh11)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<__darwin_x86_avx_state64>())).__fpu_ymmh12 as *const _ as usize
        },
        780usize,
        concat!(
            "Offset of field: ",
            stringify!(__darwin_x86_avx_state64),
            "::",
            stringify!(__fpu_ymmh12)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<__darwin_x86_avx_state64>())).__fpu_ymmh13 as *const _ as usize
        },
        796usize,
        concat!(
            "Offset of field: ",
            stringify!(__darwin_x86_avx_state64),
            "::",
            stringify!(__fpu_ymmh13)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<__darwin_x86_avx_state64>())).__fpu_ymmh14 as *const _ as usize
        },
        812usize,
        concat!(
            "Offset of field: ",
            stringify!(__darwin_x86_avx_state64),
            "::",
            stringify!(__fpu_ymmh14)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<__darwin_x86_avx_state64>())).__fpu_ymmh15 as *const _ as usize
        },
        828usize,
        concat!(
            "Offset of field: ",
            stringify!(__darwin_x86_avx_state64),
            "::",
            stringify!(__fpu_ymmh15)
        )
    );
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct __darwin_x86_avx512_state64 {
    pub __fpu_reserved: [::std::os::raw::c_int; 2usize],
    pub __fpu_fcw: __darwin_fp_control,
    pub __fpu_fsw: __darwin_fp_status,
    pub __fpu_ftw: __uint8_t,
    pub __fpu_rsrv1: __uint8_t,
    pub __fpu_fop: __uint16_t,
    pub __fpu_ip: __uint32_t,
    pub __fpu_cs: __uint16_t,
    pub __fpu_rsrv2: __uint16_t,
    pub __fpu_dp: __uint32_t,
    pub __fpu_ds: __uint16_t,
    pub __fpu_rsrv3: __uint16_t,
    pub __fpu_mxcsr: __uint32_t,
    pub __fpu_mxcsrmask: __uint32_t,
    pub __fpu_stmm0: __darwin_mmst_reg,
    pub __fpu_stmm1: __darwin_mmst_reg,
    pub __fpu_stmm2: __darwin_mmst_reg,
    pub __fpu_stmm3: __darwin_mmst_reg,
    pub __fpu_stmm4: __darwin_mmst_reg,
    pub __fpu_stmm5: __darwin_mmst_reg,
    pub __fpu_stmm6: __darwin_mmst_reg,
    pub __fpu_stmm7: __darwin_mmst_reg,
    pub __fpu_xmm0: __darwin_xmm_reg,
    pub __fpu_xmm1: __darwin_xmm_reg,
    pub __fpu_xmm2: __darwin_xmm_reg,
    pub __fpu_xmm3: __darwin_xmm_reg,
    pub __fpu_xmm4: __darwin_xmm_reg,
    pub __fpu_xmm5: __darwin_xmm_reg,
    pub __fpu_xmm6: __darwin_xmm_reg,
    pub __fpu_xmm7: __darwin_xmm_reg,
    pub __fpu_xmm8: __darwin_xmm_reg,
    pub __fpu_xmm9: __darwin_xmm_reg,
    pub __fpu_xmm10: __darwin_xmm_reg,
    pub __fpu_xmm11: __darwin_xmm_reg,
    pub __fpu_xmm12: __darwin_xmm_reg,
    pub __fpu_xmm13: __darwin_xmm_reg,
    pub __fpu_xmm14: __darwin_xmm_reg,
    pub __fpu_xmm15: __darwin_xmm_reg,
    pub __fpu_rsrv4: [::std::os::raw::c_char; 96usize],
    pub __fpu_reserved1: ::std::os::raw::c_int,
    pub __avx_reserved1: [::std::os::raw::c_char; 64usize],
    pub __fpu_ymmh0: __darwin_xmm_reg,
    pub __fpu_ymmh1: __darwin_xmm_reg,
    pub __fpu_ymmh2: __darwin_xmm_reg,
    pub __fpu_ymmh3: __darwin_xmm_reg,
    pub __fpu_ymmh4: __darwin_xmm_reg,
    pub __fpu_ymmh5: __darwin_xmm_reg,
    pub __fpu_ymmh6: __darwin_xmm_reg,
    pub __fpu_ymmh7: __darwin_xmm_reg,
    pub __fpu_ymmh8: __darwin_xmm_reg,
    pub __fpu_ymmh9: __darwin_xmm_reg,
    pub __fpu_ymmh10: __darwin_xmm_reg,
    pub __fpu_ymmh11: __darwin_xmm_reg,
    pub __fpu_ymmh12: __darwin_xmm_reg,
    pub __fpu_ymmh13: __darwin_xmm_reg,
    pub __fpu_ymmh14: __darwin_xmm_reg,
    pub __fpu_ymmh15: __darwin_xmm_reg,
    pub __fpu_k0: __darwin_opmask_reg,
    pub __fpu_k1: __darwin_opmask_reg,
    pub __fpu_k2: __darwin_opmask_reg,
    pub __fpu_k3: __darwin_opmask_reg,
    pub __fpu_k4: __darwin_opmask_reg,
    pub __fpu_k5: __darwin_opmask_reg,
    pub __fpu_k6: __darwin_opmask_reg,
    pub __fpu_k7: __darwin_opmask_reg,
    pub __fpu_zmmh0: __darwin_ymm_reg,
    pub __fpu_zmmh1: __darwin_ymm_reg,
    pub __fpu_zmmh2: __darwin_ymm_reg,
    pub __fpu_zmmh3: __darwin_ymm_reg,
    pub __fpu_zmmh4: __darwin_ymm_reg,
    pub __fpu_zmmh5: __darwin_ymm_reg,
    pub __fpu_zmmh6: __darwin_ymm_reg,
    pub __fpu_zmmh7: __darwin_ymm_reg,
    pub __fpu_zmmh8: __darwin_ymm_reg,
    pub __fpu_zmmh9: __darwin_ymm_reg,
    pub __fpu_zmmh10: __darwin_ymm_reg,
    pub __fpu_zmmh11: __darwin_ymm_reg,
    pub __fpu_zmmh12: __darwin_ymm_reg,
    pub __fpu_zmmh13: __darwin_ymm_reg,
    pub __fpu_zmmh14: __darwin_ymm_reg,
    pub __fpu_zmmh15: __darwin_ymm_reg,
    pub __fpu_zmm16: __darwin_zmm_reg,
    pub __fpu_zmm17: __darwin_zmm_reg,
    pub __fpu_zmm18: __darwin_zmm_reg,
    pub __fpu_zmm19: __darwin_zmm_reg,
    pub __fpu_zmm20: __darwin_zmm_reg,
    pub __fpu_zmm21: __darwin_zmm_reg,
    pub __fpu_zmm22: __darwin_zmm_reg,
    pub __fpu_zmm23: __darwin_zmm_reg,
    pub __fpu_zmm24: __darwin_zmm_reg,
    pub __fpu_zmm25: __darwin_zmm_reg,
    pub __fpu_zmm26: __darwin_zmm_reg,
    pub __fpu_zmm27: __darwin_zmm_reg,
    pub __fpu_zmm28: __darwin_zmm_reg,
    pub __fpu_zmm29: __darwin_zmm_reg,
    pub __fpu_zmm30: __darwin_zmm_reg,
    pub __fpu_zmm31: __darwin_zmm_reg,
}
#[test]
fn bindgen_test_layout___darwin_x86_avx512_state64() {
    assert_eq!(
        ::std::mem::size_of::<__darwin_x86_avx512_state64>(),
        2444usize,
        concat!("Size of: ", stringify!(__darwin_x86_avx512_state64))
    );
    assert_eq!(
        ::std::mem::align_of::<__darwin_x86_avx512_state64>(),
        4usize,
        concat!("Alignment of ", stringify!(__darwin_x86_avx512_state64))
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<__darwin_x86_avx512_state64>())).__fpu_reserved as *const _
                as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(__darwin_x86_avx512_state64),
            "::",
            stringify!(__fpu_reserved)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<__darwin_x86_avx512_state64>())).__fpu_fcw as *const _ as usize
        },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(__darwin_x86_avx512_state64),
            "::",
            stringify!(__fpu_fcw)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<__darwin_x86_avx512_state64>())).__fpu_fsw as *const _ as usize
        },
        10usize,
        concat!(
            "Offset of field: ",
            stringify!(__darwin_x86_avx512_state64),
            "::",
            stringify!(__fpu_fsw)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<__darwin_x86_avx512_state64>())).__fpu_ftw as *const _ as usize
        },
        12usize,
        concat!(
            "Offset of field: ",
            stringify!(__darwin_x86_avx512_state64),
            "::",
            stringify!(__fpu_ftw)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<__darwin_x86_avx512_state64>())).__fpu_rsrv1 as *const _ as usize
        },
        13usize,
        concat!(
            "Offset of field: ",
            stringify!(__darwin_x86_avx512_state64),
            "::",
            stringify!(__fpu_rsrv1)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<__darwin_x86_avx512_state64>())).__fpu_fop as *const _ as usize
        },
        14usize,
        concat!(
            "Offset of field: ",
            stringify!(__darwin_x86_avx512_state64),
            "::",
            stringify!(__fpu_fop)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<__darwin_x86_avx512_state64>())).__fpu_ip as *const _ as usize
        },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(__darwin_x86_avx512_state64),
            "::",
            stringify!(__fpu_ip)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<__darwin_x86_avx512_state64>())).__fpu_cs as *const _ as usize
        },
        20usize,
        concat!(
            "Offset of field: ",
            stringify!(__darwin_x86_avx512_state64),
            "::",
            stringify!(__fpu_cs)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<__darwin_x86_avx512_state64>())).__fpu_rsrv2 as *const _ as usize
        },
        22usize,
        concat!(
            "Offset of field: ",
            stringify!(__darwin_x86_avx512_state64),
            "::",
            stringify!(__fpu_rsrv2)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<__darwin_x86_avx512_state64>())).__fpu_dp as *const _ as usize
        },
        24usize,
        concat!(
            "Offset of field: ",
            stringify!(__darwin_x86_avx512_state64),
            "::",
            stringify!(__fpu_dp)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<__darwin_x86_avx512_state64>())).__fpu_ds as *const _ as usize
        },
        28usize,
        concat!(
            "Offset of field: ",
            stringify!(__darwin_x86_avx512_state64),
            "::",
            stringify!(__fpu_ds)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<__darwin_x86_avx512_state64>())).__fpu_rsrv3 as *const _ as usize
        },
        30usize,
        concat!(
            "Offset of field: ",
            stringify!(__darwin_x86_avx512_state64),
            "::",
            stringify!(__fpu_rsrv3)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<__darwin_x86_avx512_state64>())).__fpu_mxcsr as *const _ as usize
        },
        32usize,
        concat!(
            "Offset of field: ",
            stringify!(__darwin_x86_avx512_state64),
            "::",
            stringify!(__fpu_mxcsr)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<__darwin_x86_avx512_state64>())).__fpu_mxcsrmask as *const _
                as usize
        },
        36usize,
        concat!(
            "Offset of field: ",
            stringify!(__darwin_x86_avx512_state64),
            "::",
            stringify!(__fpu_mxcsrmask)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<__darwin_x86_avx512_state64>())).__fpu_stmm0 as *const _ as usize
        },
        40usize,
        concat!(
            "Offset of field: ",
            stringify!(__darwin_x86_avx512_state64),
            "::",
            stringify!(__fpu_stmm0)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<__darwin_x86_avx512_state64>())).__fpu_stmm1 as *const _ as usize
        },
        56usize,
        concat!(
            "Offset of field: ",
            stringify!(__darwin_x86_avx512_state64),
            "::",
            stringify!(__fpu_stmm1)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<__darwin_x86_avx512_state64>())).__fpu_stmm2 as *const _ as usize
        },
        72usize,
        concat!(
            "Offset of field: ",
            stringify!(__darwin_x86_avx512_state64),
            "::",
            stringify!(__fpu_stmm2)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<__darwin_x86_avx512_state64>())).__fpu_stmm3 as *const _ as usize
        },
        88usize,
        concat!(
            "Offset of field: ",
            stringify!(__darwin_x86_avx512_state64),
            "::",
            stringify!(__fpu_stmm3)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<__darwin_x86_avx512_state64>())).__fpu_stmm4 as *const _ as usize
        },
        104usize,
        concat!(
            "Offset of field: ",
            stringify!(__darwin_x86_avx512_state64),
            "::",
            stringify!(__fpu_stmm4)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<__darwin_x86_avx512_state64>())).__fpu_stmm5 as *const _ as usize
        },
        120usize,
        concat!(
            "Offset of field: ",
            stringify!(__darwin_x86_avx512_state64),
            "::",
            stringify!(__fpu_stmm5)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<__darwin_x86_avx512_state64>())).__fpu_stmm6 as *const _ as usize
        },
        136usize,
        concat!(
            "Offset of field: ",
            stringify!(__darwin_x86_avx512_state64),
            "::",
            stringify!(__fpu_stmm6)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<__darwin_x86_avx512_state64>())).__fpu_stmm7 as *const _ as usize
        },
        152usize,
        concat!(
            "Offset of field: ",
            stringify!(__darwin_x86_avx512_state64),
            "::",
            stringify!(__fpu_stmm7)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<__darwin_x86_avx512_state64>())).__fpu_xmm0 as *const _ as usize
        },
        168usize,
        concat!(
            "Offset of field: ",
            stringify!(__darwin_x86_avx512_state64),
            "::",
            stringify!(__fpu_xmm0)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<__darwin_x86_avx512_state64>())).__fpu_xmm1 as *const _ as usize
        },
        184usize,
        concat!(
            "Offset of field: ",
            stringify!(__darwin_x86_avx512_state64),
            "::",
            stringify!(__fpu_xmm1)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<__darwin_x86_avx512_state64>())).__fpu_xmm2 as *const _ as usize
        },
        200usize,
        concat!(
            "Offset of field: ",
            stringify!(__darwin_x86_avx512_state64),
            "::",
            stringify!(__fpu_xmm2)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<__darwin_x86_avx512_state64>())).__fpu_xmm3 as *const _ as usize
        },
        216usize,
        concat!(
            "Offset of field: ",
            stringify!(__darwin_x86_avx512_state64),
            "::",
            stringify!(__fpu_xmm3)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<__darwin_x86_avx512_state64>())).__fpu_xmm4 as *const _ as usize
        },
        232usize,
        concat!(
            "Offset of field: ",
            stringify!(__darwin_x86_avx512_state64),
            "::",
            stringify!(__fpu_xmm4)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<__darwin_x86_avx512_state64>())).__fpu_xmm5 as *const _ as usize
        },
        248usize,
        concat!(
            "Offset of field: ",
            stringify!(__darwin_x86_avx512_state64),
            "::",
            stringify!(__fpu_xmm5)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<__darwin_x86_avx512_state64>())).__fpu_xmm6 as *const _ as usize
        },
        264usize,
        concat!(
            "Offset of field: ",
            stringify!(__darwin_x86_avx512_state64),
            "::",
            stringify!(__fpu_xmm6)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<__darwin_x86_avx512_state64>())).__fpu_xmm7 as *const _ as usize
        },
        280usize,
        concat!(
            "Offset of field: ",
            stringify!(__darwin_x86_avx512_state64),
            "::",
            stringify!(__fpu_xmm7)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<__darwin_x86_avx512_state64>())).__fpu_xmm8 as *const _ as usize
        },
        296usize,
        concat!(
            "Offset of field: ",
            stringify!(__darwin_x86_avx512_state64),
            "::",
            stringify!(__fpu_xmm8)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<__darwin_x86_avx512_state64>())).__fpu_xmm9 as *const _ as usize
        },
        312usize,
        concat!(
            "Offset of field: ",
            stringify!(__darwin_x86_avx512_state64),
            "::",
            stringify!(__fpu_xmm9)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<__darwin_x86_avx512_state64>())).__fpu_xmm10 as *const _ as usize
        },
        328usize,
        concat!(
            "Offset of field: ",
            stringify!(__darwin_x86_avx512_state64),
            "::",
            stringify!(__fpu_xmm10)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<__darwin_x86_avx512_state64>())).__fpu_xmm11 as *const _ as usize
        },
        344usize,
        concat!(
            "Offset of field: ",
            stringify!(__darwin_x86_avx512_state64),
            "::",
            stringify!(__fpu_xmm11)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<__darwin_x86_avx512_state64>())).__fpu_xmm12 as *const _ as usize
        },
        360usize,
        concat!(
            "Offset of field: ",
            stringify!(__darwin_x86_avx512_state64),
            "::",
            stringify!(__fpu_xmm12)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<__darwin_x86_avx512_state64>())).__fpu_xmm13 as *const _ as usize
        },
        376usize,
        concat!(
            "Offset of field: ",
            stringify!(__darwin_x86_avx512_state64),
            "::",
            stringify!(__fpu_xmm13)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<__darwin_x86_avx512_state64>())).__fpu_xmm14 as *const _ as usize
        },
        392usize,
        concat!(
            "Offset of field: ",
            stringify!(__darwin_x86_avx512_state64),
            "::",
            stringify!(__fpu_xmm14)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<__darwin_x86_avx512_state64>())).__fpu_xmm15 as *const _ as usize
        },
        408usize,
        concat!(
            "Offset of field: ",
            stringify!(__darwin_x86_avx512_state64),
            "::",
            stringify!(__fpu_xmm15)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<__darwin_x86_avx512_state64>())).__fpu_rsrv4 as *const _ as usize
        },
        424usize,
        concat!(
            "Offset of field: ",
            stringify!(__darwin_x86_avx512_state64),
            "::",
            stringify!(__fpu_rsrv4)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<__darwin_x86_avx512_state64>())).__fpu_reserved1 as *const _
                as usize
        },
        520usize,
        concat!(
            "Offset of field: ",
            stringify!(__darwin_x86_avx512_state64),
            "::",
            stringify!(__fpu_reserved1)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<__darwin_x86_avx512_state64>())).__avx_reserved1 as *const _
                as usize
        },
        524usize,
        concat!(
            "Offset of field: ",
            stringify!(__darwin_x86_avx512_state64),
            "::",
            stringify!(__avx_reserved1)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<__darwin_x86_avx512_state64>())).__fpu_ymmh0 as *const _ as usize
        },
        588usize,
        concat!(
            "Offset of field: ",
            stringify!(__darwin_x86_avx512_state64),
            "::",
            stringify!(__fpu_ymmh0)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<__darwin_x86_avx512_state64>())).__fpu_ymmh1 as *const _ as usize
        },
        604usize,
        concat!(
            "Offset of field: ",
            stringify!(__darwin_x86_avx512_state64),
            "::",
            stringify!(__fpu_ymmh1)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<__darwin_x86_avx512_state64>())).__fpu_ymmh2 as *const _ as usize
        },
        620usize,
        concat!(
            "Offset of field: ",
            stringify!(__darwin_x86_avx512_state64),
            "::",
            stringify!(__fpu_ymmh2)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<__darwin_x86_avx512_state64>())).__fpu_ymmh3 as *const _ as usize
        },
        636usize,
        concat!(
            "Offset of field: ",
            stringify!(__darwin_x86_avx512_state64),
            "::",
            stringify!(__fpu_ymmh3)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<__darwin_x86_avx512_state64>())).__fpu_ymmh4 as *const _ as usize
        },
        652usize,
        concat!(
            "Offset of field: ",
            stringify!(__darwin_x86_avx512_state64),
            "::",
            stringify!(__fpu_ymmh4)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<__darwin_x86_avx512_state64>())).__fpu_ymmh5 as *const _ as usize
        },
        668usize,
        concat!(
            "Offset of field: ",
            stringify!(__darwin_x86_avx512_state64),
            "::",
            stringify!(__fpu_ymmh5)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<__darwin_x86_avx512_state64>())).__fpu_ymmh6 as *const _ as usize
        },
        684usize,
        concat!(
            "Offset of field: ",
            stringify!(__darwin_x86_avx512_state64),
            "::",
            stringify!(__fpu_ymmh6)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<__darwin_x86_avx512_state64>())).__fpu_ymmh7 as *const _ as usize
        },
        700usize,
        concat!(
            "Offset of field: ",
            stringify!(__darwin_x86_avx512_state64),
            "::",
            stringify!(__fpu_ymmh7)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<__darwin_x86_avx512_state64>())).__fpu_ymmh8 as *const _ as usize
        },
        716usize,
        concat!(
            "Offset of field: ",
            stringify!(__darwin_x86_avx512_state64),
            "::",
            stringify!(__fpu_ymmh8)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<__darwin_x86_avx512_state64>())).__fpu_ymmh9 as *const _ as usize
        },
        732usize,
        concat!(
            "Offset of field: ",
            stringify!(__darwin_x86_avx512_state64),
            "::",
            stringify!(__fpu_ymmh9)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<__darwin_x86_avx512_state64>())).__fpu_ymmh10 as *const _
                as usize
        },
        748usize,
        concat!(
            "Offset of field: ",
            stringify!(__darwin_x86_avx512_state64),
            "::",
            stringify!(__fpu_ymmh10)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<__darwin_x86_avx512_state64>())).__fpu_ymmh11 as *const _
                as usize
        },
        764usize,
        concat!(
            "Offset of field: ",
            stringify!(__darwin_x86_avx512_state64),
            "::",
            stringify!(__fpu_ymmh11)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<__darwin_x86_avx512_state64>())).__fpu_ymmh12 as *const _
                as usize
        },
        780usize,
        concat!(
            "Offset of field: ",
            stringify!(__darwin_x86_avx512_state64),
            "::",
            stringify!(__fpu_ymmh12)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<__darwin_x86_avx512_state64>())).__fpu_ymmh13 as *const _
                as usize
        },
        796usize,
        concat!(
            "Offset of field: ",
            stringify!(__darwin_x86_avx512_state64),
            "::",
            stringify!(__fpu_ymmh13)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<__darwin_x86_avx512_state64>())).__fpu_ymmh14 as *const _
                as usize
        },
        812usize,
        concat!(
            "Offset of field: ",
            stringify!(__darwin_x86_avx512_state64),
            "::",
            stringify!(__fpu_ymmh14)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<__darwin_x86_avx512_state64>())).__fpu_ymmh15 as *const _
                as usize
        },
        828usize,
        concat!(
            "Offset of field: ",
            stringify!(__darwin_x86_avx512_state64),
            "::",
            stringify!(__fpu_ymmh15)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<__darwin_x86_avx512_state64>())).__fpu_k0 as *const _ as usize
        },
        844usize,
        concat!(
            "Offset of field: ",
            stringify!(__darwin_x86_avx512_state64),
            "::",
            stringify!(__fpu_k0)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<__darwin_x86_avx512_state64>())).__fpu_k1 as *const _ as usize
        },
        852usize,
        concat!(
            "Offset of field: ",
            stringify!(__darwin_x86_avx512_state64),
            "::",
            stringify!(__fpu_k1)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<__darwin_x86_avx512_state64>())).__fpu_k2 as *const _ as usize
        },
        860usize,
        concat!(
            "Offset of field: ",
            stringify!(__darwin_x86_avx512_state64),
            "::",
            stringify!(__fpu_k2)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<__darwin_x86_avx512_state64>())).__fpu_k3 as *const _ as usize
        },
        868usize,
        concat!(
            "Offset of field: ",
            stringify!(__darwin_x86_avx512_state64),
            "::",
            stringify!(__fpu_k3)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<__darwin_x86_avx512_state64>())).__fpu_k4 as *const _ as usize
        },
        876usize,
        concat!(
            "Offset of field: ",
            stringify!(__darwin_x86_avx512_state64),
            "::",
            stringify!(__fpu_k4)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<__darwin_x86_avx512_state64>())).__fpu_k5 as *const _ as usize
        },
        884usize,
        concat!(
            "Offset of field: ",
            stringify!(__darwin_x86_avx512_state64),
            "::",
            stringify!(__fpu_k5)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<__darwin_x86_avx512_state64>())).__fpu_k6 as *const _ as usize
        },
        892usize,
        concat!(
            "Offset of field: ",
            stringify!(__darwin_x86_avx512_state64),
            "::",
            stringify!(__fpu_k6)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<__darwin_x86_avx512_state64>())).__fpu_k7 as *const _ as usize
        },
        900usize,
        concat!(
            "Offset of field: ",
            stringify!(__darwin_x86_avx512_state64),
            "::",
            stringify!(__fpu_k7)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<__darwin_x86_avx512_state64>())).__fpu_zmmh0 as *const _ as usize
        },
        908usize,
        concat!(
            "Offset of field: ",
            stringify!(__darwin_x86_avx512_state64),
            "::",
            stringify!(__fpu_zmmh0)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<__darwin_x86_avx512_state64>())).__fpu_zmmh1 as *const _ as usize
        },
        940usize,
        concat!(
            "Offset of field: ",
            stringify!(__darwin_x86_avx512_state64),
            "::",
            stringify!(__fpu_zmmh1)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<__darwin_x86_avx512_state64>())).__fpu_zmmh2 as *const _ as usize
        },
        972usize,
        concat!(
            "Offset of field: ",
            stringify!(__darwin_x86_avx512_state64),
            "::",
            stringify!(__fpu_zmmh2)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<__darwin_x86_avx512_state64>())).__fpu_zmmh3 as *const _ as usize
        },
        1004usize,
        concat!(
            "Offset of field: ",
            stringify!(__darwin_x86_avx512_state64),
            "::",
            stringify!(__fpu_zmmh3)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<__darwin_x86_avx512_state64>())).__fpu_zmmh4 as *const _ as usize
        },
        1036usize,
        concat!(
            "Offset of field: ",
            stringify!(__darwin_x86_avx512_state64),
            "::",
            stringify!(__fpu_zmmh4)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<__darwin_x86_avx512_state64>())).__fpu_zmmh5 as *const _ as usize
        },
        1068usize,
        concat!(
            "Offset of field: ",
            stringify!(__darwin_x86_avx512_state64),
            "::",
            stringify!(__fpu_zmmh5)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<__darwin_x86_avx512_state64>())).__fpu_zmmh6 as *const _ as usize
        },
        1100usize,
        concat!(
            "Offset of field: ",
            stringify!(__darwin_x86_avx512_state64),
            "::",
            stringify!(__fpu_zmmh6)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<__darwin_x86_avx512_state64>())).__fpu_zmmh7 as *const _ as usize
        },
        1132usize,
        concat!(
            "Offset of field: ",
            stringify!(__darwin_x86_avx512_state64),
            "::",
            stringify!(__fpu_zmmh7)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<__darwin_x86_avx512_state64>())).__fpu_zmmh8 as *const _ as usize
        },
        1164usize,
        concat!(
            "Offset of field: ",
            stringify!(__darwin_x86_avx512_state64),
            "::",
            stringify!(__fpu_zmmh8)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<__darwin_x86_avx512_state64>())).__fpu_zmmh9 as *const _ as usize
        },
        1196usize,
        concat!(
            "Offset of field: ",
            stringify!(__darwin_x86_avx512_state64),
            "::",
            stringify!(__fpu_zmmh9)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<__darwin_x86_avx512_state64>())).__fpu_zmmh10 as *const _
                as usize
        },
        1228usize,
        concat!(
            "Offset of field: ",
            stringify!(__darwin_x86_avx512_state64),
            "::",
            stringify!(__fpu_zmmh10)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<__darwin_x86_avx512_state64>())).__fpu_zmmh11 as *const _
                as usize
        },
        1260usize,
        concat!(
            "Offset of field: ",
            stringify!(__darwin_x86_avx512_state64),
            "::",
            stringify!(__fpu_zmmh11)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<__darwin_x86_avx512_state64>())).__fpu_zmmh12 as *const _
                as usize
        },
        1292usize,
        concat!(
            "Offset of field: ",
            stringify!(__darwin_x86_avx512_state64),
            "::",
            stringify!(__fpu_zmmh12)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<__darwin_x86_avx512_state64>())).__fpu_zmmh13 as *const _
                as usize
        },
        1324usize,
        concat!(
            "Offset of field: ",
            stringify!(__darwin_x86_avx512_state64),
            "::",
            stringify!(__fpu_zmmh13)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<__darwin_x86_avx512_state64>())).__fpu_zmmh14 as *const _
                as usize
        },
        1356usize,
        concat!(
            "Offset of field: ",
            stringify!(__darwin_x86_avx512_state64),
            "::",
            stringify!(__fpu_zmmh14)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<__darwin_x86_avx512_state64>())).__fpu_zmmh15 as *const _
                as usize
        },
        1388usize,
        concat!(
            "Offset of field: ",
            stringify!(__darwin_x86_avx512_state64),
            "::",
            stringify!(__fpu_zmmh15)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<__darwin_x86_avx512_state64>())).__fpu_zmm16 as *const _ as usize
        },
        1420usize,
        concat!(
            "Offset of field: ",
            stringify!(__darwin_x86_avx512_state64),
            "::",
            stringify!(__fpu_zmm16)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<__darwin_x86_avx512_state64>())).__fpu_zmm17 as *const _ as usize
        },
        1484usize,
        concat!(
            "Offset of field: ",
            stringify!(__darwin_x86_avx512_state64),
            "::",
            stringify!(__fpu_zmm17)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<__darwin_x86_avx512_state64>())).__fpu_zmm18 as *const _ as usize
        },
        1548usize,
        concat!(
            "Offset of field: ",
            stringify!(__darwin_x86_avx512_state64),
            "::",
            stringify!(__fpu_zmm18)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<__darwin_x86_avx512_state64>())).__fpu_zmm19 as *const _ as usize
        },
        1612usize,
        concat!(
            "Offset of field: ",
            stringify!(__darwin_x86_avx512_state64),
            "::",
            stringify!(__fpu_zmm19)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<__darwin_x86_avx512_state64>())).__fpu_zmm20 as *const _ as usize
        },
        1676usize,
        concat!(
            "Offset of field: ",
            stringify!(__darwin_x86_avx512_state64),
            "::",
            stringify!(__fpu_zmm20)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<__darwin_x86_avx512_state64>())).__fpu_zmm21 as *const _ as usize
        },
        1740usize,
        concat!(
            "Offset of field: ",
            stringify!(__darwin_x86_avx512_state64),
            "::",
            stringify!(__fpu_zmm21)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<__darwin_x86_avx512_state64>())).__fpu_zmm22 as *const _ as usize
        },
        1804usize,
        concat!(
            "Offset of field: ",
            stringify!(__darwin_x86_avx512_state64),
            "::",
            stringify!(__fpu_zmm22)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<__darwin_x86_avx512_state64>())).__fpu_zmm23 as *const _ as usize
        },
        1868usize,
        concat!(
            "Offset of field: ",
            stringify!(__darwin_x86_avx512_state64),
            "::",
            stringify!(__fpu_zmm23)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<__darwin_x86_avx512_state64>())).__fpu_zmm24 as *const _ as usize
        },
        1932usize,
        concat!(
            "Offset of field: ",
            stringify!(__darwin_x86_avx512_state64),
            "::",
            stringify!(__fpu_zmm24)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<__darwin_x86_avx512_state64>())).__fpu_zmm25 as *const _ as usize
        },
        1996usize,
        concat!(
            "Offset of field: ",
            stringify!(__darwin_x86_avx512_state64),
            "::",
            stringify!(__fpu_zmm25)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<__darwin_x86_avx512_state64>())).__fpu_zmm26 as *const _ as usize
        },
        2060usize,
        concat!(
            "Offset of field: ",
            stringify!(__darwin_x86_avx512_state64),
            "::",
            stringify!(__fpu_zmm26)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<__darwin_x86_avx512_state64>())).__fpu_zmm27 as *const _ as usize
        },
        2124usize,
        concat!(
            "Offset of field: ",
            stringify!(__darwin_x86_avx512_state64),
            "::",
            stringify!(__fpu_zmm27)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<__darwin_x86_avx512_state64>())).__fpu_zmm28 as *const _ as usize
        },
        2188usize,
        concat!(
            "Offset of field: ",
            stringify!(__darwin_x86_avx512_state64),
            "::",
            stringify!(__fpu_zmm28)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<__darwin_x86_avx512_state64>())).__fpu_zmm29 as *const _ as usize
        },
        2252usize,
        concat!(
            "Offset of field: ",
            stringify!(__darwin_x86_avx512_state64),
            "::",
            stringify!(__fpu_zmm29)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<__darwin_x86_avx512_state64>())).__fpu_zmm30 as *const _ as usize
        },
        2316usize,
        concat!(
            "Offset of field: ",
            stringify!(__darwin_x86_avx512_state64),
            "::",
            stringify!(__fpu_zmm30)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<__darwin_x86_avx512_state64>())).__fpu_zmm31 as *const _ as usize
        },
        2380usize,
        concat!(
            "Offset of field: ",
            stringify!(__darwin_x86_avx512_state64),
            "::",
            stringify!(__fpu_zmm31)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __darwin_x86_exception_state64 {
    pub __trapno: __uint16_t,
    pub __cpu: __uint16_t,
    pub __err: __uint32_t,
    pub __faultvaddr: __uint64_t,
}
#[test]
fn bindgen_test_layout___darwin_x86_exception_state64() {
    assert_eq!(
        ::std::mem::size_of::<__darwin_x86_exception_state64>(),
        16usize,
        concat!("Size of: ", stringify!(__darwin_x86_exception_state64))
    );
    assert_eq!(
        ::std::mem::align_of::<__darwin_x86_exception_state64>(),
        8usize,
        concat!("Alignment of ", stringify!(__darwin_x86_exception_state64))
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<__darwin_x86_exception_state64>())).__trapno as *const _ as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(__darwin_x86_exception_state64),
            "::",
            stringify!(__trapno)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<__darwin_x86_exception_state64>())).__cpu as *const _ as usize
        },
        2usize,
        concat!(
            "Offset of field: ",
            stringify!(__darwin_x86_exception_state64),
            "::",
            stringify!(__cpu)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<__darwin_x86_exception_state64>())).__err as *const _ as usize
        },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(__darwin_x86_exception_state64),
            "::",
            stringify!(__err)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<__darwin_x86_exception_state64>())).__faultvaddr as *const _
                as usize
        },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(__darwin_x86_exception_state64),
            "::",
            stringify!(__faultvaddr)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __darwin_x86_debug_state64 {
    pub __dr0: __uint64_t,
    pub __dr1: __uint64_t,
    pub __dr2: __uint64_t,
    pub __dr3: __uint64_t,
    pub __dr4: __uint64_t,
    pub __dr5: __uint64_t,
    pub __dr6: __uint64_t,
    pub __dr7: __uint64_t,
}
#[test]
fn bindgen_test_layout___darwin_x86_debug_state64() {
    assert_eq!(
        ::std::mem::size_of::<__darwin_x86_debug_state64>(),
        64usize,
        concat!("Size of: ", stringify!(__darwin_x86_debug_state64))
    );
    assert_eq!(
        ::std::mem::align_of::<__darwin_x86_debug_state64>(),
        8usize,
        concat!("Alignment of ", stringify!(__darwin_x86_debug_state64))
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<__darwin_x86_debug_state64>())).__dr0 as *const _ as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(__darwin_x86_debug_state64),
            "::",
            stringify!(__dr0)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<__darwin_x86_debug_state64>())).__dr1 as *const _ as usize
        },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(__darwin_x86_debug_state64),
            "::",
            stringify!(__dr1)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<__darwin_x86_debug_state64>())).__dr2 as *const _ as usize
        },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(__darwin_x86_debug_state64),
            "::",
            stringify!(__dr2)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<__darwin_x86_debug_state64>())).__dr3 as *const _ as usize
        },
        24usize,
        concat!(
            "Offset of field: ",
            stringify!(__darwin_x86_debug_state64),
            "::",
            stringify!(__dr3)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<__darwin_x86_debug_state64>())).__dr4 as *const _ as usize
        },
        32usize,
        concat!(
            "Offset of field: ",
            stringify!(__darwin_x86_debug_state64),
            "::",
            stringify!(__dr4)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<__darwin_x86_debug_state64>())).__dr5 as *const _ as usize
        },
        40usize,
        concat!(
            "Offset of field: ",
            stringify!(__darwin_x86_debug_state64),
            "::",
            stringify!(__dr5)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<__darwin_x86_debug_state64>())).__dr6 as *const _ as usize
        },
        48usize,
        concat!(
            "Offset of field: ",
            stringify!(__darwin_x86_debug_state64),
            "::",
            stringify!(__dr6)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<__darwin_x86_debug_state64>())).__dr7 as *const _ as usize
        },
        56usize,
        concat!(
            "Offset of field: ",
            stringify!(__darwin_x86_debug_state64),
            "::",
            stringify!(__dr7)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __darwin_x86_cpmu_state64 {
    pub __ctrs: [__uint64_t; 16usize],
}
#[test]
fn bindgen_test_layout___darwin_x86_cpmu_state64() {
    assert_eq!(
        ::std::mem::size_of::<__darwin_x86_cpmu_state64>(),
        128usize,
        concat!("Size of: ", stringify!(__darwin_x86_cpmu_state64))
    );
    assert_eq!(
        ::std::mem::align_of::<__darwin_x86_cpmu_state64>(),
        8usize,
        concat!("Alignment of ", stringify!(__darwin_x86_cpmu_state64))
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<__darwin_x86_cpmu_state64>())).__ctrs as *const _ as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(__darwin_x86_cpmu_state64),
            "::",
            stringify!(__ctrs)
        )
    );
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct __darwin_mcontext32 {
    pub __es: __darwin_i386_exception_state,
    pub __ss: __darwin_i386_thread_state,
    pub __fs: __darwin_i386_float_state,
}
#[test]
fn bindgen_test_layout___darwin_mcontext32() {
    assert_eq!(
        ::std::mem::size_of::<__darwin_mcontext32>(),
        600usize,
        concat!("Size of: ", stringify!(__darwin_mcontext32))
    );
    assert_eq!(
        ::std::mem::align_of::<__darwin_mcontext32>(),
        4usize,
        concat!("Alignment of ", stringify!(__darwin_mcontext32))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<__darwin_mcontext32>())).__es as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(__darwin_mcontext32),
            "::",
            stringify!(__es)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<__darwin_mcontext32>())).__ss as *const _ as usize },
        12usize,
        concat!(
            "Offset of field: ",
            stringify!(__darwin_mcontext32),
            "::",
            stringify!(__ss)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<__darwin_mcontext32>())).__fs as *const _ as usize },
        76usize,
        concat!(
            "Offset of field: ",
            stringify!(__darwin_mcontext32),
            "::",
            stringify!(__fs)
        )
    );
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct __darwin_mcontext_avx32 {
    pub __es: __darwin_i386_exception_state,
    pub __ss: __darwin_i386_thread_state,
    pub __fs: __darwin_i386_avx_state,
}
#[test]
fn bindgen_test_layout___darwin_mcontext_avx32() {
    assert_eq!(
        ::std::mem::size_of::<__darwin_mcontext_avx32>(),
        792usize,
        concat!("Size of: ", stringify!(__darwin_mcontext_avx32))
    );
    assert_eq!(
        ::std::mem::align_of::<__darwin_mcontext_avx32>(),
        4usize,
        concat!("Alignment of ", stringify!(__darwin_mcontext_avx32))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<__darwin_mcontext_avx32>())).__es as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(__darwin_mcontext_avx32),
            "::",
            stringify!(__es)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<__darwin_mcontext_avx32>())).__ss as *const _ as usize },
        12usize,
        concat!(
            "Offset of field: ",
            stringify!(__darwin_mcontext_avx32),
            "::",
            stringify!(__ss)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<__darwin_mcontext_avx32>())).__fs as *const _ as usize },
        76usize,
        concat!(
            "Offset of field: ",
            stringify!(__darwin_mcontext_avx32),
            "::",
            stringify!(__fs)
        )
    );
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct __darwin_mcontext_avx512_32 {
    pub __es: __darwin_i386_exception_state,
    pub __ss: __darwin_i386_thread_state,
    pub __fs: __darwin_i386_avx512_state,
}
#[test]
fn bindgen_test_layout___darwin_mcontext_avx512_32() {
    assert_eq!(
        ::std::mem::size_of::<__darwin_mcontext_avx512_32>(),
        1112usize,
        concat!("Size of: ", stringify!(__darwin_mcontext_avx512_32))
    );
    assert_eq!(
        ::std::mem::align_of::<__darwin_mcontext_avx512_32>(),
        4usize,
        concat!("Alignment of ", stringify!(__darwin_mcontext_avx512_32))
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<__darwin_mcontext_avx512_32>())).__es as *const _ as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(__darwin_mcontext_avx512_32),
            "::",
            stringify!(__es)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<__darwin_mcontext_avx512_32>())).__ss as *const _ as usize
        },
        12usize,
        concat!(
            "Offset of field: ",
            stringify!(__darwin_mcontext_avx512_32),
            "::",
            stringify!(__ss)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<__darwin_mcontext_avx512_32>())).__fs as *const _ as usize
        },
        76usize,
        concat!(
            "Offset of field: ",
            stringify!(__darwin_mcontext_avx512_32),
            "::",
            stringify!(__fs)
        )
    );
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct __darwin_mcontext64 {
    pub __es: __darwin_x86_exception_state64,
    pub __ss: __darwin_x86_thread_state64,
    pub __fs: __darwin_x86_float_state64,
}
#[test]
fn bindgen_test_layout___darwin_mcontext64() {
    assert_eq!(
        ::std::mem::size_of::<__darwin_mcontext64>(),
        712usize,
        concat!("Size of: ", stringify!(__darwin_mcontext64))
    );
    assert_eq!(
        ::std::mem::align_of::<__darwin_mcontext64>(),
        8usize,
        concat!("Alignment of ", stringify!(__darwin_mcontext64))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<__darwin_mcontext64>())).__es as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(__darwin_mcontext64),
            "::",
            stringify!(__es)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<__darwin_mcontext64>())).__ss as *const _ as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(__darwin_mcontext64),
            "::",
            stringify!(__ss)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<__darwin_mcontext64>())).__fs as *const _ as usize },
        184usize,
        concat!(
            "Offset of field: ",
            stringify!(__darwin_mcontext64),
            "::",
            stringify!(__fs)
        )
    );
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct __darwin_mcontext_avx64 {
    pub __es: __darwin_x86_exception_state64,
    pub __ss: __darwin_x86_thread_state64,
    pub __fs: __darwin_x86_avx_state64,
}
#[test]
fn bindgen_test_layout___darwin_mcontext_avx64() {
    assert_eq!(
        ::std::mem::size_of::<__darwin_mcontext_avx64>(),
        1032usize,
        concat!("Size of: ", stringify!(__darwin_mcontext_avx64))
    );
    assert_eq!(
        ::std::mem::align_of::<__darwin_mcontext_avx64>(),
        8usize,
        concat!("Alignment of ", stringify!(__darwin_mcontext_avx64))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<__darwin_mcontext_avx64>())).__es as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(__darwin_mcontext_avx64),
            "::",
            stringify!(__es)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<__darwin_mcontext_avx64>())).__ss as *const _ as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(__darwin_mcontext_avx64),
            "::",
            stringify!(__ss)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<__darwin_mcontext_avx64>())).__fs as *const _ as usize },
        184usize,
        concat!(
            "Offset of field: ",
            stringify!(__darwin_mcontext_avx64),
            "::",
            stringify!(__fs)
        )
    );
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct __darwin_mcontext_avx512_64 {
    pub __es: __darwin_x86_exception_state64,
    pub __ss: __darwin_x86_thread_state64,
    pub __fs: __darwin_x86_avx512_state64,
}
#[test]
fn bindgen_test_layout___darwin_mcontext_avx512_64() {
    assert_eq!(
        ::std::mem::size_of::<__darwin_mcontext_avx512_64>(),
        2632usize,
        concat!("Size of: ", stringify!(__darwin_mcontext_avx512_64))
    );
    assert_eq!(
        ::std::mem::align_of::<__darwin_mcontext_avx512_64>(),
        8usize,
        concat!("Alignment of ", stringify!(__darwin_mcontext_avx512_64))
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<__darwin_mcontext_avx512_64>())).__es as *const _ as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(__darwin_mcontext_avx512_64),
            "::",
            stringify!(__es)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<__darwin_mcontext_avx512_64>())).__ss as *const _ as usize
        },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(__darwin_mcontext_avx512_64),
            "::",
            stringify!(__ss)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<__darwin_mcontext_avx512_64>())).__fs as *const _ as usize
        },
        184usize,
        concat!(
            "Offset of field: ",
            stringify!(__darwin_mcontext_avx512_64),
            "::",
            stringify!(__fs)
        )
    );
}
pub type mcontext_t = *mut __darwin_mcontext64;
pub type pthread_attr_t = __darwin_pthread_attr_t;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __darwin_sigaltstack {
    pub ss_sp: *mut ::std::os::raw::c_void,
    pub ss_size: __darwin_size_t,
    pub ss_flags: ::std::os::raw::c_int,
}
#[test]
fn bindgen_test_layout___darwin_sigaltstack() {
    assert_eq!(
        ::std::mem::size_of::<__darwin_sigaltstack>(),
        24usize,
        concat!("Size of: ", stringify!(__darwin_sigaltstack))
    );
    assert_eq!(
        ::std::mem::align_of::<__darwin_sigaltstack>(),
        8usize,
        concat!("Alignment of ", stringify!(__darwin_sigaltstack))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<__darwin_sigaltstack>())).ss_sp as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(__darwin_sigaltstack),
            "::",
            stringify!(ss_sp)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<__darwin_sigaltstack>())).ss_size as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(__darwin_sigaltstack),
            "::",
            stringify!(ss_size)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<__darwin_sigaltstack>())).ss_flags as *const _ as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(__darwin_sigaltstack),
            "::",
            stringify!(ss_flags)
        )
    );
}
pub type stack_t = __darwin_sigaltstack;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __darwin_ucontext {
    pub uc_onstack: ::std::os::raw::c_int,
    pub uc_sigmask: __darwin_sigset_t,
    pub uc_stack: __darwin_sigaltstack,
    pub uc_link: *mut __darwin_ucontext,
    pub uc_mcsize: __darwin_size_t,
    pub uc_mcontext: *mut __darwin_mcontext64,
}
#[test]
fn bindgen_test_layout___darwin_ucontext() {
    assert_eq!(
        ::std::mem::size_of::<__darwin_ucontext>(),
        56usize,
        concat!("Size of: ", stringify!(__darwin_ucontext))
    );
    assert_eq!(
        ::std::mem::align_of::<__darwin_ucontext>(),
        8usize,
        concat!("Alignment of ", stringify!(__darwin_ucontext))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<__darwin_ucontext>())).uc_onstack as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(__darwin_ucontext),
            "::",
            stringify!(uc_onstack)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<__darwin_ucontext>())).uc_sigmask as *const _ as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(__darwin_ucontext),
            "::",
            stringify!(uc_sigmask)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<__darwin_ucontext>())).uc_stack as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(__darwin_ucontext),
            "::",
            stringify!(uc_stack)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<__darwin_ucontext>())).uc_link as *const _ as usize },
        32usize,
        concat!(
            "Offset of field: ",
            stringify!(__darwin_ucontext),
            "::",
            stringify!(uc_link)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<__darwin_ucontext>())).uc_mcsize as *const _ as usize },
        40usize,
        concat!(
            "Offset of field: ",
            stringify!(__darwin_ucontext),
            "::",
            stringify!(uc_mcsize)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<__darwin_ucontext>())).uc_mcontext as *const _ as usize },
        48usize,
        concat!(
            "Offset of field: ",
            stringify!(__darwin_ucontext),
            "::",
            stringify!(uc_mcontext)
        )
    );
}
pub type ucontext_t = __darwin_ucontext;
pub type sigset_t = __darwin_sigset_t;
pub type size_t = __darwin_size_t;
pub type uid_t = __darwin_uid_t;
#[repr(C)]
#[derive(Copy, Clone)]
pub union sigval {
    pub sival_int: ::std::os::raw::c_int,
    pub sival_ptr: *mut ::std::os::raw::c_void,
    _bindgen_union_align: u64,
}
#[test]
fn bindgen_test_layout_sigval() {
    assert_eq!(
        ::std::mem::size_of::<sigval>(),
        8usize,
        concat!("Size of: ", stringify!(sigval))
    );
    assert_eq!(
        ::std::mem::align_of::<sigval>(),
        8usize,
        concat!("Alignment of ", stringify!(sigval))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<sigval>())).sival_int as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(sigval),
            "::",
            stringify!(sival_int)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<sigval>())).sival_ptr as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(sigval),
            "::",
            stringify!(sival_ptr)
        )
    );
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sigevent {
    pub sigev_notify: ::std::os::raw::c_int,
    pub sigev_signo: ::std::os::raw::c_int,
    pub sigev_value: sigval,
    pub sigev_notify_function: ::std::option::Option<unsafe extern "C" fn(arg1: sigval)>,
    pub sigev_notify_attributes: *mut pthread_attr_t,
}
#[test]
fn bindgen_test_layout_sigevent() {
    assert_eq!(
        ::std::mem::size_of::<sigevent>(),
        32usize,
        concat!("Size of: ", stringify!(sigevent))
    );
    assert_eq!(
        ::std::mem::align_of::<sigevent>(),
        8usize,
        concat!("Alignment of ", stringify!(sigevent))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<sigevent>())).sigev_notify as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(sigevent),
            "::",
            stringify!(sigev_notify)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<sigevent>())).sigev_signo as *const _ as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(sigevent),
            "::",
            stringify!(sigev_signo)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<sigevent>())).sigev_value as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(sigevent),
            "::",
            stringify!(sigev_value)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<sigevent>())).sigev_notify_function as *const _ as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(sigevent),
            "::",
            stringify!(sigev_notify_function)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<sigevent>())).sigev_notify_attributes as *const _ as usize
        },
        24usize,
        concat!(
            "Offset of field: ",
            stringify!(sigevent),
            "::",
            stringify!(sigev_notify_attributes)
        )
    );
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct __siginfo {
    pub si_signo: ::std::os::raw::c_int,
    pub si_errno: ::std::os::raw::c_int,
    pub si_code: ::std::os::raw::c_int,
    pub si_pid: pid_t,
    pub si_uid: uid_t,
    pub si_status: ::std::os::raw::c_int,
    pub si_addr: *mut ::std::os::raw::c_void,
    pub si_value: sigval,
    pub si_band: ::std::os::raw::c_long,
    pub __pad: [::std::os::raw::c_ulong; 7usize],
}
#[test]
fn bindgen_test_layout___siginfo() {
    assert_eq!(
        ::std::mem::size_of::<__siginfo>(),
        104usize,
        concat!("Size of: ", stringify!(__siginfo))
    );
    assert_eq!(
        ::std::mem::align_of::<__siginfo>(),
        8usize,
        concat!("Alignment of ", stringify!(__siginfo))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<__siginfo>())).si_signo as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(__siginfo),
            "::",
            stringify!(si_signo)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<__siginfo>())).si_errno as *const _ as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(__siginfo),
            "::",
            stringify!(si_errno)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<__siginfo>())).si_code as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(__siginfo),
            "::",
            stringify!(si_code)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<__siginfo>())).si_pid as *const _ as usize },
        12usize,
        concat!(
            "Offset of field: ",
            stringify!(__siginfo),
            "::",
            stringify!(si_pid)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<__siginfo>())).si_uid as *const _ as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(__siginfo),
            "::",
            stringify!(si_uid)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<__siginfo>())).si_status as *const _ as usize },
        20usize,
        concat!(
            "Offset of field: ",
            stringify!(__siginfo),
            "::",
            stringify!(si_status)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<__siginfo>())).si_addr as *const _ as usize },
        24usize,
        concat!(
            "Offset of field: ",
            stringify!(__siginfo),
            "::",
            stringify!(si_addr)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<__siginfo>())).si_value as *const _ as usize },
        32usize,
        concat!(
            "Offset of field: ",
            stringify!(__siginfo),
            "::",
            stringify!(si_value)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<__siginfo>())).si_band as *const _ as usize },
        40usize,
        concat!(
            "Offset of field: ",
            stringify!(__siginfo),
            "::",
            stringify!(si_band)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<__siginfo>())).__pad as *const _ as usize },
        48usize,
        concat!(
            "Offset of field: ",
            stringify!(__siginfo),
            "::",
            stringify!(__pad)
        )
    );
}
pub type siginfo_t = __siginfo;
#[repr(C)]
#[derive(Copy, Clone)]
pub union __sigaction_u {
    pub __sa_handler: ::std::option::Option<unsafe extern "C" fn(arg1: ::std::os::raw::c_int)>,
    pub __sa_sigaction: ::std::option::Option<
        unsafe extern "C" fn(
            arg1: ::std::os::raw::c_int,
            arg2: *mut __siginfo,
            arg3: *mut ::std::os::raw::c_void,
        ),
    >,
    _bindgen_union_align: u64,
}
#[test]
fn bindgen_test_layout___sigaction_u() {
    assert_eq!(
        ::std::mem::size_of::<__sigaction_u>(),
        8usize,
        concat!("Size of: ", stringify!(__sigaction_u))
    );
    assert_eq!(
        ::std::mem::align_of::<__sigaction_u>(),
        8usize,
        concat!("Alignment of ", stringify!(__sigaction_u))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<__sigaction_u>())).__sa_handler as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(__sigaction_u),
            "::",
            stringify!(__sa_handler)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<__sigaction_u>())).__sa_sigaction as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(__sigaction_u),
            "::",
            stringify!(__sa_sigaction)
        )
    );
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct __sigaction {
    pub __sigaction_u: __sigaction_u,
    pub sa_tramp: ::std::option::Option<
        unsafe extern "C" fn(
            arg1: *mut ::std::os::raw::c_void,
            arg2: ::std::os::raw::c_int,
            arg3: ::std::os::raw::c_int,
            arg4: *mut siginfo_t,
            arg5: *mut ::std::os::raw::c_void,
        ),
    >,
    pub sa_mask: sigset_t,
    pub sa_flags: ::std::os::raw::c_int,
}
#[test]
fn bindgen_test_layout___sigaction() {
    assert_eq!(
        ::std::mem::size_of::<__sigaction>(),
        24usize,
        concat!("Size of: ", stringify!(__sigaction))
    );
    assert_eq!(
        ::std::mem::align_of::<__sigaction>(),
        8usize,
        concat!("Alignment of ", stringify!(__sigaction))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<__sigaction>())).__sigaction_u as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(__sigaction),
            "::",
            stringify!(__sigaction_u)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<__sigaction>())).sa_tramp as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(__sigaction),
            "::",
            stringify!(sa_tramp)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<__sigaction>())).sa_mask as *const _ as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(__sigaction),
            "::",
            stringify!(sa_mask)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<__sigaction>())).sa_flags as *const _ as usize },
        20usize,
        concat!(
            "Offset of field: ",
            stringify!(__sigaction),
            "::",
            stringify!(sa_flags)
        )
    );
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sigaction {
    pub __sigaction_u: __sigaction_u,
    pub sa_mask: sigset_t,
    pub sa_flags: ::std::os::raw::c_int,
}
#[test]
fn bindgen_test_layout_sigaction() {
    assert_eq!(
        ::std::mem::size_of::<sigaction>(),
        16usize,
        concat!("Size of: ", stringify!(sigaction))
    );
    assert_eq!(
        ::std::mem::align_of::<sigaction>(),
        8usize,
        concat!("Alignment of ", stringify!(sigaction))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<sigaction>())).__sigaction_u as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(sigaction),
            "::",
            stringify!(__sigaction_u)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<sigaction>())).sa_mask as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(sigaction),
            "::",
            stringify!(sa_mask)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<sigaction>())).sa_flags as *const _ as usize },
        12usize,
        concat!(
            "Offset of field: ",
            stringify!(sigaction),
            "::",
            stringify!(sa_flags)
        )
    );
}
pub type sig_t = ::std::option::Option<unsafe extern "C" fn(arg1: ::std::os::raw::c_int)>;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct sigvec {
    pub sv_handler: ::std::option::Option<unsafe extern "C" fn(arg1: ::std::os::raw::c_int)>,
    pub sv_mask: ::std::os::raw::c_int,
    pub sv_flags: ::std::os::raw::c_int,
}
#[test]
fn bindgen_test_layout_sigvec() {
    assert_eq!(
        ::std::mem::size_of::<sigvec>(),
        16usize,
        concat!("Size of: ", stringify!(sigvec))
    );
    assert_eq!(
        ::std::mem::align_of::<sigvec>(),
        8usize,
        concat!("Alignment of ", stringify!(sigvec))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<sigvec>())).sv_handler as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(sigvec),
            "::",
            stringify!(sv_handler)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<sigvec>())).sv_mask as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(sigvec),
            "::",
            stringify!(sv_mask)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<sigvec>())).sv_flags as *const _ as usize },
        12usize,
        concat!(
            "Offset of field: ",
            stringify!(sigvec),
            "::",
            stringify!(sv_flags)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct sigstack {
    pub ss_sp: *mut ::std::os::raw::c_char,
    pub ss_onstack: ::std::os::raw::c_int,
}
#[test]
fn bindgen_test_layout_sigstack() {
    assert_eq!(
        ::std::mem::size_of::<sigstack>(),
        16usize,
        concat!("Size of: ", stringify!(sigstack))
    );
    assert_eq!(
        ::std::mem::align_of::<sigstack>(),
        8usize,
        concat!("Alignment of ", stringify!(sigstack))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<sigstack>())).ss_sp as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(sigstack),
            "::",
            stringify!(ss_sp)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<sigstack>())).ss_onstack as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(sigstack),
            "::",
            stringify!(ss_onstack)
        )
    );
}
extern "C" {
    pub fn signal(
        arg1: ::std::os::raw::c_int,
        arg2: ::std::option::Option<unsafe extern "C" fn(arg1: ::std::os::raw::c_int)>,
    ) -> ::std::option::Option<
        unsafe extern "C" fn(
            arg1: ::std::os::raw::c_int,
            arg2: ::std::option::Option<unsafe extern "C" fn(arg1: ::std::os::raw::c_int)>,
        ),
    >;
}
pub type int_least8_t = i8;
pub type int_least16_t = i16;
pub type int_least32_t = i32;
pub type int_least64_t = i64;
pub type uint_least8_t = u8;
pub type uint_least16_t = u16;
pub type uint_least32_t = u32;
pub type uint_least64_t = u64;
pub type int_fast8_t = i8;
pub type int_fast16_t = i16;
pub type int_fast32_t = i32;
pub type int_fast64_t = i64;
pub type uint_fast8_t = u8;
pub type uint_fast16_t = u16;
pub type uint_fast32_t = u32;
pub type uint_fast64_t = u64;
pub type intmax_t = ::std::os::raw::c_long;
pub type uintmax_t = ::std::os::raw::c_ulong;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct timeval {
    pub tv_sec: __darwin_time_t,
    pub tv_usec: __darwin_suseconds_t,
}
#[test]
fn bindgen_test_layout_timeval() {
    assert_eq!(
        ::std::mem::size_of::<timeval>(),
        16usize,
        concat!("Size of: ", stringify!(timeval))
    );
    assert_eq!(
        ::std::mem::align_of::<timeval>(),
        8usize,
        concat!("Alignment of ", stringify!(timeval))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<timeval>())).tv_sec as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(timeval),
            "::",
            stringify!(tv_sec)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<timeval>())).tv_usec as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(timeval),
            "::",
            stringify!(tv_usec)
        )
    );
}
pub type rlim_t = __uint64_t;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct rusage {
    pub ru_utime: timeval,
    pub ru_stime: timeval,
    pub ru_maxrss: ::std::os::raw::c_long,
    pub ru_ixrss: ::std::os::raw::c_long,
    pub ru_idrss: ::std::os::raw::c_long,
    pub ru_isrss: ::std::os::raw::c_long,
    pub ru_minflt: ::std::os::raw::c_long,
    pub ru_majflt: ::std::os::raw::c_long,
    pub ru_nswap: ::std::os::raw::c_long,
    pub ru_inblock: ::std::os::raw::c_long,
    pub ru_oublock: ::std::os::raw::c_long,
    pub ru_msgsnd: ::std::os::raw::c_long,
    pub ru_msgrcv: ::std::os::raw::c_long,
    pub ru_nsignals: ::std::os::raw::c_long,
    pub ru_nvcsw: ::std::os::raw::c_long,
    pub ru_nivcsw: ::std::os::raw::c_long,
}
#[test]
fn bindgen_test_layout_rusage() {
    assert_eq!(
        ::std::mem::size_of::<rusage>(),
        144usize,
        concat!("Size of: ", stringify!(rusage))
    );
    assert_eq!(
        ::std::mem::align_of::<rusage>(),
        8usize,
        concat!("Alignment of ", stringify!(rusage))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<rusage>())).ru_utime as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(rusage),
            "::",
            stringify!(ru_utime)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<rusage>())).ru_stime as *const _ as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(rusage),
            "::",
            stringify!(ru_stime)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<rusage>())).ru_maxrss as *const _ as usize },
        32usize,
        concat!(
            "Offset of field: ",
            stringify!(rusage),
            "::",
            stringify!(ru_maxrss)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<rusage>())).ru_ixrss as *const _ as usize },
        40usize,
        concat!(
            "Offset of field: ",
            stringify!(rusage),
            "::",
            stringify!(ru_ixrss)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<rusage>())).ru_idrss as *const _ as usize },
        48usize,
        concat!(
            "Offset of field: ",
            stringify!(rusage),
            "::",
            stringify!(ru_idrss)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<rusage>())).ru_isrss as *const _ as usize },
        56usize,
        concat!(
            "Offset of field: ",
            stringify!(rusage),
            "::",
            stringify!(ru_isrss)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<rusage>())).ru_minflt as *const _ as usize },
        64usize,
        concat!(
            "Offset of field: ",
            stringify!(rusage),
            "::",
            stringify!(ru_minflt)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<rusage>())).ru_majflt as *const _ as usize },
        72usize,
        concat!(
            "Offset of field: ",
            stringify!(rusage),
            "::",
            stringify!(ru_majflt)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<rusage>())).ru_nswap as *const _ as usize },
        80usize,
        concat!(
            "Offset of field: ",
            stringify!(rusage),
            "::",
            stringify!(ru_nswap)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<rusage>())).ru_inblock as *const _ as usize },
        88usize,
        concat!(
            "Offset of field: ",
            stringify!(rusage),
            "::",
            stringify!(ru_inblock)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<rusage>())).ru_oublock as *const _ as usize },
        96usize,
        concat!(
            "Offset of field: ",
            stringify!(rusage),
            "::",
            stringify!(ru_oublock)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<rusage>())).ru_msgsnd as *const _ as usize },
        104usize,
        concat!(
            "Offset of field: ",
            stringify!(rusage),
            "::",
            stringify!(ru_msgsnd)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<rusage>())).ru_msgrcv as *const _ as usize },
        112usize,
        concat!(
            "Offset of field: ",
            stringify!(rusage),
            "::",
            stringify!(ru_msgrcv)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<rusage>())).ru_nsignals as *const _ as usize },
        120usize,
        concat!(
            "Offset of field: ",
            stringify!(rusage),
            "::",
            stringify!(ru_nsignals)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<rusage>())).ru_nvcsw as *const _ as usize },
        128usize,
        concat!(
            "Offset of field: ",
            stringify!(rusage),
            "::",
            stringify!(ru_nvcsw)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<rusage>())).ru_nivcsw as *const _ as usize },
        136usize,
        concat!(
            "Offset of field: ",
            stringify!(rusage),
            "::",
            stringify!(ru_nivcsw)
        )
    );
}
pub type rusage_info_t = *mut ::std::os::raw::c_void;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct rusage_info_v0 {
    pub ri_uuid: [u8; 16usize],
    pub ri_user_time: u64,
    pub ri_system_time: u64,
    pub ri_pkg_idle_wkups: u64,
    pub ri_interrupt_wkups: u64,
    pub ri_pageins: u64,
    pub ri_wired_size: u64,
    pub ri_resident_size: u64,
    pub ri_phys_footprint: u64,
    pub ri_proc_start_abstime: u64,
    pub ri_proc_exit_abstime: u64,
}
#[test]
fn bindgen_test_layout_rusage_info_v0() {
    assert_eq!(
        ::std::mem::size_of::<rusage_info_v0>(),
        96usize,
        concat!("Size of: ", stringify!(rusage_info_v0))
    );
    assert_eq!(
        ::std::mem::align_of::<rusage_info_v0>(),
        8usize,
        concat!("Alignment of ", stringify!(rusage_info_v0))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<rusage_info_v0>())).ri_uuid as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(rusage_info_v0),
            "::",
            stringify!(ri_uuid)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<rusage_info_v0>())).ri_user_time as *const _ as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(rusage_info_v0),
            "::",
            stringify!(ri_user_time)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<rusage_info_v0>())).ri_system_time as *const _ as usize },
        24usize,
        concat!(
            "Offset of field: ",
            stringify!(rusage_info_v0),
            "::",
            stringify!(ri_system_time)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<rusage_info_v0>())).ri_pkg_idle_wkups as *const _ as usize
        },
        32usize,
        concat!(
            "Offset of field: ",
            stringify!(rusage_info_v0),
            "::",
            stringify!(ri_pkg_idle_wkups)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<rusage_info_v0>())).ri_interrupt_wkups as *const _ as usize
        },
        40usize,
        concat!(
            "Offset of field: ",
            stringify!(rusage_info_v0),
            "::",
            stringify!(ri_interrupt_wkups)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<rusage_info_v0>())).ri_pageins as *const _ as usize },
        48usize,
        concat!(
            "Offset of field: ",
            stringify!(rusage_info_v0),
            "::",
            stringify!(ri_pageins)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<rusage_info_v0>())).ri_wired_size as *const _ as usize },
        56usize,
        concat!(
            "Offset of field: ",
            stringify!(rusage_info_v0),
            "::",
            stringify!(ri_wired_size)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<rusage_info_v0>())).ri_resident_size as *const _ as usize },
        64usize,
        concat!(
            "Offset of field: ",
            stringify!(rusage_info_v0),
            "::",
            stringify!(ri_resident_size)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<rusage_info_v0>())).ri_phys_footprint as *const _ as usize
        },
        72usize,
        concat!(
            "Offset of field: ",
            stringify!(rusage_info_v0),
            "::",
            stringify!(ri_phys_footprint)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<rusage_info_v0>())).ri_proc_start_abstime as *const _ as usize
        },
        80usize,
        concat!(
            "Offset of field: ",
            stringify!(rusage_info_v0),
            "::",
            stringify!(ri_proc_start_abstime)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<rusage_info_v0>())).ri_proc_exit_abstime as *const _ as usize
        },
        88usize,
        concat!(
            "Offset of field: ",
            stringify!(rusage_info_v0),
            "::",
            stringify!(ri_proc_exit_abstime)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct rusage_info_v1 {
    pub ri_uuid: [u8; 16usize],
    pub ri_user_time: u64,
    pub ri_system_time: u64,
    pub ri_pkg_idle_wkups: u64,
    pub ri_interrupt_wkups: u64,
    pub ri_pageins: u64,
    pub ri_wired_size: u64,
    pub ri_resident_size: u64,
    pub ri_phys_footprint: u64,
    pub ri_proc_start_abstime: u64,
    pub ri_proc_exit_abstime: u64,
    pub ri_child_user_time: u64,
    pub ri_child_system_time: u64,
    pub ri_child_pkg_idle_wkups: u64,
    pub ri_child_interrupt_wkups: u64,
    pub ri_child_pageins: u64,
    pub ri_child_elapsed_abstime: u64,
}
#[test]
fn bindgen_test_layout_rusage_info_v1() {
    assert_eq!(
        ::std::mem::size_of::<rusage_info_v1>(),
        144usize,
        concat!("Size of: ", stringify!(rusage_info_v1))
    );
    assert_eq!(
        ::std::mem::align_of::<rusage_info_v1>(),
        8usize,
        concat!("Alignment of ", stringify!(rusage_info_v1))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<rusage_info_v1>())).ri_uuid as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(rusage_info_v1),
            "::",
            stringify!(ri_uuid)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<rusage_info_v1>())).ri_user_time as *const _ as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(rusage_info_v1),
            "::",
            stringify!(ri_user_time)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<rusage_info_v1>())).ri_system_time as *const _ as usize },
        24usize,
        concat!(
            "Offset of field: ",
            stringify!(rusage_info_v1),
            "::",
            stringify!(ri_system_time)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<rusage_info_v1>())).ri_pkg_idle_wkups as *const _ as usize
        },
        32usize,
        concat!(
            "Offset of field: ",
            stringify!(rusage_info_v1),
            "::",
            stringify!(ri_pkg_idle_wkups)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<rusage_info_v1>())).ri_interrupt_wkups as *const _ as usize
        },
        40usize,
        concat!(
            "Offset of field: ",
            stringify!(rusage_info_v1),
            "::",
            stringify!(ri_interrupt_wkups)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<rusage_info_v1>())).ri_pageins as *const _ as usize },
        48usize,
        concat!(
            "Offset of field: ",
            stringify!(rusage_info_v1),
            "::",
            stringify!(ri_pageins)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<rusage_info_v1>())).ri_wired_size as *const _ as usize },
        56usize,
        concat!(
            "Offset of field: ",
            stringify!(rusage_info_v1),
            "::",
            stringify!(ri_wired_size)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<rusage_info_v1>())).ri_resident_size as *const _ as usize },
        64usize,
        concat!(
            "Offset of field: ",
            stringify!(rusage_info_v1),
            "::",
            stringify!(ri_resident_size)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<rusage_info_v1>())).ri_phys_footprint as *const _ as usize
        },
        72usize,
        concat!(
            "Offset of field: ",
            stringify!(rusage_info_v1),
            "::",
            stringify!(ri_phys_footprint)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<rusage_info_v1>())).ri_proc_start_abstime as *const _ as usize
        },
        80usize,
        concat!(
            "Offset of field: ",
            stringify!(rusage_info_v1),
            "::",
            stringify!(ri_proc_start_abstime)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<rusage_info_v1>())).ri_proc_exit_abstime as *const _ as usize
        },
        88usize,
        concat!(
            "Offset of field: ",
            stringify!(rusage_info_v1),
            "::",
            stringify!(ri_proc_exit_abstime)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<rusage_info_v1>())).ri_child_user_time as *const _ as usize
        },
        96usize,
        concat!(
            "Offset of field: ",
            stringify!(rusage_info_v1),
            "::",
            stringify!(ri_child_user_time)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<rusage_info_v1>())).ri_child_system_time as *const _ as usize
        },
        104usize,
        concat!(
            "Offset of field: ",
            stringify!(rusage_info_v1),
            "::",
            stringify!(ri_child_system_time)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<rusage_info_v1>())).ri_child_pkg_idle_wkups as *const _ as usize
        },
        112usize,
        concat!(
            "Offset of field: ",
            stringify!(rusage_info_v1),
            "::",
            stringify!(ri_child_pkg_idle_wkups)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<rusage_info_v1>())).ri_child_interrupt_wkups as *const _ as usize
        },
        120usize,
        concat!(
            "Offset of field: ",
            stringify!(rusage_info_v1),
            "::",
            stringify!(ri_child_interrupt_wkups)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<rusage_info_v1>())).ri_child_pageins as *const _ as usize },
        128usize,
        concat!(
            "Offset of field: ",
            stringify!(rusage_info_v1),
            "::",
            stringify!(ri_child_pageins)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<rusage_info_v1>())).ri_child_elapsed_abstime as *const _ as usize
        },
        136usize,
        concat!(
            "Offset of field: ",
            stringify!(rusage_info_v1),
            "::",
            stringify!(ri_child_elapsed_abstime)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct rusage_info_v2 {
    pub ri_uuid: [u8; 16usize],
    pub ri_user_time: u64,
    pub ri_system_time: u64,
    pub ri_pkg_idle_wkups: u64,
    pub ri_interrupt_wkups: u64,
    pub ri_pageins: u64,
    pub ri_wired_size: u64,
    pub ri_resident_size: u64,
    pub ri_phys_footprint: u64,
    pub ri_proc_start_abstime: u64,
    pub ri_proc_exit_abstime: u64,
    pub ri_child_user_time: u64,
    pub ri_child_system_time: u64,
    pub ri_child_pkg_idle_wkups: u64,
    pub ri_child_interrupt_wkups: u64,
    pub ri_child_pageins: u64,
    pub ri_child_elapsed_abstime: u64,
    pub ri_diskio_bytesread: u64,
    pub ri_diskio_byteswritten: u64,
}
#[test]
fn bindgen_test_layout_rusage_info_v2() {
    assert_eq!(
        ::std::mem::size_of::<rusage_info_v2>(),
        160usize,
        concat!("Size of: ", stringify!(rusage_info_v2))
    );
    assert_eq!(
        ::std::mem::align_of::<rusage_info_v2>(),
        8usize,
        concat!("Alignment of ", stringify!(rusage_info_v2))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<rusage_info_v2>())).ri_uuid as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(rusage_info_v2),
            "::",
            stringify!(ri_uuid)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<rusage_info_v2>())).ri_user_time as *const _ as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(rusage_info_v2),
            "::",
            stringify!(ri_user_time)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<rusage_info_v2>())).ri_system_time as *const _ as usize },
        24usize,
        concat!(
            "Offset of field: ",
            stringify!(rusage_info_v2),
            "::",
            stringify!(ri_system_time)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<rusage_info_v2>())).ri_pkg_idle_wkups as *const _ as usize
        },
        32usize,
        concat!(
            "Offset of field: ",
            stringify!(rusage_info_v2),
            "::",
            stringify!(ri_pkg_idle_wkups)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<rusage_info_v2>())).ri_interrupt_wkups as *const _ as usize
        },
        40usize,
        concat!(
            "Offset of field: ",
            stringify!(rusage_info_v2),
            "::",
            stringify!(ri_interrupt_wkups)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<rusage_info_v2>())).ri_pageins as *const _ as usize },
        48usize,
        concat!(
            "Offset of field: ",
            stringify!(rusage_info_v2),
            "::",
            stringify!(ri_pageins)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<rusage_info_v2>())).ri_wired_size as *const _ as usize },
        56usize,
        concat!(
            "Offset of field: ",
            stringify!(rusage_info_v2),
            "::",
            stringify!(ri_wired_size)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<rusage_info_v2>())).ri_resident_size as *const _ as usize },
        64usize,
        concat!(
            "Offset of field: ",
            stringify!(rusage_info_v2),
            "::",
            stringify!(ri_resident_size)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<rusage_info_v2>())).ri_phys_footprint as *const _ as usize
        },
        72usize,
        concat!(
            "Offset of field: ",
            stringify!(rusage_info_v2),
            "::",
            stringify!(ri_phys_footprint)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<rusage_info_v2>())).ri_proc_start_abstime as *const _ as usize
        },
        80usize,
        concat!(
            "Offset of field: ",
            stringify!(rusage_info_v2),
            "::",
            stringify!(ri_proc_start_abstime)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<rusage_info_v2>())).ri_proc_exit_abstime as *const _ as usize
        },
        88usize,
        concat!(
            "Offset of field: ",
            stringify!(rusage_info_v2),
            "::",
            stringify!(ri_proc_exit_abstime)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<rusage_info_v2>())).ri_child_user_time as *const _ as usize
        },
        96usize,
        concat!(
            "Offset of field: ",
            stringify!(rusage_info_v2),
            "::",
            stringify!(ri_child_user_time)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<rusage_info_v2>())).ri_child_system_time as *const _ as usize
        },
        104usize,
        concat!(
            "Offset of field: ",
            stringify!(rusage_info_v2),
            "::",
            stringify!(ri_child_system_time)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<rusage_info_v2>())).ri_child_pkg_idle_wkups as *const _ as usize
        },
        112usize,
        concat!(
            "Offset of field: ",
            stringify!(rusage_info_v2),
            "::",
            stringify!(ri_child_pkg_idle_wkups)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<rusage_info_v2>())).ri_child_interrupt_wkups as *const _ as usize
        },
        120usize,
        concat!(
            "Offset of field: ",
            stringify!(rusage_info_v2),
            "::",
            stringify!(ri_child_interrupt_wkups)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<rusage_info_v2>())).ri_child_pageins as *const _ as usize },
        128usize,
        concat!(
            "Offset of field: ",
            stringify!(rusage_info_v2),
            "::",
            stringify!(ri_child_pageins)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<rusage_info_v2>())).ri_child_elapsed_abstime as *const _ as usize
        },
        136usize,
        concat!(
            "Offset of field: ",
            stringify!(rusage_info_v2),
            "::",
            stringify!(ri_child_elapsed_abstime)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<rusage_info_v2>())).ri_diskio_bytesread as *const _ as usize
        },
        144usize,
        concat!(
            "Offset of field: ",
            stringify!(rusage_info_v2),
            "::",
            stringify!(ri_diskio_bytesread)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<rusage_info_v2>())).ri_diskio_byteswritten as *const _ as usize
        },
        152usize,
        concat!(
            "Offset of field: ",
            stringify!(rusage_info_v2),
            "::",
            stringify!(ri_diskio_byteswritten)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct rusage_info_v3 {
    pub ri_uuid: [u8; 16usize],
    pub ri_user_time: u64,
    pub ri_system_time: u64,
    pub ri_pkg_idle_wkups: u64,
    pub ri_interrupt_wkups: u64,
    pub ri_pageins: u64,
    pub ri_wired_size: u64,
    pub ri_resident_size: u64,
    pub ri_phys_footprint: u64,
    pub ri_proc_start_abstime: u64,
    pub ri_proc_exit_abstime: u64,
    pub ri_child_user_time: u64,
    pub ri_child_system_time: u64,
    pub ri_child_pkg_idle_wkups: u64,
    pub ri_child_interrupt_wkups: u64,
    pub ri_child_pageins: u64,
    pub ri_child_elapsed_abstime: u64,
    pub ri_diskio_bytesread: u64,
    pub ri_diskio_byteswritten: u64,
    pub ri_cpu_time_qos_default: u64,
    pub ri_cpu_time_qos_maintenance: u64,
    pub ri_cpu_time_qos_background: u64,
    pub ri_cpu_time_qos_utility: u64,
    pub ri_cpu_time_qos_legacy: u64,
    pub ri_cpu_time_qos_user_initiated: u64,
    pub ri_cpu_time_qos_user_interactive: u64,
    pub ri_billed_system_time: u64,
    pub ri_serviced_system_time: u64,
}
#[test]
fn bindgen_test_layout_rusage_info_v3() {
    assert_eq!(
        ::std::mem::size_of::<rusage_info_v3>(),
        232usize,
        concat!("Size of: ", stringify!(rusage_info_v3))
    );
    assert_eq!(
        ::std::mem::align_of::<rusage_info_v3>(),
        8usize,
        concat!("Alignment of ", stringify!(rusage_info_v3))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<rusage_info_v3>())).ri_uuid as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(rusage_info_v3),
            "::",
            stringify!(ri_uuid)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<rusage_info_v3>())).ri_user_time as *const _ as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(rusage_info_v3),
            "::",
            stringify!(ri_user_time)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<rusage_info_v3>())).ri_system_time as *const _ as usize },
        24usize,
        concat!(
            "Offset of field: ",
            stringify!(rusage_info_v3),
            "::",
            stringify!(ri_system_time)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<rusage_info_v3>())).ri_pkg_idle_wkups as *const _ as usize
        },
        32usize,
        concat!(
            "Offset of field: ",
            stringify!(rusage_info_v3),
            "::",
            stringify!(ri_pkg_idle_wkups)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<rusage_info_v3>())).ri_interrupt_wkups as *const _ as usize
        },
        40usize,
        concat!(
            "Offset of field: ",
            stringify!(rusage_info_v3),
            "::",
            stringify!(ri_interrupt_wkups)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<rusage_info_v3>())).ri_pageins as *const _ as usize },
        48usize,
        concat!(
            "Offset of field: ",
            stringify!(rusage_info_v3),
            "::",
            stringify!(ri_pageins)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<rusage_info_v3>())).ri_wired_size as *const _ as usize },
        56usize,
        concat!(
            "Offset of field: ",
            stringify!(rusage_info_v3),
            "::",
            stringify!(ri_wired_size)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<rusage_info_v3>())).ri_resident_size as *const _ as usize },
        64usize,
        concat!(
            "Offset of field: ",
            stringify!(rusage_info_v3),
            "::",
            stringify!(ri_resident_size)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<rusage_info_v3>())).ri_phys_footprint as *const _ as usize
        },
        72usize,
        concat!(
            "Offset of field: ",
            stringify!(rusage_info_v3),
            "::",
            stringify!(ri_phys_footprint)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<rusage_info_v3>())).ri_proc_start_abstime as *const _ as usize
        },
        80usize,
        concat!(
            "Offset of field: ",
            stringify!(rusage_info_v3),
            "::",
            stringify!(ri_proc_start_abstime)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<rusage_info_v3>())).ri_proc_exit_abstime as *const _ as usize
        },
        88usize,
        concat!(
            "Offset of field: ",
            stringify!(rusage_info_v3),
            "::",
            stringify!(ri_proc_exit_abstime)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<rusage_info_v3>())).ri_child_user_time as *const _ as usize
        },
        96usize,
        concat!(
            "Offset of field: ",
            stringify!(rusage_info_v3),
            "::",
            stringify!(ri_child_user_time)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<rusage_info_v3>())).ri_child_system_time as *const _ as usize
        },
        104usize,
        concat!(
            "Offset of field: ",
            stringify!(rusage_info_v3),
            "::",
            stringify!(ri_child_system_time)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<rusage_info_v3>())).ri_child_pkg_idle_wkups as *const _ as usize
        },
        112usize,
        concat!(
            "Offset of field: ",
            stringify!(rusage_info_v3),
            "::",
            stringify!(ri_child_pkg_idle_wkups)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<rusage_info_v3>())).ri_child_interrupt_wkups as *const _ as usize
        },
        120usize,
        concat!(
            "Offset of field: ",
            stringify!(rusage_info_v3),
            "::",
            stringify!(ri_child_interrupt_wkups)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<rusage_info_v3>())).ri_child_pageins as *const _ as usize },
        128usize,
        concat!(
            "Offset of field: ",
            stringify!(rusage_info_v3),
            "::",
            stringify!(ri_child_pageins)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<rusage_info_v3>())).ri_child_elapsed_abstime as *const _ as usize
        },
        136usize,
        concat!(
            "Offset of field: ",
            stringify!(rusage_info_v3),
            "::",
            stringify!(ri_child_elapsed_abstime)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<rusage_info_v3>())).ri_diskio_bytesread as *const _ as usize
        },
        144usize,
        concat!(
            "Offset of field: ",
            stringify!(rusage_info_v3),
            "::",
            stringify!(ri_diskio_bytesread)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<rusage_info_v3>())).ri_diskio_byteswritten as *const _ as usize
        },
        152usize,
        concat!(
            "Offset of field: ",
            stringify!(rusage_info_v3),
            "::",
            stringify!(ri_diskio_byteswritten)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<rusage_info_v3>())).ri_cpu_time_qos_default as *const _ as usize
        },
        160usize,
        concat!(
            "Offset of field: ",
            stringify!(rusage_info_v3),
            "::",
            stringify!(ri_cpu_time_qos_default)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<rusage_info_v3>())).ri_cpu_time_qos_maintenance as *const _
                as usize
        },
        168usize,
        concat!(
            "Offset of field: ",
            stringify!(rusage_info_v3),
            "::",
            stringify!(ri_cpu_time_qos_maintenance)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<rusage_info_v3>())).ri_cpu_time_qos_background as *const _
                as usize
        },
        176usize,
        concat!(
            "Offset of field: ",
            stringify!(rusage_info_v3),
            "::",
            stringify!(ri_cpu_time_qos_background)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<rusage_info_v3>())).ri_cpu_time_qos_utility as *const _ as usize
        },
        184usize,
        concat!(
            "Offset of field: ",
            stringify!(rusage_info_v3),
            "::",
            stringify!(ri_cpu_time_qos_utility)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<rusage_info_v3>())).ri_cpu_time_qos_legacy as *const _ as usize
        },
        192usize,
        concat!(
            "Offset of field: ",
            stringify!(rusage_info_v3),
            "::",
            stringify!(ri_cpu_time_qos_legacy)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<rusage_info_v3>())).ri_cpu_time_qos_user_initiated as *const _
                as usize
        },
        200usize,
        concat!(
            "Offset of field: ",
            stringify!(rusage_info_v3),
            "::",
            stringify!(ri_cpu_time_qos_user_initiated)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<rusage_info_v3>())).ri_cpu_time_qos_user_interactive as *const _
                as usize
        },
        208usize,
        concat!(
            "Offset of field: ",
            stringify!(rusage_info_v3),
            "::",
            stringify!(ri_cpu_time_qos_user_interactive)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<rusage_info_v3>())).ri_billed_system_time as *const _ as usize
        },
        216usize,
        concat!(
            "Offset of field: ",
            stringify!(rusage_info_v3),
            "::",
            stringify!(ri_billed_system_time)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<rusage_info_v3>())).ri_serviced_system_time as *const _ as usize
        },
        224usize,
        concat!(
            "Offset of field: ",
            stringify!(rusage_info_v3),
            "::",
            stringify!(ri_serviced_system_time)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct rusage_info_v4 {
    pub ri_uuid: [u8; 16usize],
    pub ri_user_time: u64,
    pub ri_system_time: u64,
    pub ri_pkg_idle_wkups: u64,
    pub ri_interrupt_wkups: u64,
    pub ri_pageins: u64,
    pub ri_wired_size: u64,
    pub ri_resident_size: u64,
    pub ri_phys_footprint: u64,
    pub ri_proc_start_abstime: u64,
    pub ri_proc_exit_abstime: u64,
    pub ri_child_user_time: u64,
    pub ri_child_system_time: u64,
    pub ri_child_pkg_idle_wkups: u64,
    pub ri_child_interrupt_wkups: u64,
    pub ri_child_pageins: u64,
    pub ri_child_elapsed_abstime: u64,
    pub ri_diskio_bytesread: u64,
    pub ri_diskio_byteswritten: u64,
    pub ri_cpu_time_qos_default: u64,
    pub ri_cpu_time_qos_maintenance: u64,
    pub ri_cpu_time_qos_background: u64,
    pub ri_cpu_time_qos_utility: u64,
    pub ri_cpu_time_qos_legacy: u64,
    pub ri_cpu_time_qos_user_initiated: u64,
    pub ri_cpu_time_qos_user_interactive: u64,
    pub ri_billed_system_time: u64,
    pub ri_serviced_system_time: u64,
    pub ri_logical_writes: u64,
    pub ri_lifetime_max_phys_footprint: u64,
    pub ri_instructions: u64,
    pub ri_cycles: u64,
    pub ri_billed_energy: u64,
    pub ri_serviced_energy: u64,
    pub ri_unused: [u64; 2usize],
}
#[test]
fn bindgen_test_layout_rusage_info_v4() {
    assert_eq!(
        ::std::mem::size_of::<rusage_info_v4>(),
        296usize,
        concat!("Size of: ", stringify!(rusage_info_v4))
    );
    assert_eq!(
        ::std::mem::align_of::<rusage_info_v4>(),
        8usize,
        concat!("Alignment of ", stringify!(rusage_info_v4))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<rusage_info_v4>())).ri_uuid as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(rusage_info_v4),
            "::",
            stringify!(ri_uuid)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<rusage_info_v4>())).ri_user_time as *const _ as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(rusage_info_v4),
            "::",
            stringify!(ri_user_time)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<rusage_info_v4>())).ri_system_time as *const _ as usize },
        24usize,
        concat!(
            "Offset of field: ",
            stringify!(rusage_info_v4),
            "::",
            stringify!(ri_system_time)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<rusage_info_v4>())).ri_pkg_idle_wkups as *const _ as usize
        },
        32usize,
        concat!(
            "Offset of field: ",
            stringify!(rusage_info_v4),
            "::",
            stringify!(ri_pkg_idle_wkups)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<rusage_info_v4>())).ri_interrupt_wkups as *const _ as usize
        },
        40usize,
        concat!(
            "Offset of field: ",
            stringify!(rusage_info_v4),
            "::",
            stringify!(ri_interrupt_wkups)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<rusage_info_v4>())).ri_pageins as *const _ as usize },
        48usize,
        concat!(
            "Offset of field: ",
            stringify!(rusage_info_v4),
            "::",
            stringify!(ri_pageins)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<rusage_info_v4>())).ri_wired_size as *const _ as usize },
        56usize,
        concat!(
            "Offset of field: ",
            stringify!(rusage_info_v4),
            "::",
            stringify!(ri_wired_size)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<rusage_info_v4>())).ri_resident_size as *const _ as usize },
        64usize,
        concat!(
            "Offset of field: ",
            stringify!(rusage_info_v4),
            "::",
            stringify!(ri_resident_size)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<rusage_info_v4>())).ri_phys_footprint as *const _ as usize
        },
        72usize,
        concat!(
            "Offset of field: ",
            stringify!(rusage_info_v4),
            "::",
            stringify!(ri_phys_footprint)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<rusage_info_v4>())).ri_proc_start_abstime as *const _ as usize
        },
        80usize,
        concat!(
            "Offset of field: ",
            stringify!(rusage_info_v4),
            "::",
            stringify!(ri_proc_start_abstime)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<rusage_info_v4>())).ri_proc_exit_abstime as *const _ as usize
        },
        88usize,
        concat!(
            "Offset of field: ",
            stringify!(rusage_info_v4),
            "::",
            stringify!(ri_proc_exit_abstime)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<rusage_info_v4>())).ri_child_user_time as *const _ as usize
        },
        96usize,
        concat!(
            "Offset of field: ",
            stringify!(rusage_info_v4),
            "::",
            stringify!(ri_child_user_time)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<rusage_info_v4>())).ri_child_system_time as *const _ as usize
        },
        104usize,
        concat!(
            "Offset of field: ",
            stringify!(rusage_info_v4),
            "::",
            stringify!(ri_child_system_time)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<rusage_info_v4>())).ri_child_pkg_idle_wkups as *const _ as usize
        },
        112usize,
        concat!(
            "Offset of field: ",
            stringify!(rusage_info_v4),
            "::",
            stringify!(ri_child_pkg_idle_wkups)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<rusage_info_v4>())).ri_child_interrupt_wkups as *const _ as usize
        },
        120usize,
        concat!(
            "Offset of field: ",
            stringify!(rusage_info_v4),
            "::",
            stringify!(ri_child_interrupt_wkups)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<rusage_info_v4>())).ri_child_pageins as *const _ as usize },
        128usize,
        concat!(
            "Offset of field: ",
            stringify!(rusage_info_v4),
            "::",
            stringify!(ri_child_pageins)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<rusage_info_v4>())).ri_child_elapsed_abstime as *const _ as usize
        },
        136usize,
        concat!(
            "Offset of field: ",
            stringify!(rusage_info_v4),
            "::",
            stringify!(ri_child_elapsed_abstime)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<rusage_info_v4>())).ri_diskio_bytesread as *const _ as usize
        },
        144usize,
        concat!(
            "Offset of field: ",
            stringify!(rusage_info_v4),
            "::",
            stringify!(ri_diskio_bytesread)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<rusage_info_v4>())).ri_diskio_byteswritten as *const _ as usize
        },
        152usize,
        concat!(
            "Offset of field: ",
            stringify!(rusage_info_v4),
            "::",
            stringify!(ri_diskio_byteswritten)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<rusage_info_v4>())).ri_cpu_time_qos_default as *const _ as usize
        },
        160usize,
        concat!(
            "Offset of field: ",
            stringify!(rusage_info_v4),
            "::",
            stringify!(ri_cpu_time_qos_default)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<rusage_info_v4>())).ri_cpu_time_qos_maintenance as *const _
                as usize
        },
        168usize,
        concat!(
            "Offset of field: ",
            stringify!(rusage_info_v4),
            "::",
            stringify!(ri_cpu_time_qos_maintenance)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<rusage_info_v4>())).ri_cpu_time_qos_background as *const _
                as usize
        },
        176usize,
        concat!(
            "Offset of field: ",
            stringify!(rusage_info_v4),
            "::",
            stringify!(ri_cpu_time_qos_background)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<rusage_info_v4>())).ri_cpu_time_qos_utility as *const _ as usize
        },
        184usize,
        concat!(
            "Offset of field: ",
            stringify!(rusage_info_v4),
            "::",
            stringify!(ri_cpu_time_qos_utility)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<rusage_info_v4>())).ri_cpu_time_qos_legacy as *const _ as usize
        },
        192usize,
        concat!(
            "Offset of field: ",
            stringify!(rusage_info_v4),
            "::",
            stringify!(ri_cpu_time_qos_legacy)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<rusage_info_v4>())).ri_cpu_time_qos_user_initiated as *const _
                as usize
        },
        200usize,
        concat!(
            "Offset of field: ",
            stringify!(rusage_info_v4),
            "::",
            stringify!(ri_cpu_time_qos_user_initiated)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<rusage_info_v4>())).ri_cpu_time_qos_user_interactive as *const _
                as usize
        },
        208usize,
        concat!(
            "Offset of field: ",
            stringify!(rusage_info_v4),
            "::",
            stringify!(ri_cpu_time_qos_user_interactive)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<rusage_info_v4>())).ri_billed_system_time as *const _ as usize
        },
        216usize,
        concat!(
            "Offset of field: ",
            stringify!(rusage_info_v4),
            "::",
            stringify!(ri_billed_system_time)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<rusage_info_v4>())).ri_serviced_system_time as *const _ as usize
        },
        224usize,
        concat!(
            "Offset of field: ",
            stringify!(rusage_info_v4),
            "::",
            stringify!(ri_serviced_system_time)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<rusage_info_v4>())).ri_logical_writes as *const _ as usize
        },
        232usize,
        concat!(
            "Offset of field: ",
            stringify!(rusage_info_v4),
            "::",
            stringify!(ri_logical_writes)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<rusage_info_v4>())).ri_lifetime_max_phys_footprint as *const _
                as usize
        },
        240usize,
        concat!(
            "Offset of field: ",
            stringify!(rusage_info_v4),
            "::",
            stringify!(ri_lifetime_max_phys_footprint)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<rusage_info_v4>())).ri_instructions as *const _ as usize },
        248usize,
        concat!(
            "Offset of field: ",
            stringify!(rusage_info_v4),
            "::",
            stringify!(ri_instructions)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<rusage_info_v4>())).ri_cycles as *const _ as usize },
        256usize,
        concat!(
            "Offset of field: ",
            stringify!(rusage_info_v4),
            "::",
            stringify!(ri_cycles)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<rusage_info_v4>())).ri_billed_energy as *const _ as usize },
        264usize,
        concat!(
            "Offset of field: ",
            stringify!(rusage_info_v4),
            "::",
            stringify!(ri_billed_energy)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<rusage_info_v4>())).ri_serviced_energy as *const _ as usize
        },
        272usize,
        concat!(
            "Offset of field: ",
            stringify!(rusage_info_v4),
            "::",
            stringify!(ri_serviced_energy)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<rusage_info_v4>())).ri_unused as *const _ as usize },
        280usize,
        concat!(
            "Offset of field: ",
            stringify!(rusage_info_v4),
            "::",
            stringify!(ri_unused)
        )
    );
}
pub type rusage_info_current = rusage_info_v4;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct rlimit {
    pub rlim_cur: rlim_t,
    pub rlim_max: rlim_t,
}
#[test]
fn bindgen_test_layout_rlimit() {
    assert_eq!(
        ::std::mem::size_of::<rlimit>(),
        16usize,
        concat!("Size of: ", stringify!(rlimit))
    );
    assert_eq!(
        ::std::mem::align_of::<rlimit>(),
        8usize,
        concat!("Alignment of ", stringify!(rlimit))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<rlimit>())).rlim_cur as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(rlimit),
            "::",
            stringify!(rlim_cur)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<rlimit>())).rlim_max as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(rlimit),
            "::",
            stringify!(rlim_max)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct proc_rlimit_control_wakeupmon {
    pub wm_flags: u32,
    pub wm_rate: i32,
}
#[test]
fn bindgen_test_layout_proc_rlimit_control_wakeupmon() {
    assert_eq!(
        ::std::mem::size_of::<proc_rlimit_control_wakeupmon>(),
        8usize,
        concat!("Size of: ", stringify!(proc_rlimit_control_wakeupmon))
    );
    assert_eq!(
        ::std::mem::align_of::<proc_rlimit_control_wakeupmon>(),
        4usize,
        concat!("Alignment of ", stringify!(proc_rlimit_control_wakeupmon))
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<proc_rlimit_control_wakeupmon>())).wm_flags as *const _ as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(proc_rlimit_control_wakeupmon),
            "::",
            stringify!(wm_flags)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<proc_rlimit_control_wakeupmon>())).wm_rate as *const _ as usize
        },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(proc_rlimit_control_wakeupmon),
            "::",
            stringify!(wm_rate)
        )
    );
}
extern "C" {
    pub fn getpriority(arg1: ::std::os::raw::c_int, arg2: id_t) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn getiopolicy_np(
        arg1: ::std::os::raw::c_int,
        arg2: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn getrlimit(arg1: ::std::os::raw::c_int, arg2: *mut rlimit) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn getrusage(arg1: ::std::os::raw::c_int, arg2: *mut rusage) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn setpriority(
        arg1: ::std::os::raw::c_int,
        arg2: id_t,
        arg3: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn setiopolicy_np(
        arg1: ::std::os::raw::c_int,
        arg2: ::std::os::raw::c_int,
        arg3: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn setrlimit(arg1: ::std::os::raw::c_int, arg2: *const rlimit) -> ::std::os::raw::c_int;
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union wait {
    pub w_status: ::std::os::raw::c_int,
    pub w_T: wait__bindgen_ty_1,
    pub w_S: wait__bindgen_ty_2,
    _bindgen_union_align: u32,
}
#[repr(C)]
#[repr(align(4))]
#[derive(Debug, Copy, Clone)]
pub struct wait__bindgen_ty_1 {
    pub _bitfield_1: __BindgenBitfieldUnit<[u8; 4usize], u16>,
}
#[test]
fn bindgen_test_layout_wait__bindgen_ty_1() {
    assert_eq!(
        ::std::mem::size_of::<wait__bindgen_ty_1>(),
        4usize,
        concat!("Size of: ", stringify!(wait__bindgen_ty_1))
    );
    assert_eq!(
        ::std::mem::align_of::<wait__bindgen_ty_1>(),
        4usize,
        concat!("Alignment of ", stringify!(wait__bindgen_ty_1))
    );
}
impl wait__bindgen_ty_1 {
    #[inline]
    pub fn w_Termsig(&self) -> ::std::os::raw::c_uint {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(0usize, 7u8) as u32) }
    }
    #[inline]
    pub fn set_w_Termsig(&mut self, val: ::std::os::raw::c_uint) {
        unsafe {
            let val: u32 = ::std::mem::transmute(val);
            self._bitfield_1.set(0usize, 7u8, val as u64)
        }
    }
    #[inline]
    pub fn w_Coredump(&self) -> ::std::os::raw::c_uint {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(7usize, 1u8) as u32) }
    }
    #[inline]
    pub fn set_w_Coredump(&mut self, val: ::std::os::raw::c_uint) {
        unsafe {
            let val: u32 = ::std::mem::transmute(val);
            self._bitfield_1.set(7usize, 1u8, val as u64)
        }
    }
    #[inline]
    pub fn w_Retcode(&self) -> ::std::os::raw::c_uint {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(8usize, 8u8) as u32) }
    }
    #[inline]
    pub fn set_w_Retcode(&mut self, val: ::std::os::raw::c_uint) {
        unsafe {
            let val: u32 = ::std::mem::transmute(val);
            self._bitfield_1.set(8usize, 8u8, val as u64)
        }
    }
    #[inline]
    pub fn w_Filler(&self) -> ::std::os::raw::c_uint {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(16usize, 16u8) as u32) }
    }
    #[inline]
    pub fn set_w_Filler(&mut self, val: ::std::os::raw::c_uint) {
        unsafe {
            let val: u32 = ::std::mem::transmute(val);
            self._bitfield_1.set(16usize, 16u8, val as u64)
        }
    }
    #[inline]
    pub fn new_bitfield_1(
        w_Termsig: ::std::os::raw::c_uint,
        w_Coredump: ::std::os::raw::c_uint,
        w_Retcode: ::std::os::raw::c_uint,
        w_Filler: ::std::os::raw::c_uint,
    ) -> __BindgenBitfieldUnit<[u8; 4usize], u16> {
        let mut __bindgen_bitfield_unit: __BindgenBitfieldUnit<[u8; 4usize], u16> =
            Default::default();
        __bindgen_bitfield_unit.set(0usize, 7u8, {
            let w_Termsig: u32 = unsafe { ::std::mem::transmute(w_Termsig) };
            w_Termsig as u64
        });
        __bindgen_bitfield_unit.set(7usize, 1u8, {
            let w_Coredump: u32 = unsafe { ::std::mem::transmute(w_Coredump) };
            w_Coredump as u64
        });
        __bindgen_bitfield_unit.set(8usize, 8u8, {
            let w_Retcode: u32 = unsafe { ::std::mem::transmute(w_Retcode) };
            w_Retcode as u64
        });
        __bindgen_bitfield_unit.set(16usize, 16u8, {
            let w_Filler: u32 = unsafe { ::std::mem::transmute(w_Filler) };
            w_Filler as u64
        });
        __bindgen_bitfield_unit
    }
}
#[repr(C)]
#[repr(align(4))]
#[derive(Debug, Copy, Clone)]
pub struct wait__bindgen_ty_2 {
    pub _bitfield_1: __BindgenBitfieldUnit<[u8; 4usize], u16>,
}
#[test]
fn bindgen_test_layout_wait__bindgen_ty_2() {
    assert_eq!(
        ::std::mem::size_of::<wait__bindgen_ty_2>(),
        4usize,
        concat!("Size of: ", stringify!(wait__bindgen_ty_2))
    );
    assert_eq!(
        ::std::mem::align_of::<wait__bindgen_ty_2>(),
        4usize,
        concat!("Alignment of ", stringify!(wait__bindgen_ty_2))
    );
}
impl wait__bindgen_ty_2 {
    #[inline]
    pub fn w_Stopval(&self) -> ::std::os::raw::c_uint {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(0usize, 8u8) as u32) }
    }
    #[inline]
    pub fn set_w_Stopval(&mut self, val: ::std::os::raw::c_uint) {
        unsafe {
            let val: u32 = ::std::mem::transmute(val);
            self._bitfield_1.set(0usize, 8u8, val as u64)
        }
    }
    #[inline]
    pub fn w_Stopsig(&self) -> ::std::os::raw::c_uint {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(8usize, 8u8) as u32) }
    }
    #[inline]
    pub fn set_w_Stopsig(&mut self, val: ::std::os::raw::c_uint) {
        unsafe {
            let val: u32 = ::std::mem::transmute(val);
            self._bitfield_1.set(8usize, 8u8, val as u64)
        }
    }
    #[inline]
    pub fn w_Filler(&self) -> ::std::os::raw::c_uint {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(16usize, 16u8) as u32) }
    }
    #[inline]
    pub fn set_w_Filler(&mut self, val: ::std::os::raw::c_uint) {
        unsafe {
            let val: u32 = ::std::mem::transmute(val);
            self._bitfield_1.set(16usize, 16u8, val as u64)
        }
    }
    #[inline]
    pub fn new_bitfield_1(
        w_Stopval: ::std::os::raw::c_uint,
        w_Stopsig: ::std::os::raw::c_uint,
        w_Filler: ::std::os::raw::c_uint,
    ) -> __BindgenBitfieldUnit<[u8; 4usize], u16> {
        let mut __bindgen_bitfield_unit: __BindgenBitfieldUnit<[u8; 4usize], u16> =
            Default::default();
        __bindgen_bitfield_unit.set(0usize, 8u8, {
            let w_Stopval: u32 = unsafe { ::std::mem::transmute(w_Stopval) };
            w_Stopval as u64
        });
        __bindgen_bitfield_unit.set(8usize, 8u8, {
            let w_Stopsig: u32 = unsafe { ::std::mem::transmute(w_Stopsig) };
            w_Stopsig as u64
        });
        __bindgen_bitfield_unit.set(16usize, 16u8, {
            let w_Filler: u32 = unsafe { ::std::mem::transmute(w_Filler) };
            w_Filler as u64
        });
        __bindgen_bitfield_unit
    }
}
#[test]
fn bindgen_test_layout_wait() {
    assert_eq!(
        ::std::mem::size_of::<wait>(),
        4usize,
        concat!("Size of: ", stringify!(wait))
    );
    assert_eq!(
        ::std::mem::align_of::<wait>(),
        4usize,
        concat!("Alignment of ", stringify!(wait))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<wait>())).w_status as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(wait),
            "::",
            stringify!(w_status)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<wait>())).w_T as *const _ as usize },
        0usize,
        concat!("Offset of field: ", stringify!(wait), "::", stringify!(w_T))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<wait>())).w_S as *const _ as usize },
        0usize,
        concat!("Offset of field: ", stringify!(wait), "::", stringify!(w_S))
    );
}
extern "C" {
    pub fn wait(arg1: *mut ::std::os::raw::c_int) -> pid_t;
}
extern "C" {
    pub fn waitpid(
        arg1: pid_t,
        arg2: *mut ::std::os::raw::c_int,
        arg3: ::std::os::raw::c_int,
    ) -> pid_t;
}
extern "C" {
    pub fn waitid(
        arg1: idtype_t,
        arg2: id_t,
        arg3: *mut siginfo_t,
        arg4: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn wait3(
        arg1: *mut ::std::os::raw::c_int,
        arg2: ::std::os::raw::c_int,
        arg3: *mut rusage,
    ) -> pid_t;
}
extern "C" {
    pub fn wait4(
        arg1: pid_t,
        arg2: *mut ::std::os::raw::c_int,
        arg3: ::std::os::raw::c_int,
        arg4: *mut rusage,
    ) -> pid_t;
}
extern "C" {
    pub fn alloca(arg1: ::std::os::raw::c_ulong) -> *mut ::std::os::raw::c_void;
}
pub type ct_rune_t = __darwin_ct_rune_t;
pub type rune_t = __darwin_rune_t;
pub type wchar_t = __darwin_wchar_t;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct div_t {
    pub quot: ::std::os::raw::c_int,
    pub rem: ::std::os::raw::c_int,
}
#[test]
fn bindgen_test_layout_div_t() {
    assert_eq!(
        ::std::mem::size_of::<div_t>(),
        8usize,
        concat!("Size of: ", stringify!(div_t))
    );
    assert_eq!(
        ::std::mem::align_of::<div_t>(),
        4usize,
        concat!("Alignment of ", stringify!(div_t))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<div_t>())).quot as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(div_t),
            "::",
            stringify!(quot)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<div_t>())).rem as *const _ as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(div_t),
            "::",
            stringify!(rem)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ldiv_t {
    pub quot: ::std::os::raw::c_long,
    pub rem: ::std::os::raw::c_long,
}
#[test]
fn bindgen_test_layout_ldiv_t() {
    assert_eq!(
        ::std::mem::size_of::<ldiv_t>(),
        16usize,
        concat!("Size of: ", stringify!(ldiv_t))
    );
    assert_eq!(
        ::std::mem::align_of::<ldiv_t>(),
        8usize,
        concat!("Alignment of ", stringify!(ldiv_t))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<ldiv_t>())).quot as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(ldiv_t),
            "::",
            stringify!(quot)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<ldiv_t>())).rem as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(ldiv_t),
            "::",
            stringify!(rem)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct lldiv_t {
    pub quot: ::std::os::raw::c_longlong,
    pub rem: ::std::os::raw::c_longlong,
}
#[test]
fn bindgen_test_layout_lldiv_t() {
    assert_eq!(
        ::std::mem::size_of::<lldiv_t>(),
        16usize,
        concat!("Size of: ", stringify!(lldiv_t))
    );
    assert_eq!(
        ::std::mem::align_of::<lldiv_t>(),
        8usize,
        concat!("Alignment of ", stringify!(lldiv_t))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<lldiv_t>())).quot as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(lldiv_t),
            "::",
            stringify!(quot)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<lldiv_t>())).rem as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(lldiv_t),
            "::",
            stringify!(rem)
        )
    );
}
extern "C" {
    pub static mut __mb_cur_max: ::std::os::raw::c_int;
}
extern "C" {
    pub fn abort();
}
extern "C" {
    pub fn abs(arg1: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn atexit(arg1: ::std::option::Option<unsafe extern "C" fn()>) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn atof(arg1: *const ::std::os::raw::c_char) -> f64;
}
extern "C" {
    pub fn atoi(arg1: *const ::std::os::raw::c_char) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn atol(arg1: *const ::std::os::raw::c_char) -> ::std::os::raw::c_long;
}
extern "C" {
    pub fn atoll(arg1: *const ::std::os::raw::c_char) -> ::std::os::raw::c_longlong;
}
extern "C" {
    pub fn bsearch(
        __key: *const ::std::os::raw::c_void,
        __base: *const ::std::os::raw::c_void,
        __nel: size_t,
        __width: size_t,
        __compar: ::std::option::Option<
            unsafe extern "C" fn(
                arg1: *const ::std::os::raw::c_void,
                arg2: *const ::std::os::raw::c_void,
            ) -> ::std::os::raw::c_int,
        >,
    ) -> *mut ::std::os::raw::c_void;
}
extern "C" {
    pub fn calloc(
        __count: ::std::os::raw::c_ulong,
        __size: ::std::os::raw::c_ulong,
    ) -> *mut ::std::os::raw::c_void;
}
extern "C" {
    pub fn div(arg1: ::std::os::raw::c_int, arg2: ::std::os::raw::c_int) -> div_t;
}
extern "C" {
    pub fn exit(arg1: ::std::os::raw::c_int);
}
extern "C" {
    pub fn free(arg1: *mut ::std::os::raw::c_void);
}
extern "C" {
    pub fn getenv(arg1: *const ::std::os::raw::c_char) -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn labs(arg1: ::std::os::raw::c_long) -> ::std::os::raw::c_long;
}
extern "C" {
    pub fn ldiv(arg1: ::std::os::raw::c_long, arg2: ::std::os::raw::c_long) -> ldiv_t;
}
extern "C" {
    pub fn llabs(arg1: ::std::os::raw::c_longlong) -> ::std::os::raw::c_longlong;
}
extern "C" {
    pub fn lldiv(arg1: ::std::os::raw::c_longlong, arg2: ::std::os::raw::c_longlong) -> lldiv_t;
}
extern "C" {
    pub fn malloc(__size: ::std::os::raw::c_ulong) -> *mut ::std::os::raw::c_void;
}
extern "C" {
    pub fn mblen(__s: *const ::std::os::raw::c_char, __n: size_t) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn mbstowcs(
        arg1: *mut wchar_t,
        arg2: *const ::std::os::raw::c_char,
        arg3: size_t,
    ) -> size_t;
}
extern "C" {
    pub fn mbtowc(
        arg1: *mut wchar_t,
        arg2: *const ::std::os::raw::c_char,
        arg3: size_t,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn posix_memalign(
        __memptr: *mut *mut ::std::os::raw::c_void,
        __alignment: size_t,
        __size: size_t,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn qsort(
        __base: *mut ::std::os::raw::c_void,
        __nel: size_t,
        __width: size_t,
        __compar: ::std::option::Option<
            unsafe extern "C" fn(
                arg1: *const ::std::os::raw::c_void,
                arg2: *const ::std::os::raw::c_void,
            ) -> ::std::os::raw::c_int,
        >,
    );
}
extern "C" {
    pub fn rand() -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn realloc(
        __ptr: *mut ::std::os::raw::c_void,
        __size: ::std::os::raw::c_ulong,
    ) -> *mut ::std::os::raw::c_void;
}
extern "C" {
    pub fn srand(arg1: ::std::os::raw::c_uint);
}
extern "C" {
    pub fn strtod(
        arg1: *const ::std::os::raw::c_char,
        arg2: *mut *mut ::std::os::raw::c_char,
    ) -> f64;
}
extern "C" {
    pub fn strtof(
        arg1: *const ::std::os::raw::c_char,
        arg2: *mut *mut ::std::os::raw::c_char,
    ) -> f32;
}
extern "C" {
    pub fn strtol(
        __str: *const ::std::os::raw::c_char,
        __endptr: *mut *mut ::std::os::raw::c_char,
        __base: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_long;
}
extern "C" {
    pub fn strtold(
        arg1: *const ::std::os::raw::c_char,
        arg2: *mut *mut ::std::os::raw::c_char,
    ) -> u128;
}
extern "C" {
    pub fn strtoll(
        __str: *const ::std::os::raw::c_char,
        __endptr: *mut *mut ::std::os::raw::c_char,
        __base: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_longlong;
}
extern "C" {
    pub fn strtoul(
        __str: *const ::std::os::raw::c_char,
        __endptr: *mut *mut ::std::os::raw::c_char,
        __base: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_ulong;
}
extern "C" {
    pub fn strtoull(
        __str: *const ::std::os::raw::c_char,
        __endptr: *mut *mut ::std::os::raw::c_char,
        __base: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_ulonglong;
}
extern "C" {
    pub fn system(arg1: *const ::std::os::raw::c_char) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn wcstombs(
        arg1: *mut ::std::os::raw::c_char,
        arg2: *const wchar_t,
        arg3: size_t,
    ) -> size_t;
}
extern "C" {
    pub fn wctomb(arg1: *mut ::std::os::raw::c_char, arg2: wchar_t) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn _Exit(arg1: ::std::os::raw::c_int);
}
extern "C" {
    pub fn a64l(arg1: *const ::std::os::raw::c_char) -> ::std::os::raw::c_long;
}
extern "C" {
    pub fn drand48() -> f64;
}
extern "C" {
    pub fn ecvt(
        arg1: f64,
        arg2: ::std::os::raw::c_int,
        arg3: *mut ::std::os::raw::c_int,
        arg4: *mut ::std::os::raw::c_int,
    ) -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn erand48(arg1: *mut ::std::os::raw::c_ushort) -> f64;
}
extern "C" {
    pub fn fcvt(
        arg1: f64,
        arg2: ::std::os::raw::c_int,
        arg3: *mut ::std::os::raw::c_int,
        arg4: *mut ::std::os::raw::c_int,
    ) -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn gcvt(
        arg1: f64,
        arg2: ::std::os::raw::c_int,
        arg3: *mut ::std::os::raw::c_char,
    ) -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn getsubopt(
        arg1: *mut *mut ::std::os::raw::c_char,
        arg2: *const *mut ::std::os::raw::c_char,
        arg3: *mut *mut ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn grantpt(arg1: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn initstate(
        arg1: ::std::os::raw::c_uint,
        arg2: *mut ::std::os::raw::c_char,
        arg3: size_t,
    ) -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn jrand48(arg1: *mut ::std::os::raw::c_ushort) -> ::std::os::raw::c_long;
}
extern "C" {
    pub fn l64a(arg1: ::std::os::raw::c_long) -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn lcong48(arg1: *mut ::std::os::raw::c_ushort);
}
extern "C" {
    pub fn lrand48() -> ::std::os::raw::c_long;
}
extern "C" {
    pub fn mktemp(arg1: *mut ::std::os::raw::c_char) -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn mkstemp(arg1: *mut ::std::os::raw::c_char) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn mrand48() -> ::std::os::raw::c_long;
}
extern "C" {
    pub fn nrand48(arg1: *mut ::std::os::raw::c_ushort) -> ::std::os::raw::c_long;
}
extern "C" {
    pub fn posix_openpt(arg1: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn ptsname(arg1: ::std::os::raw::c_int) -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn ptsname_r(
        fildes: ::std::os::raw::c_int,
        buffer: *mut ::std::os::raw::c_char,
        buflen: size_t,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn putenv(arg1: *mut ::std::os::raw::c_char) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn random() -> ::std::os::raw::c_long;
}
extern "C" {
    pub fn rand_r(arg1: *mut ::std::os::raw::c_uint) -> ::std::os::raw::c_int;
}
extern "C" {
    #[link_name = "\u{1}_realpath$DARWIN_EXTSN"]
    pub fn realpath(
        arg1: *const ::std::os::raw::c_char,
        arg2: *mut ::std::os::raw::c_char,
    ) -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn seed48(arg1: *mut ::std::os::raw::c_ushort) -> *mut ::std::os::raw::c_ushort;
}
extern "C" {
    pub fn setenv(
        __name: *const ::std::os::raw::c_char,
        __value: *const ::std::os::raw::c_char,
        __overwrite: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn setkey(arg1: *const ::std::os::raw::c_char);
}
extern "C" {
    pub fn setstate(arg1: *const ::std::os::raw::c_char) -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn srand48(arg1: ::std::os::raw::c_long);
}
extern "C" {
    pub fn srandom(arg1: ::std::os::raw::c_uint);
}
extern "C" {
    pub fn unlockpt(arg1: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn unsetenv(arg1: *const ::std::os::raw::c_char) -> ::std::os::raw::c_int;
}
pub type dev_t = __darwin_dev_t;
pub type mode_t = __darwin_mode_t;
extern "C" {
    pub fn arc4random() -> u32;
}
extern "C" {
    pub fn arc4random_addrandom(arg1: *mut ::std::os::raw::c_uchar, arg2: ::std::os::raw::c_int);
}
extern "C" {
    pub fn arc4random_buf(__buf: *mut ::std::os::raw::c_void, __nbytes: size_t);
}
extern "C" {
    pub fn arc4random_stir();
}
extern "C" {
    pub fn arc4random_uniform(__upper_bound: u32) -> u32;
}
extern "C" {
    pub fn atexit_b(arg1: *mut ::std::os::raw::c_void) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn bsearch_b(
        __key: *const ::std::os::raw::c_void,
        __base: *const ::std::os::raw::c_void,
        __nel: size_t,
        __width: size_t,
        __compar: *mut ::std::os::raw::c_void,
    ) -> *mut ::std::os::raw::c_void;
}
extern "C" {
    pub fn cgetcap(
        arg1: *mut ::std::os::raw::c_char,
        arg2: *const ::std::os::raw::c_char,
        arg3: ::std::os::raw::c_int,
    ) -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn cgetclose() -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn cgetent(
        arg1: *mut *mut ::std::os::raw::c_char,
        arg2: *mut *mut ::std::os::raw::c_char,
        arg3: *const ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn cgetfirst(
        arg1: *mut *mut ::std::os::raw::c_char,
        arg2: *mut *mut ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn cgetmatch(
        arg1: *const ::std::os::raw::c_char,
        arg2: *const ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn cgetnext(
        arg1: *mut *mut ::std::os::raw::c_char,
        arg2: *mut *mut ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn cgetnum(
        arg1: *mut ::std::os::raw::c_char,
        arg2: *const ::std::os::raw::c_char,
        arg3: *mut ::std::os::raw::c_long,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn cgetset(arg1: *const ::std::os::raw::c_char) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn cgetstr(
        arg1: *mut ::std::os::raw::c_char,
        arg2: *const ::std::os::raw::c_char,
        arg3: *mut *mut ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn cgetustr(
        arg1: *mut ::std::os::raw::c_char,
        arg2: *const ::std::os::raw::c_char,
        arg3: *mut *mut ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    #[link_name = "\u{1}_daemon$1050"]
    pub fn daemon(
        arg1: ::std::os::raw::c_int,
        arg2: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn devname(arg1: dev_t, arg2: mode_t) -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn devname_r(
        arg1: dev_t,
        arg2: mode_t,
        buf: *mut ::std::os::raw::c_char,
        len: ::std::os::raw::c_int,
    ) -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn getbsize(
        arg1: *mut ::std::os::raw::c_int,
        arg2: *mut ::std::os::raw::c_long,
    ) -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn getloadavg(arg1: *mut f64, arg2: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn getprogname() -> *const ::std::os::raw::c_char;
}
extern "C" {
    pub fn heapsort(
        __base: *mut ::std::os::raw::c_void,
        __nel: size_t,
        __width: size_t,
        __compar: ::std::option::Option<
            unsafe extern "C" fn(
                arg1: *const ::std::os::raw::c_void,
                arg2: *const ::std::os::raw::c_void,
            ) -> ::std::os::raw::c_int,
        >,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn heapsort_b(
        __base: *mut ::std::os::raw::c_void,
        __nel: size_t,
        __width: size_t,
        __compar: *mut ::std::os::raw::c_void,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn mergesort(
        __base: *mut ::std::os::raw::c_void,
        __nel: size_t,
        __width: size_t,
        __compar: ::std::option::Option<
            unsafe extern "C" fn(
                arg1: *const ::std::os::raw::c_void,
                arg2: *const ::std::os::raw::c_void,
            ) -> ::std::os::raw::c_int,
        >,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn mergesort_b(
        __base: *mut ::std::os::raw::c_void,
        __nel: size_t,
        __width: size_t,
        __compar: *mut ::std::os::raw::c_void,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn psort(
        __base: *mut ::std::os::raw::c_void,
        __nel: size_t,
        __width: size_t,
        __compar: ::std::option::Option<
            unsafe extern "C" fn(
                arg1: *const ::std::os::raw::c_void,
                arg2: *const ::std::os::raw::c_void,
            ) -> ::std::os::raw::c_int,
        >,
    );
}
extern "C" {
    pub fn psort_b(
        __base: *mut ::std::os::raw::c_void,
        __nel: size_t,
        __width: size_t,
        __compar: *mut ::std::os::raw::c_void,
    );
}
extern "C" {
    pub fn psort_r(
        __base: *mut ::std::os::raw::c_void,
        __nel: size_t,
        __width: size_t,
        arg1: *mut ::std::os::raw::c_void,
        __compar: ::std::option::Option<
            unsafe extern "C" fn(
                arg1: *mut ::std::os::raw::c_void,
                arg2: *const ::std::os::raw::c_void,
                arg3: *const ::std::os::raw::c_void,
            ) -> ::std::os::raw::c_int,
        >,
    );
}
extern "C" {
    pub fn qsort_b(
        __base: *mut ::std::os::raw::c_void,
        __nel: size_t,
        __width: size_t,
        __compar: *mut ::std::os::raw::c_void,
    );
}
extern "C" {
    pub fn qsort_r(
        __base: *mut ::std::os::raw::c_void,
        __nel: size_t,
        __width: size_t,
        arg1: *mut ::std::os::raw::c_void,
        __compar: ::std::option::Option<
            unsafe extern "C" fn(
                arg1: *mut ::std::os::raw::c_void,
                arg2: *const ::std::os::raw::c_void,
                arg3: *const ::std::os::raw::c_void,
            ) -> ::std::os::raw::c_int,
        >,
    );
}
extern "C" {
    pub fn radixsort(
        __base: *mut *const ::std::os::raw::c_uchar,
        __nel: ::std::os::raw::c_int,
        __table: *const ::std::os::raw::c_uchar,
        __endbyte: ::std::os::raw::c_uint,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn setprogname(arg1: *const ::std::os::raw::c_char);
}
extern "C" {
    pub fn sradixsort(
        __base: *mut *const ::std::os::raw::c_uchar,
        __nel: ::std::os::raw::c_int,
        __table: *const ::std::os::raw::c_uchar,
        __endbyte: ::std::os::raw::c_uint,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn sranddev();
}
extern "C" {
    pub fn srandomdev();
}
extern "C" {
    pub fn reallocf(
        __ptr: *mut ::std::os::raw::c_void,
        __size: size_t,
    ) -> *mut ::std::os::raw::c_void;
}
extern "C" {
    pub fn strtoq(
        __str: *const ::std::os::raw::c_char,
        __endptr: *mut *mut ::std::os::raw::c_char,
        __base: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_longlong;
}
extern "C" {
    pub fn strtouq(
        __str: *const ::std::os::raw::c_char,
        __endptr: *mut *mut ::std::os::raw::c_char,
        __base: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_ulonglong;
}
extern "C" {
    pub static mut suboptarg: *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn valloc(arg1: size_t) -> *mut ::std::os::raw::c_void;
}
pub type SCHAR = ::std::os::raw::c_schar;
pub type UCHAR = ::std::os::raw::c_uchar;
pub type SWORD = ::std::os::raw::c_short;
pub type USHORT = ::std::os::raw::c_ushort;
pub type SSHORT = ::std::os::raw::c_short;
pub type UWORD = ::std::os::raw::c_ushort;
pub type SDWORD = sqlint32;
pub type ULONG = sqluint32;
pub type UDWORD = sqluint32;
pub type SLONG = sqlint32;
pub type SDOUBLE = f64;
pub type SFLOAT = f32;
pub type SQLDATE = ::std::os::raw::c_uchar;
pub type SQLTIME = ::std::os::raw::c_uchar;
pub type SQLTIMESTAMP = ::std::os::raw::c_uchar;
pub type SQLDECIMAL = ::std::os::raw::c_uchar;
pub type SQLNUMERIC = ::std::os::raw::c_uchar;
pub type LDOUBLE = f64;
pub type PTR = *mut ::std::os::raw::c_void;
pub type HENV = *mut ::std::os::raw::c_void;
pub type HDBC = *mut ::std::os::raw::c_void;
pub type HSTMT = *mut ::std::os::raw::c_void;
pub type RETCODE = ::std::os::raw::c_short;
pub type SQLCHAR = UCHAR;
pub type SQLVARCHAR = UCHAR;
pub type SQLSCHAR = SCHAR;
pub type SQLINTEGER = SDWORD;
pub type SQLSMALLINT = SWORD;
pub type SQLDOUBLE = SDOUBLE;
pub type SQLFLOAT = SDOUBLE;
pub type SQLREAL = SFLOAT;
pub type SQLRETURN = SQLSMALLINT;
pub type SQLUINTEGER = UDWORD;
pub type SQLUSMALLINT = UWORD;
pub type SQLPOINTER = PTR;
pub type SQLDBCHAR = ::std::os::raw::c_ushort;
pub type SQLWCHAR = ::std::os::raw::c_ushort;
pub type SQLTCHAR = SQLCHAR;
pub type SQLHANDLE = SQLINTEGER;
pub type SQLHENV = SQLINTEGER;
pub type SQLHDBC = SQLINTEGER;
pub type SQLHSTMT = SQLINTEGER;
pub type SQLHWND = SQLPOINTER;
pub type SQLHDESC = SQLHANDLE;
pub type SQLBIGINT = ::std::os::raw::c_long;
pub type SQLUBIGINT = ::std::os::raw::c_ulong;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct DATE_STRUCT {
    pub year: SQLSMALLINT,
    pub month: SQLUSMALLINT,
    pub day: SQLUSMALLINT,
}
#[test]
fn bindgen_test_layout_DATE_STRUCT() {
    assert_eq!(
        ::std::mem::size_of::<DATE_STRUCT>(),
        6usize,
        concat!("Size of: ", stringify!(DATE_STRUCT))
    );
    assert_eq!(
        ::std::mem::align_of::<DATE_STRUCT>(),
        2usize,
        concat!("Alignment of ", stringify!(DATE_STRUCT))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<DATE_STRUCT>())).year as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(DATE_STRUCT),
            "::",
            stringify!(year)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<DATE_STRUCT>())).month as *const _ as usize },
        2usize,
        concat!(
            "Offset of field: ",
            stringify!(DATE_STRUCT),
            "::",
            stringify!(month)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<DATE_STRUCT>())).day as *const _ as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(DATE_STRUCT),
            "::",
            stringify!(day)
        )
    );
}
pub type SQL_DATE_STRUCT = DATE_STRUCT;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct TIME_STRUCT {
    pub hour: SQLUSMALLINT,
    pub minute: SQLUSMALLINT,
    pub second: SQLUSMALLINT,
}
#[test]
fn bindgen_test_layout_TIME_STRUCT() {
    assert_eq!(
        ::std::mem::size_of::<TIME_STRUCT>(),
        6usize,
        concat!("Size of: ", stringify!(TIME_STRUCT))
    );
    assert_eq!(
        ::std::mem::align_of::<TIME_STRUCT>(),
        2usize,
        concat!("Alignment of ", stringify!(TIME_STRUCT))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<TIME_STRUCT>())).hour as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(TIME_STRUCT),
            "::",
            stringify!(hour)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<TIME_STRUCT>())).minute as *const _ as usize },
        2usize,
        concat!(
            "Offset of field: ",
            stringify!(TIME_STRUCT),
            "::",
            stringify!(minute)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<TIME_STRUCT>())).second as *const _ as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(TIME_STRUCT),
            "::",
            stringify!(second)
        )
    );
}
pub type SQL_TIME_STRUCT = TIME_STRUCT;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct TIMESTAMP_STRUCT {
    pub year: SQLSMALLINT,
    pub month: SQLUSMALLINT,
    pub day: SQLUSMALLINT,
    pub hour: SQLUSMALLINT,
    pub minute: SQLUSMALLINT,
    pub second: SQLUSMALLINT,
    pub fraction: SQLUINTEGER,
}
#[test]
fn bindgen_test_layout_TIMESTAMP_STRUCT() {
    assert_eq!(
        ::std::mem::size_of::<TIMESTAMP_STRUCT>(),
        16usize,
        concat!("Size of: ", stringify!(TIMESTAMP_STRUCT))
    );
    assert_eq!(
        ::std::mem::align_of::<TIMESTAMP_STRUCT>(),
        4usize,
        concat!("Alignment of ", stringify!(TIMESTAMP_STRUCT))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<TIMESTAMP_STRUCT>())).year as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(TIMESTAMP_STRUCT),
            "::",
            stringify!(year)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<TIMESTAMP_STRUCT>())).month as *const _ as usize },
        2usize,
        concat!(
            "Offset of field: ",
            stringify!(TIMESTAMP_STRUCT),
            "::",
            stringify!(month)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<TIMESTAMP_STRUCT>())).day as *const _ as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(TIMESTAMP_STRUCT),
            "::",
            stringify!(day)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<TIMESTAMP_STRUCT>())).hour as *const _ as usize },
        6usize,
        concat!(
            "Offset of field: ",
            stringify!(TIMESTAMP_STRUCT),
            "::",
            stringify!(hour)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<TIMESTAMP_STRUCT>())).minute as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(TIMESTAMP_STRUCT),
            "::",
            stringify!(minute)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<TIMESTAMP_STRUCT>())).second as *const _ as usize },
        10usize,
        concat!(
            "Offset of field: ",
            stringify!(TIMESTAMP_STRUCT),
            "::",
            stringify!(second)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<TIMESTAMP_STRUCT>())).fraction as *const _ as usize },
        12usize,
        concat!(
            "Offset of field: ",
            stringify!(TIMESTAMP_STRUCT),
            "::",
            stringify!(fraction)
        )
    );
}
pub type SQL_TIMESTAMP_STRUCT = TIMESTAMP_STRUCT;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct TIMESTAMP_STRUCT_EXT {
    pub year: SQLSMALLINT,
    pub month: SQLUSMALLINT,
    pub day: SQLUSMALLINT,
    pub hour: SQLUSMALLINT,
    pub minute: SQLUSMALLINT,
    pub second: SQLUSMALLINT,
    pub fraction: SQLUINTEGER,
    pub fraction2: SQLUINTEGER,
}
#[test]
fn bindgen_test_layout_TIMESTAMP_STRUCT_EXT() {
    assert_eq!(
        ::std::mem::size_of::<TIMESTAMP_STRUCT_EXT>(),
        20usize,
        concat!("Size of: ", stringify!(TIMESTAMP_STRUCT_EXT))
    );
    assert_eq!(
        ::std::mem::align_of::<TIMESTAMP_STRUCT_EXT>(),
        4usize,
        concat!("Alignment of ", stringify!(TIMESTAMP_STRUCT_EXT))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<TIMESTAMP_STRUCT_EXT>())).year as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(TIMESTAMP_STRUCT_EXT),
            "::",
            stringify!(year)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<TIMESTAMP_STRUCT_EXT>())).month as *const _ as usize },
        2usize,
        concat!(
            "Offset of field: ",
            stringify!(TIMESTAMP_STRUCT_EXT),
            "::",
            stringify!(month)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<TIMESTAMP_STRUCT_EXT>())).day as *const _ as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(TIMESTAMP_STRUCT_EXT),
            "::",
            stringify!(day)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<TIMESTAMP_STRUCT_EXT>())).hour as *const _ as usize },
        6usize,
        concat!(
            "Offset of field: ",
            stringify!(TIMESTAMP_STRUCT_EXT),
            "::",
            stringify!(hour)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<TIMESTAMP_STRUCT_EXT>())).minute as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(TIMESTAMP_STRUCT_EXT),
            "::",
            stringify!(minute)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<TIMESTAMP_STRUCT_EXT>())).second as *const _ as usize },
        10usize,
        concat!(
            "Offset of field: ",
            stringify!(TIMESTAMP_STRUCT_EXT),
            "::",
            stringify!(second)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<TIMESTAMP_STRUCT_EXT>())).fraction as *const _ as usize },
        12usize,
        concat!(
            "Offset of field: ",
            stringify!(TIMESTAMP_STRUCT_EXT),
            "::",
            stringify!(fraction)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<TIMESTAMP_STRUCT_EXT>())).fraction2 as *const _ as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(TIMESTAMP_STRUCT_EXT),
            "::",
            stringify!(fraction2)
        )
    );
}
pub type SQL_TIMESTAMP_STRUCT_EXT = TIMESTAMP_STRUCT_EXT;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct TIMESTAMP_STRUCT_EXT_TZ {
    pub year: SQLSMALLINT,
    pub month: SQLUSMALLINT,
    pub day: SQLUSMALLINT,
    pub hour: SQLUSMALLINT,
    pub minute: SQLUSMALLINT,
    pub second: SQLUSMALLINT,
    pub fraction: SQLUINTEGER,
    pub fraction2: SQLUINTEGER,
    pub timezone_hour: SQLSMALLINT,
    pub timezone_minute: SQLSMALLINT,
}
#[test]
fn bindgen_test_layout_TIMESTAMP_STRUCT_EXT_TZ() {
    assert_eq!(
        ::std::mem::size_of::<TIMESTAMP_STRUCT_EXT_TZ>(),
        24usize,
        concat!("Size of: ", stringify!(TIMESTAMP_STRUCT_EXT_TZ))
    );
    assert_eq!(
        ::std::mem::align_of::<TIMESTAMP_STRUCT_EXT_TZ>(),
        4usize,
        concat!("Alignment of ", stringify!(TIMESTAMP_STRUCT_EXT_TZ))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<TIMESTAMP_STRUCT_EXT_TZ>())).year as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(TIMESTAMP_STRUCT_EXT_TZ),
            "::",
            stringify!(year)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<TIMESTAMP_STRUCT_EXT_TZ>())).month as *const _ as usize },
        2usize,
        concat!(
            "Offset of field: ",
            stringify!(TIMESTAMP_STRUCT_EXT_TZ),
            "::",
            stringify!(month)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<TIMESTAMP_STRUCT_EXT_TZ>())).day as *const _ as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(TIMESTAMP_STRUCT_EXT_TZ),
            "::",
            stringify!(day)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<TIMESTAMP_STRUCT_EXT_TZ>())).hour as *const _ as usize },
        6usize,
        concat!(
            "Offset of field: ",
            stringify!(TIMESTAMP_STRUCT_EXT_TZ),
            "::",
            stringify!(hour)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<TIMESTAMP_STRUCT_EXT_TZ>())).minute as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(TIMESTAMP_STRUCT_EXT_TZ),
            "::",
            stringify!(minute)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<TIMESTAMP_STRUCT_EXT_TZ>())).second as *const _ as usize },
        10usize,
        concat!(
            "Offset of field: ",
            stringify!(TIMESTAMP_STRUCT_EXT_TZ),
            "::",
            stringify!(second)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<TIMESTAMP_STRUCT_EXT_TZ>())).fraction as *const _ as usize
        },
        12usize,
        concat!(
            "Offset of field: ",
            stringify!(TIMESTAMP_STRUCT_EXT_TZ),
            "::",
            stringify!(fraction)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<TIMESTAMP_STRUCT_EXT_TZ>())).fraction2 as *const _ as usize
        },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(TIMESTAMP_STRUCT_EXT_TZ),
            "::",
            stringify!(fraction2)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<TIMESTAMP_STRUCT_EXT_TZ>())).timezone_hour as *const _ as usize
        },
        20usize,
        concat!(
            "Offset of field: ",
            stringify!(TIMESTAMP_STRUCT_EXT_TZ),
            "::",
            stringify!(timezone_hour)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<TIMESTAMP_STRUCT_EXT_TZ>())).timezone_minute as *const _ as usize
        },
        22usize,
        concat!(
            "Offset of field: ",
            stringify!(TIMESTAMP_STRUCT_EXT_TZ),
            "::",
            stringify!(timezone_minute)
        )
    );
}
pub type SQL_TIMESTAMP_STRUCT_EXT_TZ = TIMESTAMP_STRUCT_EXT_TZ;
pub const SQLINTERVAL_SQL_IS_YEAR: SQLINTERVAL = 1;
pub const SQLINTERVAL_SQL_IS_MONTH: SQLINTERVAL = 2;
pub const SQLINTERVAL_SQL_IS_DAY: SQLINTERVAL = 3;
pub const SQLINTERVAL_SQL_IS_HOUR: SQLINTERVAL = 4;
pub const SQLINTERVAL_SQL_IS_MINUTE: SQLINTERVAL = 5;
pub const SQLINTERVAL_SQL_IS_SECOND: SQLINTERVAL = 6;
pub const SQLINTERVAL_SQL_IS_YEAR_TO_MONTH: SQLINTERVAL = 7;
pub const SQLINTERVAL_SQL_IS_DAY_TO_HOUR: SQLINTERVAL = 8;
pub const SQLINTERVAL_SQL_IS_DAY_TO_MINUTE: SQLINTERVAL = 9;
pub const SQLINTERVAL_SQL_IS_DAY_TO_SECOND: SQLINTERVAL = 10;
pub const SQLINTERVAL_SQL_IS_HOUR_TO_MINUTE: SQLINTERVAL = 11;
pub const SQLINTERVAL_SQL_IS_HOUR_TO_SECOND: SQLINTERVAL = 12;
pub const SQLINTERVAL_SQL_IS_MINUTE_TO_SECOND: SQLINTERVAL = 13;
pub type SQLINTERVAL = ::std::os::raw::c_uint;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct tagSQL_YEAR_MONTH {
    pub year: SQLUINTEGER,
    pub month: SQLUINTEGER,
}
#[test]
fn bindgen_test_layout_tagSQL_YEAR_MONTH() {
    assert_eq!(
        ::std::mem::size_of::<tagSQL_YEAR_MONTH>(),
        8usize,
        concat!("Size of: ", stringify!(tagSQL_YEAR_MONTH))
    );
    assert_eq!(
        ::std::mem::align_of::<tagSQL_YEAR_MONTH>(),
        4usize,
        concat!("Alignment of ", stringify!(tagSQL_YEAR_MONTH))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagSQL_YEAR_MONTH>())).year as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(tagSQL_YEAR_MONTH),
            "::",
            stringify!(year)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagSQL_YEAR_MONTH>())).month as *const _ as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(tagSQL_YEAR_MONTH),
            "::",
            stringify!(month)
        )
    );
}
pub type SQL_YEAR_MONTH_STRUCT = tagSQL_YEAR_MONTH;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct tagSQL_DAY_SECOND {
    pub day: SQLUINTEGER,
    pub hour: SQLUINTEGER,
    pub minute: SQLUINTEGER,
    pub second: SQLUINTEGER,
    pub fraction: SQLUINTEGER,
}
#[test]
fn bindgen_test_layout_tagSQL_DAY_SECOND() {
    assert_eq!(
        ::std::mem::size_of::<tagSQL_DAY_SECOND>(),
        20usize,
        concat!("Size of: ", stringify!(tagSQL_DAY_SECOND))
    );
    assert_eq!(
        ::std::mem::align_of::<tagSQL_DAY_SECOND>(),
        4usize,
        concat!("Alignment of ", stringify!(tagSQL_DAY_SECOND))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagSQL_DAY_SECOND>())).day as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(tagSQL_DAY_SECOND),
            "::",
            stringify!(day)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagSQL_DAY_SECOND>())).hour as *const _ as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(tagSQL_DAY_SECOND),
            "::",
            stringify!(hour)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagSQL_DAY_SECOND>())).minute as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(tagSQL_DAY_SECOND),
            "::",
            stringify!(minute)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagSQL_DAY_SECOND>())).second as *const _ as usize },
        12usize,
        concat!(
            "Offset of field: ",
            stringify!(tagSQL_DAY_SECOND),
            "::",
            stringify!(second)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagSQL_DAY_SECOND>())).fraction as *const _ as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(tagSQL_DAY_SECOND),
            "::",
            stringify!(fraction)
        )
    );
}
pub type SQL_DAY_SECOND_STRUCT = tagSQL_DAY_SECOND;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tagSQL_INTERVAL_STRUCT {
    pub interval_type: SQLINTERVAL,
    pub interval_sign: SQLSMALLINT,
    pub intval: tagSQL_INTERVAL_STRUCT__bindgen_ty_1,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union tagSQL_INTERVAL_STRUCT__bindgen_ty_1 {
    pub year_month: SQL_YEAR_MONTH_STRUCT,
    pub day_second: SQL_DAY_SECOND_STRUCT,
    _bindgen_union_align: [u32; 5usize],
}
#[test]
fn bindgen_test_layout_tagSQL_INTERVAL_STRUCT__bindgen_ty_1() {
    assert_eq!(
        ::std::mem::size_of::<tagSQL_INTERVAL_STRUCT__bindgen_ty_1>(),
        20usize,
        concat!(
            "Size of: ",
            stringify!(tagSQL_INTERVAL_STRUCT__bindgen_ty_1)
        )
    );
    assert_eq!(
        ::std::mem::align_of::<tagSQL_INTERVAL_STRUCT__bindgen_ty_1>(),
        4usize,
        concat!(
            "Alignment of ",
            stringify!(tagSQL_INTERVAL_STRUCT__bindgen_ty_1)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<tagSQL_INTERVAL_STRUCT__bindgen_ty_1>())).year_month as *const _
                as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(tagSQL_INTERVAL_STRUCT__bindgen_ty_1),
            "::",
            stringify!(year_month)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<tagSQL_INTERVAL_STRUCT__bindgen_ty_1>())).day_second as *const _
                as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(tagSQL_INTERVAL_STRUCT__bindgen_ty_1),
            "::",
            stringify!(day_second)
        )
    );
}
#[test]
fn bindgen_test_layout_tagSQL_INTERVAL_STRUCT() {
    assert_eq!(
        ::std::mem::size_of::<tagSQL_INTERVAL_STRUCT>(),
        28usize,
        concat!("Size of: ", stringify!(tagSQL_INTERVAL_STRUCT))
    );
    assert_eq!(
        ::std::mem::align_of::<tagSQL_INTERVAL_STRUCT>(),
        4usize,
        concat!("Alignment of ", stringify!(tagSQL_INTERVAL_STRUCT))
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<tagSQL_INTERVAL_STRUCT>())).interval_type as *const _ as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(tagSQL_INTERVAL_STRUCT),
            "::",
            stringify!(interval_type)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<tagSQL_INTERVAL_STRUCT>())).interval_sign as *const _ as usize
        },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(tagSQL_INTERVAL_STRUCT),
            "::",
            stringify!(interval_sign)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagSQL_INTERVAL_STRUCT>())).intval as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(tagSQL_INTERVAL_STRUCT),
            "::",
            stringify!(intval)
        )
    );
}
pub type SQL_INTERVAL_STRUCT = tagSQL_INTERVAL_STRUCT;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct tagSQL_NUMERIC_STRUCT {
    pub precision: SQLCHAR,
    pub scale: SQLSCHAR,
    pub sign: SQLCHAR,
    pub val: [SQLCHAR; 16usize],
}
#[test]
fn bindgen_test_layout_tagSQL_NUMERIC_STRUCT() {
    assert_eq!(
        ::std::mem::size_of::<tagSQL_NUMERIC_STRUCT>(),
        19usize,
        concat!("Size of: ", stringify!(tagSQL_NUMERIC_STRUCT))
    );
    assert_eq!(
        ::std::mem::align_of::<tagSQL_NUMERIC_STRUCT>(),
        1usize,
        concat!("Alignment of ", stringify!(tagSQL_NUMERIC_STRUCT))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagSQL_NUMERIC_STRUCT>())).precision as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(tagSQL_NUMERIC_STRUCT),
            "::",
            stringify!(precision)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagSQL_NUMERIC_STRUCT>())).scale as *const _ as usize },
        1usize,
        concat!(
            "Offset of field: ",
            stringify!(tagSQL_NUMERIC_STRUCT),
            "::",
            stringify!(scale)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagSQL_NUMERIC_STRUCT>())).sign as *const _ as usize },
        2usize,
        concat!(
            "Offset of field: ",
            stringify!(tagSQL_NUMERIC_STRUCT),
            "::",
            stringify!(sign)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagSQL_NUMERIC_STRUCT>())).val as *const _ as usize },
        3usize,
        concat!(
            "Offset of field: ",
            stringify!(tagSQL_NUMERIC_STRUCT),
            "::",
            stringify!(val)
        )
    );
}
pub type SQL_NUMERIC_STRUCT = tagSQL_NUMERIC_STRUCT;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct SQLDECIMAL64 {
    pub udec64: SQLDECIMAL64__bindgen_ty_1,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union SQLDECIMAL64__bindgen_ty_1 {
    pub dummy: SQLDOUBLE,
    pub dec64: [SQLCHAR; 8usize],
    _bindgen_union_align: u64,
}
#[test]
fn bindgen_test_layout_SQLDECIMAL64__bindgen_ty_1() {
    assert_eq!(
        ::std::mem::size_of::<SQLDECIMAL64__bindgen_ty_1>(),
        8usize,
        concat!("Size of: ", stringify!(SQLDECIMAL64__bindgen_ty_1))
    );
    assert_eq!(
        ::std::mem::align_of::<SQLDECIMAL64__bindgen_ty_1>(),
        8usize,
        concat!("Alignment of ", stringify!(SQLDECIMAL64__bindgen_ty_1))
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<SQLDECIMAL64__bindgen_ty_1>())).dummy as *const _ as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(SQLDECIMAL64__bindgen_ty_1),
            "::",
            stringify!(dummy)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<SQLDECIMAL64__bindgen_ty_1>())).dec64 as *const _ as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(SQLDECIMAL64__bindgen_ty_1),
            "::",
            stringify!(dec64)
        )
    );
}
#[test]
fn bindgen_test_layout_SQLDECIMAL64() {
    assert_eq!(
        ::std::mem::size_of::<SQLDECIMAL64>(),
        8usize,
        concat!("Size of: ", stringify!(SQLDECIMAL64))
    );
    assert_eq!(
        ::std::mem::align_of::<SQLDECIMAL64>(),
        8usize,
        concat!("Alignment of ", stringify!(SQLDECIMAL64))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<SQLDECIMAL64>())).udec64 as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(SQLDECIMAL64),
            "::",
            stringify!(udec64)
        )
    );
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct SQLDECIMAL128 {
    pub udec128: SQLDECIMAL128__bindgen_ty_1,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union SQLDECIMAL128__bindgen_ty_1 {
    pub dummy: SQLDOUBLE,
    pub dec128: [SQLCHAR; 16usize],
    _bindgen_union_align: [u64; 2usize],
}
#[test]
fn bindgen_test_layout_SQLDECIMAL128__bindgen_ty_1() {
    assert_eq!(
        ::std::mem::size_of::<SQLDECIMAL128__bindgen_ty_1>(),
        16usize,
        concat!("Size of: ", stringify!(SQLDECIMAL128__bindgen_ty_1))
    );
    assert_eq!(
        ::std::mem::align_of::<SQLDECIMAL128__bindgen_ty_1>(),
        8usize,
        concat!("Alignment of ", stringify!(SQLDECIMAL128__bindgen_ty_1))
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<SQLDECIMAL128__bindgen_ty_1>())).dummy as *const _ as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(SQLDECIMAL128__bindgen_ty_1),
            "::",
            stringify!(dummy)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<SQLDECIMAL128__bindgen_ty_1>())).dec128 as *const _ as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(SQLDECIMAL128__bindgen_ty_1),
            "::",
            stringify!(dec128)
        )
    );
}
#[test]
fn bindgen_test_layout_SQLDECIMAL128() {
    assert_eq!(
        ::std::mem::size_of::<SQLDECIMAL128>(),
        16usize,
        concat!("Size of: ", stringify!(SQLDECIMAL128))
    );
    assert_eq!(
        ::std::mem::align_of::<SQLDECIMAL128>(),
        8usize,
        concat!("Alignment of ", stringify!(SQLDECIMAL128))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<SQLDECIMAL128>())).udec128 as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(SQLDECIMAL128),
            "::",
            stringify!(udec128)
        )
    );
}
extern "C" {
    pub fn SQLAllocConnect(henv: SQLHENV, phdbc: *mut SQLHDBC) -> SQLRETURN;
}
extern "C" {
    pub fn SQLAllocEnv(phenv: *mut SQLHENV) -> SQLRETURN;
}
extern "C" {
    pub fn SQLAllocStmt(hdbc: SQLHDBC, phstmt: *mut SQLHSTMT) -> SQLRETURN;
}
extern "C" {
    pub fn SQLAllocHandle(
        fHandleType: SQLSMALLINT,
        hInput: SQLHANDLE,
        phOutput: *mut SQLHANDLE,
    ) -> SQLRETURN;
}
extern "C" {
    pub fn SQLBindCol(
        hstmt: SQLHSTMT,
        icol: SQLUSMALLINT,
        fCType: SQLSMALLINT,
        rgbValue: SQLPOINTER,
        cbValueMax: SQLINTEGER,
        pcbValue: *mut SQLINTEGER,
    ) -> SQLRETURN;
}
extern "C" {
    pub fn SQLCancel(hstmt: SQLHSTMT) -> SQLRETURN;
}
extern "C" {
    pub fn SQLColAttribute(
        hstmt: SQLHSTMT,
        icol: SQLUSMALLINT,
        fDescType: SQLUSMALLINT,
        rgbDesc: SQLPOINTER,
        cbDescMax: SQLSMALLINT,
        pcbDesc: *mut SQLSMALLINT,
        pfDesc: SQLPOINTER,
    ) -> SQLRETURN;
}
extern "C" {
    pub fn SQLConnect(
        hdbc: SQLHDBC,
        szDSN: *mut SQLCHAR,
        cbDSN: SQLSMALLINT,
        szUID: *mut SQLCHAR,
        cbUID: SQLSMALLINT,
        szAuthStr: *mut SQLCHAR,
        cbAuthStr: SQLSMALLINT,
    ) -> SQLRETURN;
}
extern "C" {
    pub fn SQLDescribeCol(
        hstmt: SQLHSTMT,
        icol: SQLUSMALLINT,
        szColName: *mut SQLCHAR,
        cbColNameMax: SQLSMALLINT,
        pcbColName: *mut SQLSMALLINT,
        pfSqlType: *mut SQLSMALLINT,
        pcbColDef: *mut SQLUINTEGER,
        pibScale: *mut SQLSMALLINT,
        pfNullable: *mut SQLSMALLINT,
    ) -> SQLRETURN;
}
extern "C" {
    pub fn SQLDisconnect(hdbc: SQLHDBC) -> SQLRETURN;
}
extern "C" {
    pub fn SQLError(
        henv: SQLHENV,
        hdbc: SQLHDBC,
        hstmt: SQLHSTMT,
        szSqlState: *mut SQLCHAR,
        pfNativeError: *mut SQLINTEGER,
        szErrorMsg: *mut SQLCHAR,
        cbErrorMsgMax: SQLSMALLINT,
        pcbErrorMsg: *mut SQLSMALLINT,
    ) -> SQLRETURN;
}
extern "C" {
    pub fn SQLExecDirect(
        hstmt: SQLHSTMT,
        szSqlStr: *mut SQLCHAR,
        cbSqlStr: SQLINTEGER,
    ) -> SQLRETURN;
}
extern "C" {
    pub fn SQLExecute(hstmt: SQLHSTMT) -> SQLRETURN;
}
extern "C" {
    pub fn SQLFetch(hstmt: SQLHSTMT) -> SQLRETURN;
}
extern "C" {
    pub fn SQLFreeConnect(hdbc: SQLHDBC) -> SQLRETURN;
}
extern "C" {
    pub fn SQLFreeEnv(henv: SQLHENV) -> SQLRETURN;
}
extern "C" {
    pub fn SQLFreeStmt(hstmt: SQLHSTMT, fOption: SQLUSMALLINT) -> SQLRETURN;
}
extern "C" {
    pub fn SQLCloseCursor(hStmt: SQLHSTMT) -> SQLRETURN;
}
extern "C" {
    pub fn SQLGetCursorName(
        hstmt: SQLHSTMT,
        szCursor: *mut SQLCHAR,
        cbCursorMax: SQLSMALLINT,
        pcbCursor: *mut SQLSMALLINT,
    ) -> SQLRETURN;
}
extern "C" {
    pub fn SQLGetData(
        hstmt: SQLHSTMT,
        icol: SQLUSMALLINT,
        fCType: SQLSMALLINT,
        rgbValue: SQLPOINTER,
        cbValueMax: SQLINTEGER,
        pcbValue: *mut SQLINTEGER,
    ) -> SQLRETURN;
}
extern "C" {
    pub fn SQLNumResultCols(hstmt: SQLHSTMT, pccol: *mut SQLSMALLINT) -> SQLRETURN;
}
extern "C" {
    pub fn SQLPrepare(hstmt: SQLHSTMT, szSqlStr: *mut SQLCHAR, cbSqlStr: SQLINTEGER) -> SQLRETURN;
}
extern "C" {
    pub fn SQLRowCount(hstmt: SQLHSTMT, pcrow: *mut SQLINTEGER) -> SQLRETURN;
}
extern "C" {
    pub fn SQLSetCursorName(
        hstmt: SQLHSTMT,
        szCursor: *mut SQLCHAR,
        cbCursor: SQLSMALLINT,
    ) -> SQLRETURN;
}
extern "C" {
    pub fn SQLSetParam(
        hstmt: SQLHSTMT,
        ipar: SQLUSMALLINT,
        fCType: SQLSMALLINT,
        fSqlType: SQLSMALLINT,
        cbParamDef: SQLUINTEGER,
        ibScale: SQLSMALLINT,
        rgbValue: SQLPOINTER,
        pcbValue: *mut SQLINTEGER,
    ) -> SQLRETURN;
}
extern "C" {
    pub fn SQLTransact(henv: SQLHENV, hdbc: SQLHDBC, fType: SQLUSMALLINT) -> SQLRETURN;
}
extern "C" {
    pub fn SQLEndTran(
        fHandleType: SQLSMALLINT,
        hHandle: SQLHANDLE,
        fType: SQLSMALLINT,
    ) -> SQLRETURN;
}
extern "C" {
    pub fn SQLFreeHandle(fHandleType: SQLSMALLINT, hHandle: SQLHANDLE) -> SQLRETURN;
}
extern "C" {
    pub fn SQLGetDiagRec(
        fHandleType: SQLSMALLINT,
        hHandle: SQLHANDLE,
        iRecNumber: SQLSMALLINT,
        pszSqlState: *mut SQLCHAR,
        pfNativeError: *mut SQLINTEGER,
        pszErrorMsg: *mut SQLCHAR,
        cbErrorMsgMax: SQLSMALLINT,
        pcbErrorMsg: *mut SQLSMALLINT,
    ) -> SQLRETURN;
}
extern "C" {
    pub fn SQLGetDiagField(
        fHandleType: SQLSMALLINT,
        hHandle: SQLHANDLE,
        iRecNumber: SQLSMALLINT,
        fDiagIdentifier: SQLSMALLINT,
        pDiagInfo: SQLPOINTER,
        cbDiagInfoMax: SQLSMALLINT,
        pcbDiagInfo: *mut SQLSMALLINT,
    ) -> SQLRETURN;
}
extern "C" {
    pub fn SQLCopyDesc(hDescSource: SQLHDESC, hDescTarget: SQLHDESC) -> SQLRETURN;
}
extern "C" {
    pub fn SQLCreateDb(
        hDbc: SQLHDBC,
        szDB: *mut SQLCHAR,
        cbDB: SQLINTEGER,
        szCodeset: *mut SQLCHAR,
        cbCodeset: SQLINTEGER,
        szMode: *mut SQLCHAR,
        cbMode: SQLINTEGER,
    ) -> SQLRETURN;
}
extern "C" {
    pub fn SQLDropDb(hDbc: SQLHDBC, szDB: *mut SQLCHAR, cbDB: SQLINTEGER) -> SQLRETURN;
}
extern "C" {
    pub fn SQLCreatePkg(
        hDbc: SQLHDBC,
        szBindFileName: *mut SQLCHAR,
        cbBindFileName: SQLINTEGER,
        szBindOpts: *mut SQLCHAR,
        cbBindOpts: SQLINTEGER,
    ) -> SQLRETURN;
}
extern "C" {
    pub fn SQLGetDescField(
        DescriptorHandle: SQLHDESC,
        RecNumber: SQLSMALLINT,
        FieldIdentifier: SQLSMALLINT,
        Value: SQLPOINTER,
        BufferLength: SQLINTEGER,
        StringLength: *mut SQLINTEGER,
    ) -> SQLRETURN;
}
extern "C" {
    pub fn SQLGetDescRec(
        DescriptorHandle: SQLHDESC,
        RecNumber: SQLSMALLINT,
        Name: *mut SQLCHAR,
        BufferLength: SQLSMALLINT,
        StringLength: *mut SQLSMALLINT,
        Type: *mut SQLSMALLINT,
        SubType: *mut SQLSMALLINT,
        Length: *mut SQLINTEGER,
        Precision: *mut SQLSMALLINT,
        Scale: *mut SQLSMALLINT,
        Nullable: *mut SQLSMALLINT,
    ) -> SQLRETURN;
}
extern "C" {
    pub fn SQLSetDescField(
        DescriptorHandle: SQLHDESC,
        RecNumber: SQLSMALLINT,
        FieldIdentifier: SQLSMALLINT,
        Value: SQLPOINTER,
        BufferLength: SQLINTEGER,
    ) -> SQLRETURN;
}
extern "C" {
    pub fn SQLSetDescRec(
        DescriptorHandle: SQLHDESC,
        RecNumber: SQLSMALLINT,
        Type: SQLSMALLINT,
        SubType: SQLSMALLINT,
        Length: SQLINTEGER,
        Precision: SQLSMALLINT,
        Scale: SQLSMALLINT,
        Data: SQLPOINTER,
        StringLength: *mut SQLINTEGER,
        Indicator: *mut SQLINTEGER,
    ) -> SQLRETURN;
}
pub type LPWSTR = *mut SQLWCHAR;
pub type DWORD = sqluint32;
pub type BOOL = ::std::os::raw::c_uint;
pub type WCHAR = wchar_t;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _TAGGUID {
    pub Data1: ::std::os::raw::c_ulong,
    pub Data2: ::std::os::raw::c_ushort,
    pub Data3: ::std::os::raw::c_ushort,
    pub Data4: [::std::os::raw::c_uchar; 8usize],
}
#[test]
fn bindgen_test_layout__TAGGUID() {
    assert_eq!(
        ::std::mem::size_of::<_TAGGUID>(),
        24usize,
        concat!("Size of: ", stringify!(_TAGGUID))
    );
    assert_eq!(
        ::std::mem::align_of::<_TAGGUID>(),
        8usize,
        concat!("Alignment of ", stringify!(_TAGGUID))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<_TAGGUID>())).Data1 as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(_TAGGUID),
            "::",
            stringify!(Data1)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<_TAGGUID>())).Data2 as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(_TAGGUID),
            "::",
            stringify!(Data2)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<_TAGGUID>())).Data3 as *const _ as usize },
        10usize,
        concat!(
            "Offset of field: ",
            stringify!(_TAGGUID),
            "::",
            stringify!(Data3)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<_TAGGUID>())).Data4 as *const _ as usize },
        12usize,
        concat!(
            "Offset of field: ",
            stringify!(_TAGGUID),
            "::",
            stringify!(Data4)
        )
    );
}
pub type TAGGUID = _TAGGUID;
pub type SQLSTATE = [SQLTCHAR; 6usize];
extern "C" {
    pub fn SQLDriverConnect(
        hdbc: SQLHDBC,
        hwnd: SQLHWND,
        szConnStrIn: *mut SQLCHAR,
        cchConnStrIn: SQLSMALLINT,
        szConnStrOut: *mut SQLCHAR,
        cchConnStrOutMax: SQLSMALLINT,
        pcchConnStrOut: *mut SQLSMALLINT,
        fDriverCompletion: SQLUSMALLINT,
    ) -> SQLRETURN;
}
extern "C" {
    pub fn SQLBrowseConnect(
        hdbc: SQLHDBC,
        szConnStrIn: *mut SQLCHAR,
        cchConnStrIn: SQLSMALLINT,
        szConnStrOut: *mut SQLCHAR,
        cchConnStrOutMax: SQLSMALLINT,
        pcchConnStrOut: *mut SQLSMALLINT,
    ) -> SQLRETURN;
}
extern "C" {
    pub fn SQLBulkOperations(StatementHandle: SQLHSTMT, Operation: SQLSMALLINT) -> SQLRETURN;
}
extern "C" {
    pub fn SQLColAttributes(
        hstmt: SQLHSTMT,
        icol: SQLUSMALLINT,
        fDescType: SQLUSMALLINT,
        rgbDesc: SQLPOINTER,
        cbDescMax: SQLSMALLINT,
        pcbDesc: *mut SQLSMALLINT,
        pfDesc: *mut SQLINTEGER,
    ) -> SQLRETURN;
}
extern "C" {
    pub fn SQLColumnPrivileges(
        hstmt: SQLHSTMT,
        szCatalogName: *mut SQLCHAR,
        cchCatalogName: SQLSMALLINT,
        szSchemaName: *mut SQLCHAR,
        cchSchemaName: SQLSMALLINT,
        szTableName: *mut SQLCHAR,
        cchTableName: SQLSMALLINT,
        szColumnName: *mut SQLCHAR,
        cchColumnName: SQLSMALLINT,
    ) -> SQLRETURN;
}
extern "C" {
    pub fn SQLDescribeParam(
        hstmt: SQLHSTMT,
        ipar: SQLUSMALLINT,
        pfSqlType: *mut SQLSMALLINT,
        pcbParamDef: *mut SQLUINTEGER,
        pibScale: *mut SQLSMALLINT,
        pfNullable: *mut SQLSMALLINT,
    ) -> SQLRETURN;
}
extern "C" {
    pub fn SQLExtendedFetch(
        hstmt: SQLHSTMT,
        fFetchType: SQLUSMALLINT,
        irow: SQLINTEGER,
        pcrow: *mut SQLUINTEGER,
        rgfRowStatus: *mut SQLUSMALLINT,
    ) -> SQLRETURN;
}
extern "C" {
    pub fn SQLForeignKeys(
        hstmt: SQLHSTMT,
        szPkCatalogName: *mut SQLCHAR,
        cchPkCatalogName: SQLSMALLINT,
        szPkSchemaName: *mut SQLCHAR,
        cchPkSchemaName: SQLSMALLINT,
        szPkTableName: *mut SQLCHAR,
        cchPkTableName: SQLSMALLINT,
        szFkCatalogName: *mut SQLCHAR,
        cchFkCatalogName: SQLSMALLINT,
        szFkSchemaName: *mut SQLCHAR,
        cchFkSchemaName: SQLSMALLINT,
        szFkTableName: *mut SQLCHAR,
        cchFkTableName: SQLSMALLINT,
    ) -> SQLRETURN;
}
extern "C" {
    pub fn SQLMoreResults(hstmt: SQLHSTMT) -> SQLRETURN;
}
extern "C" {
    pub fn SQLNativeSql(
        hdbc: SQLHDBC,
        szSqlStrIn: *mut SQLCHAR,
        cchSqlStrIn: SQLINTEGER,
        szSqlStr: *mut SQLCHAR,
        cchSqlStrMax: SQLINTEGER,
        pcbSqlStr: *mut SQLINTEGER,
    ) -> SQLRETURN;
}
extern "C" {
    pub fn SQLNumParams(hstmt: SQLHSTMT, pcpar: *mut SQLSMALLINT) -> SQLRETURN;
}
extern "C" {
    pub fn SQLParamOptions(
        hstmt: SQLHSTMT,
        crow: SQLUINTEGER,
        pirow: *mut SQLUINTEGER,
    ) -> SQLRETURN;
}
extern "C" {
    pub fn SQLPrimaryKeys(
        hstmt: SQLHSTMT,
        szCatalogName: *mut SQLCHAR,
        cchCatalogName: SQLSMALLINT,
        szSchemaName: *mut SQLCHAR,
        cchSchemaName: SQLSMALLINT,
        szTableName: *mut SQLCHAR,
        cchTableName: SQLSMALLINT,
    ) -> SQLRETURN;
}
extern "C" {
    pub fn SQLProcedureColumns(
        hstmt: SQLHSTMT,
        szCatalogName: *mut SQLCHAR,
        cchCatalogName: SQLSMALLINT,
        szSchemaName: *mut SQLCHAR,
        cchSchemaName: SQLSMALLINT,
        szProcName: *mut SQLCHAR,
        cchProcName: SQLSMALLINT,
        szColumnName: *mut SQLCHAR,
        cchColumnName: SQLSMALLINT,
    ) -> SQLRETURN;
}
extern "C" {
    pub fn SQLProcedures(
        hstmt: SQLHSTMT,
        szCatalogName: *mut SQLCHAR,
        cchCatalogName: SQLSMALLINT,
        szSchemaName: *mut SQLCHAR,
        cchSchemaName: SQLSMALLINT,
        szProcName: *mut SQLCHAR,
        cchProcName: SQLSMALLINT,
    ) -> SQLRETURN;
}
extern "C" {
    pub fn SQLSetPos(
        hstmt: SQLHSTMT,
        irow: SQLUSMALLINT,
        fOption: SQLUSMALLINT,
        fLock: SQLUSMALLINT,
    ) -> SQLRETURN;
}
extern "C" {
    pub fn SQLTablePrivileges(
        hstmt: SQLHSTMT,
        szCatalogName: *mut SQLCHAR,
        cchCatalogName: SQLSMALLINT,
        szSchemaName: *mut SQLCHAR,
        cchSchemaName: SQLSMALLINT,
        szTableName: *mut SQLCHAR,
        cchTableName: SQLSMALLINT,
    ) -> SQLRETURN;
}
extern "C" {
    pub fn SQLDrivers(
        henv: SQLHENV,
        fDirection: SQLUSMALLINT,
        szDriverDesc: *mut SQLCHAR,
        cchDriverDescMax: SQLSMALLINT,
        pcchDriverDesc: *mut SQLSMALLINT,
        szDriverAttributes: *mut SQLCHAR,
        cchDrvrAttrMax: SQLSMALLINT,
        pcchDrvrAttr: *mut SQLSMALLINT,
    ) -> SQLRETURN;
}
extern "C" {
    pub fn SQLBindParameter(
        hstmt: SQLHSTMT,
        ipar: SQLUSMALLINT,
        fParamType: SQLSMALLINT,
        fCType: SQLSMALLINT,
        fSqlType: SQLSMALLINT,
        cbColDef: SQLUINTEGER,
        ibScale: SQLSMALLINT,
        rgbValue: SQLPOINTER,
        cbValueMax: SQLINTEGER,
        pcbValue: *mut SQLINTEGER,
    ) -> SQLRETURN;
}
extern "C" {
    pub fn SQLAllocHandleStd(
        fHandleType: SQLSMALLINT,
        hInput: SQLHANDLE,
        phOutput: *mut SQLHANDLE,
    ) -> SQLRETURN;
}
extern "C" {
    pub fn SQLSetScrollOptions(
        hstmt: SQLHSTMT,
        fConcurrency: SQLUSMALLINT,
        crowKeyset: SQLINTEGER,
        crowRowset: SQLUSMALLINT,
    ) -> SQLRETURN;
}
extern "C" {
    pub fn TraceOpenLogFile(
        szFileName: LPWSTR,
        lpwszOutputMsg: LPWSTR,
        cbOutputMsg: DWORD,
    ) -> RETCODE;
}
extern "C" {
    pub fn TraceCloseLogFile() -> RETCODE;
}
extern "C" {
    pub fn TraceReturn(arg1: RETCODE, arg2: RETCODE);
}
extern "C" {
    pub fn TraceVersion() -> DWORD;
}
extern "C" {
    pub fn TraceVSControl(arg1: DWORD) -> RETCODE;
}
extern "C" {
    pub fn ODBCSetTryWaitValue(dwValue: DWORD) -> BOOL;
}
extern "C" {
    pub fn ODBCGetTryWaitValue() -> DWORD;
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tagODBC_VS_ARGS {
    pub pguidEvent: *const TAGGUID,
    pub dwFlags: DWORD,
    pub __bindgen_anon_1: tagODBC_VS_ARGS__bindgen_ty_1,
    pub __bindgen_anon_2: tagODBC_VS_ARGS__bindgen_ty_2,
    pub RetCode: RETCODE,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union tagODBC_VS_ARGS__bindgen_ty_1 {
    pub wszArg: *mut WCHAR,
    pub szArg: *mut ::std::os::raw::c_char,
    _bindgen_union_align: u64,
}
#[test]
fn bindgen_test_layout_tagODBC_VS_ARGS__bindgen_ty_1() {
    assert_eq!(
        ::std::mem::size_of::<tagODBC_VS_ARGS__bindgen_ty_1>(),
        8usize,
        concat!("Size of: ", stringify!(tagODBC_VS_ARGS__bindgen_ty_1))
    );
    assert_eq!(
        ::std::mem::align_of::<tagODBC_VS_ARGS__bindgen_ty_1>(),
        8usize,
        concat!("Alignment of ", stringify!(tagODBC_VS_ARGS__bindgen_ty_1))
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<tagODBC_VS_ARGS__bindgen_ty_1>())).wszArg as *const _ as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(tagODBC_VS_ARGS__bindgen_ty_1),
            "::",
            stringify!(wszArg)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<tagODBC_VS_ARGS__bindgen_ty_1>())).szArg as *const _ as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(tagODBC_VS_ARGS__bindgen_ty_1),
            "::",
            stringify!(szArg)
        )
    );
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union tagODBC_VS_ARGS__bindgen_ty_2 {
    pub wszCorrelation: *mut WCHAR,
    pub szCorrelation: *mut ::std::os::raw::c_char,
    _bindgen_union_align: u64,
}
#[test]
fn bindgen_test_layout_tagODBC_VS_ARGS__bindgen_ty_2() {
    assert_eq!(
        ::std::mem::size_of::<tagODBC_VS_ARGS__bindgen_ty_2>(),
        8usize,
        concat!("Size of: ", stringify!(tagODBC_VS_ARGS__bindgen_ty_2))
    );
    assert_eq!(
        ::std::mem::align_of::<tagODBC_VS_ARGS__bindgen_ty_2>(),
        8usize,
        concat!("Alignment of ", stringify!(tagODBC_VS_ARGS__bindgen_ty_2))
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<tagODBC_VS_ARGS__bindgen_ty_2>())).wszCorrelation as *const _
                as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(tagODBC_VS_ARGS__bindgen_ty_2),
            "::",
            stringify!(wszCorrelation)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<tagODBC_VS_ARGS__bindgen_ty_2>())).szCorrelation as *const _
                as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(tagODBC_VS_ARGS__bindgen_ty_2),
            "::",
            stringify!(szCorrelation)
        )
    );
}
#[test]
fn bindgen_test_layout_tagODBC_VS_ARGS() {
    assert_eq!(
        ::std::mem::size_of::<tagODBC_VS_ARGS>(),
        40usize,
        concat!("Size of: ", stringify!(tagODBC_VS_ARGS))
    );
    assert_eq!(
        ::std::mem::align_of::<tagODBC_VS_ARGS>(),
        8usize,
        concat!("Alignment of ", stringify!(tagODBC_VS_ARGS))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagODBC_VS_ARGS>())).pguidEvent as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(tagODBC_VS_ARGS),
            "::",
            stringify!(pguidEvent)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagODBC_VS_ARGS>())).dwFlags as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(tagODBC_VS_ARGS),
            "::",
            stringify!(dwFlags)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<tagODBC_VS_ARGS>())).RetCode as *const _ as usize },
        32usize,
        concat!(
            "Offset of field: ",
            stringify!(tagODBC_VS_ARGS),
            "::",
            stringify!(RetCode)
        )
    );
}
pub type ODBC_VS_ARGS = tagODBC_VS_ARGS;
pub type PODBC_VS_ARGS = *mut tagODBC_VS_ARGS;
extern "C" {
    pub fn FireVSDebugEvent(arg1: PODBC_VS_ARGS);
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SQL_NET_STATS {
    pub iNetStatsLength: SQLINTEGER,
    pub uiNetStatsServerTime: SQLUBIGINT,
    pub uiNetStatsNetworkTime: SQLUBIGINT,
    pub uiNetStatsBytesSent: SQLUBIGINT,
    pub uiNetStatsBytesReceived: SQLUBIGINT,
    pub uiNetStatsRoundTrips: SQLUBIGINT,
}
#[test]
fn bindgen_test_layout_SQL_NET_STATS() {
    assert_eq!(
        ::std::mem::size_of::<SQL_NET_STATS>(),
        48usize,
        concat!("Size of: ", stringify!(SQL_NET_STATS))
    );
    assert_eq!(
        ::std::mem::align_of::<SQL_NET_STATS>(),
        8usize,
        concat!("Alignment of ", stringify!(SQL_NET_STATS))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<SQL_NET_STATS>())).iNetStatsLength as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(SQL_NET_STATS),
            "::",
            stringify!(iNetStatsLength)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<SQL_NET_STATS>())).uiNetStatsServerTime as *const _ as usize
        },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(SQL_NET_STATS),
            "::",
            stringify!(uiNetStatsServerTime)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<SQL_NET_STATS>())).uiNetStatsNetworkTime as *const _ as usize
        },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(SQL_NET_STATS),
            "::",
            stringify!(uiNetStatsNetworkTime)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<SQL_NET_STATS>())).uiNetStatsBytesSent as *const _ as usize
        },
        24usize,
        concat!(
            "Offset of field: ",
            stringify!(SQL_NET_STATS),
            "::",
            stringify!(uiNetStatsBytesSent)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<SQL_NET_STATS>())).uiNetStatsBytesReceived as *const _ as usize
        },
        32usize,
        concat!(
            "Offset of field: ",
            stringify!(SQL_NET_STATS),
            "::",
            stringify!(uiNetStatsBytesReceived)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<SQL_NET_STATS>())).uiNetStatsRoundTrips as *const _ as usize
        },
        40usize,
        concat!(
            "Offset of field: ",
            stringify!(SQL_NET_STATS),
            "::",
            stringify!(uiNetStatsRoundTrips)
        )
    );
}
extern "C" {
    pub fn SQLColumns(
        hstmt: SQLHSTMT,
        szCatalogName: *mut SQLCHAR,
        cbCatalogName: SQLSMALLINT,
        szSchemaName: *mut SQLCHAR,
        cbSchemaName: SQLSMALLINT,
        szTableName: *mut SQLCHAR,
        cbTableName: SQLSMALLINT,
        szColumnName: *mut SQLCHAR,
        cbColumnName: SQLSMALLINT,
    ) -> SQLRETURN;
}
extern "C" {
    pub fn SQLDataSources(
        henv: SQLHENV,
        fDirection: SQLUSMALLINT,
        szDSN: *mut SQLCHAR,
        cbDSNMax: SQLSMALLINT,
        pcbDSN: *mut SQLSMALLINT,
        szDescription: *mut SQLCHAR,
        cbDescriptionMax: SQLSMALLINT,
        pcbDescription: *mut SQLSMALLINT,
    ) -> SQLRETURN;
}
extern "C" {
    pub fn SQLFetchScroll(
        StatementHandle: SQLHSTMT,
        FetchOrientation: SQLSMALLINT,
        FetchOffset: SQLINTEGER,
    ) -> SQLRETURN;
}
extern "C" {
    pub fn SQLGetConnectAttr(
        ConnectionHandle: SQLHDBC,
        Attribute: SQLINTEGER,
        Value: SQLPOINTER,
        BufferLength: SQLINTEGER,
        StringLength: *mut SQLINTEGER,
    ) -> SQLRETURN;
}
extern "C" {
    pub fn SQLGetConnectOption(
        hdbc: SQLHDBC,
        fOption: SQLUSMALLINT,
        pvParam: SQLPOINTER,
    ) -> SQLRETURN;
}
extern "C" {
    pub fn SQLGetFunctions(
        hdbc: SQLHDBC,
        fFunction: SQLUSMALLINT,
        pfExists: *mut SQLUSMALLINT,
    ) -> SQLRETURN;
}
extern "C" {
    pub fn SQLGetInfo(
        hdbc: SQLHDBC,
        fInfoType: SQLUSMALLINT,
        rgbInfoValue: SQLPOINTER,
        cbInfoValueMax: SQLSMALLINT,
        pcbInfoValue: *mut SQLSMALLINT,
    ) -> SQLRETURN;
}
extern "C" {
    pub fn SQLGetStmtAttr(
        StatementHandle: SQLHSTMT,
        Attribute: SQLINTEGER,
        Value: SQLPOINTER,
        BufferLength: SQLINTEGER,
        StringLength: *mut SQLINTEGER,
    ) -> SQLRETURN;
}
extern "C" {
    pub fn SQLGetStmtOption(
        hstmt: SQLHSTMT,
        fOption: SQLUSMALLINT,
        pvParam: SQLPOINTER,
    ) -> SQLRETURN;
}
extern "C" {
    pub fn SQLGetTypeInfo(hstmt: SQLHSTMT, fSqlType: SQLSMALLINT) -> SQLRETURN;
}
extern "C" {
    pub fn SQLParamData(hstmt: SQLHSTMT, prgbValue: *mut SQLPOINTER) -> SQLRETURN;
}
extern "C" {
    pub fn SQLPutData(hstmt: SQLHSTMT, rgbValue: SQLPOINTER, cbValue: SQLINTEGER) -> SQLRETURN;
}
extern "C" {
    pub fn SQLSetConnectAttr(
        hdbc: SQLHDBC,
        fOption: SQLINTEGER,
        pvParam: SQLPOINTER,
        fStrLen: SQLINTEGER,
    ) -> SQLRETURN;
}
extern "C" {
    pub fn SQLSetConnectOption(
        hdbc: SQLHDBC,
        fOption: SQLUSMALLINT,
        vParam: SQLUINTEGER,
    ) -> SQLRETURN;
}
extern "C" {
    pub fn SQLSetStmtAttr(
        hstmt: SQLHSTMT,
        fOption: SQLINTEGER,
        pvParam: SQLPOINTER,
        fStrLen: SQLINTEGER,
    ) -> SQLRETURN;
}
extern "C" {
    pub fn SQLSetStmtOption(
        hstmt: SQLHSTMT,
        fOption: SQLUSMALLINT,
        vParam: SQLUINTEGER,
    ) -> SQLRETURN;
}
extern "C" {
    pub fn SQLSpecialColumns(
        hstmt: SQLHSTMT,
        fColType: SQLUSMALLINT,
        szCatalogName: *mut SQLCHAR,
        cbCatalogName: SQLSMALLINT,
        szSchemaName: *mut SQLCHAR,
        cbSchemaName: SQLSMALLINT,
        szTableName: *mut SQLCHAR,
        cbTableName: SQLSMALLINT,
        fScope: SQLUSMALLINT,
        fNullable: SQLUSMALLINT,
    ) -> SQLRETURN;
}
extern "C" {
    pub fn SQLStatistics(
        hstmt: SQLHSTMT,
        szCatalogName: *mut SQLCHAR,
        cbCatalogName: SQLSMALLINT,
        szSchemaName: *mut SQLCHAR,
        cbSchemaName: SQLSMALLINT,
        szTableName: *mut SQLCHAR,
        cbTableName: SQLSMALLINT,
        fUnique: SQLUSMALLINT,
        fAccuracy: SQLUSMALLINT,
    ) -> SQLRETURN;
}
extern "C" {
    pub fn SQLTables(
        hstmt: SQLHSTMT,
        szCatalogName: *mut SQLCHAR,
        cbCatalogName: SQLSMALLINT,
        szSchemaName: *mut SQLCHAR,
        cbSchemaName: SQLSMALLINT,
        szTableName: *mut SQLCHAR,
        cbTableName: SQLSMALLINT,
        szTableType: *mut SQLCHAR,
        cbTableType: SQLSMALLINT,
    ) -> SQLRETURN;
}
extern "C" {
    pub fn SQLNextResult(hstmtSource: SQLHSTMT, hstmtTarget: SQLHSTMT) -> SQLRETURN;
}
extern "C" {
    pub fn SQLColAttributeW(
        hstmt: SQLHSTMT,
        iCol: SQLUSMALLINT,
        iField: SQLUSMALLINT,
        pCharAttr: SQLPOINTER,
        cbCharAttrMax: SQLSMALLINT,
        pcbCharAttr: *mut SQLSMALLINT,
        pNumAttr: SQLPOINTER,
    ) -> SQLRETURN;
}
extern "C" {
    pub fn SQLColAttributesW(
        hstmt: SQLHSTMT,
        icol: SQLUSMALLINT,
        fDescType: SQLUSMALLINT,
        rgbDesc: SQLPOINTER,
        cbDescMax: SQLSMALLINT,
        pcbDesc: *mut SQLSMALLINT,
        pfDesc: *mut SQLINTEGER,
    ) -> SQLRETURN;
}
extern "C" {
    pub fn SQLConnectW(
        hdbc: SQLHDBC,
        szDSN: *mut SQLWCHAR,
        cbDSN: SQLSMALLINT,
        szUID: *mut SQLWCHAR,
        cbUID: SQLSMALLINT,
        szAuthStr: *mut SQLWCHAR,
        cbAuthStr: SQLSMALLINT,
    ) -> SQLRETURN;
}
extern "C" {
    pub fn SQLConnectWInt(
        hdbc: SQLHDBC,
        szDSN: *mut SQLWCHAR,
        cbDSN: SQLSMALLINT,
        szUID: *mut SQLWCHAR,
        cbUID: SQLSMALLINT,
        szAuthStr: *mut SQLWCHAR,
        cbAuthStr: SQLSMALLINT,
    ) -> SQLRETURN;
}
extern "C" {
    pub fn SQLDescribeColW(
        hstmt: SQLHSTMT,
        icol: SQLUSMALLINT,
        szColName: *mut SQLWCHAR,
        cbColNameMax: SQLSMALLINT,
        pcbColName: *mut SQLSMALLINT,
        pfSqlType: *mut SQLSMALLINT,
        pcbColDef: *mut SQLUINTEGER,
        pibScale: *mut SQLSMALLINT,
        pfNullable: *mut SQLSMALLINT,
    ) -> SQLRETURN;
}
extern "C" {
    pub fn SQLErrorW(
        henv: SQLHENV,
        hdbc: SQLHDBC,
        hstmt: SQLHSTMT,
        szSqlState: *mut SQLWCHAR,
        pfNativeError: *mut SQLINTEGER,
        szErrorMsg: *mut SQLWCHAR,
        cbErrorMsgMax: SQLSMALLINT,
        pcbErrorMsg: *mut SQLSMALLINT,
    ) -> SQLRETURN;
}
extern "C" {
    pub fn SQLExecDirectW(
        hstmt: SQLHSTMT,
        szSqlStr: *mut SQLWCHAR,
        cbSqlStr: SQLINTEGER,
    ) -> SQLRETURN;
}
extern "C" {
    pub fn SQLGetConnectAttrW(
        hdbc: SQLHDBC,
        fAttribute: SQLINTEGER,
        rgbValue: SQLPOINTER,
        cbValueMax: SQLINTEGER,
        pcbValue: *mut SQLINTEGER,
    ) -> SQLRETURN;
}
extern "C" {
    pub fn SQLGetCursorNameW(
        hstmt: SQLHSTMT,
        szCursor: *mut SQLWCHAR,
        cbCursorMax: SQLSMALLINT,
        pcbCursor: *mut SQLSMALLINT,
    ) -> SQLRETURN;
}
extern "C" {
    pub fn SQLSetDescFieldW(
        DescriptorHandle: SQLHDESC,
        RecNumber: SQLSMALLINT,
        FieldIdentifier: SQLSMALLINT,
        Value: SQLPOINTER,
        BufferLength: SQLINTEGER,
    ) -> SQLRETURN;
}
extern "C" {
    pub fn SQLGetDescFieldW(
        hdesc: SQLHDESC,
        iRecord: SQLSMALLINT,
        iField: SQLSMALLINT,
        rgbValue: SQLPOINTER,
        cbValueMax: SQLINTEGER,
        pcbValue: *mut SQLINTEGER,
    ) -> SQLRETURN;
}
extern "C" {
    pub fn SQLGetDescRecW(
        hdesc: SQLHDESC,
        iRecord: SQLSMALLINT,
        szName: *mut SQLWCHAR,
        cbNameMax: SQLSMALLINT,
        pcbName: *mut SQLSMALLINT,
        pfType: *mut SQLSMALLINT,
        pfSubType: *mut SQLSMALLINT,
        pLength: *mut SQLINTEGER,
        pPrecision: *mut SQLSMALLINT,
        pScale: *mut SQLSMALLINT,
        pNullable: *mut SQLSMALLINT,
    ) -> SQLRETURN;
}
extern "C" {
    pub fn SQLGetDiagFieldW(
        fHandleType: SQLSMALLINT,
        handle: SQLHANDLE,
        iRecord: SQLSMALLINT,
        fDiagField: SQLSMALLINT,
        rgbDiagInfo: SQLPOINTER,
        cbDiagInfoMax: SQLSMALLINT,
        pcbDiagInfo: *mut SQLSMALLINT,
    ) -> SQLRETURN;
}
extern "C" {
    pub fn SQLGetDiagRecW(
        fHandleType: SQLSMALLINT,
        handle: SQLHANDLE,
        iRecord: SQLSMALLINT,
        szSqlState: *mut SQLWCHAR,
        pfNativeError: *mut SQLINTEGER,
        szErrorMsg: *mut SQLWCHAR,
        cbErrorMsgMax: SQLSMALLINT,
        pcbErrorMsg: *mut SQLSMALLINT,
    ) -> SQLRETURN;
}
extern "C" {
    pub fn SQLGetEnvAttrW(
        hEnv: SQLHENV,
        fAttribute: SQLINTEGER,
        pParam: SQLPOINTER,
        cbParamMax: SQLINTEGER,
        pcbParam: *mut SQLINTEGER,
    ) -> SQLRETURN;
}
extern "C" {
    pub fn SQLPrepareW(hstmt: SQLHSTMT, szSqlStr: *mut SQLWCHAR, cbSqlStr: SQLINTEGER)
        -> SQLRETURN;
}
extern "C" {
    pub fn SQLExtendedPrepareW(
        hStmt: SQLHSTMT,
        pszSqlStrIn: *mut SQLWCHAR,
        cbSqlStr: SQLINTEGER,
        cPars: SQLINTEGER,
        sStmtType: SQLSMALLINT,
        cStmtAttrs: SQLINTEGER,
        piStmtAttr: *mut SQLINTEGER,
        pvParams: *mut SQLINTEGER,
    ) -> SQLRETURN;
}
extern "C" {
    pub fn SQLSetConnectAttrW(
        hdbc: SQLHDBC,
        fAttribute: SQLINTEGER,
        rgbValue: SQLPOINTER,
        cbValue: SQLINTEGER,
    ) -> SQLRETURN;
}
extern "C" {
    pub fn SQLSetCursorNameW(
        hstmt: SQLHSTMT,
        szCursor: *mut SQLWCHAR,
        cbCursor: SQLSMALLINT,
    ) -> SQLRETURN;
}
extern "C" {
    pub fn SQLSetEnvAttrW(
        hEnv: SQLHENV,
        fAttribute: SQLINTEGER,
        pParam: SQLPOINTER,
        cbParam: SQLINTEGER,
    ) -> SQLRETURN;
}
extern "C" {
    pub fn SQLColumnsW(
        hstmt: SQLHSTMT,
        szCatalogName: *mut SQLWCHAR,
        cbCatalogName: SQLSMALLINT,
        szSchemaName: *mut SQLWCHAR,
        cbSchemaName: SQLSMALLINT,
        szTableName: *mut SQLWCHAR,
        cbTableName: SQLSMALLINT,
        szColumnName: *mut SQLWCHAR,
        cbColumnName: SQLSMALLINT,
    ) -> SQLRETURN;
}
extern "C" {
    pub fn SQLGetInfoW(
        hdbc: SQLHDBC,
        fInfoType: SQLUSMALLINT,
        rgbInfoValue: SQLPOINTER,
        cbInfoValueMax: SQLSMALLINT,
        pcbInfoValue: *mut SQLSMALLINT,
    ) -> SQLRETURN;
}
extern "C" {
    pub fn SQLGetConnectOptionW(
        hDbc: SQLHDBC,
        fOptionIn: SQLUSMALLINT,
        pvParam: SQLPOINTER,
    ) -> SQLRETURN;
}
extern "C" {
    pub fn SQLSetConnectOptionW(
        hDbc: SQLHDBC,
        fOptionIn: SQLUSMALLINT,
        vParam: SQLUINTEGER,
    ) -> SQLRETURN;
}
extern "C" {
    pub fn SQLGetTypeInfoW(hstmt: SQLHSTMT, fSqlType: SQLSMALLINT) -> SQLRETURN;
}
extern "C" {
    pub fn SQLSpecialColumnsW(
        hstmt: SQLHSTMT,
        fColType: SQLUSMALLINT,
        szCatalogName: *mut SQLWCHAR,
        cbCatalogName: SQLSMALLINT,
        szSchemaName: *mut SQLWCHAR,
        cbSchemaName: SQLSMALLINT,
        szTableName: *mut SQLWCHAR,
        cbTableName: SQLSMALLINT,
        fScope: SQLUSMALLINT,
        fNullable: SQLUSMALLINT,
    ) -> SQLRETURN;
}
extern "C" {
    pub fn SQLStatisticsW(
        hstmt: SQLHSTMT,
        szCatalogName: *mut SQLWCHAR,
        cbCatalogName: SQLSMALLINT,
        szSchemaName: *mut SQLWCHAR,
        cbSchemaName: SQLSMALLINT,
        szTableName: *mut SQLWCHAR,
        cbTableName: SQLSMALLINT,
        fUnique: SQLUSMALLINT,
        fAccuracy: SQLUSMALLINT,
    ) -> SQLRETURN;
}
extern "C" {
    pub fn SQLTablesW(
        hstmt: SQLHSTMT,
        szCatalogName: *mut SQLWCHAR,
        cbCatalogName: SQLSMALLINT,
        szSchemaName: *mut SQLWCHAR,
        cbSchemaName: SQLSMALLINT,
        szTableName: *mut SQLWCHAR,
        cbTableName: SQLSMALLINT,
        szTableType: *mut SQLWCHAR,
        cbTableType: SQLSMALLINT,
    ) -> SQLRETURN;
}
extern "C" {
    pub fn SQLDataSourcesW(
        henv: SQLHENV,
        fDirection: SQLUSMALLINT,
        szDSN: *mut SQLWCHAR,
        cbDSNMax: SQLSMALLINT,
        pcbDSN: *mut SQLSMALLINT,
        szDescription: *mut SQLWCHAR,
        cbDescriptionMax: SQLSMALLINT,
        pcbDescription: *mut SQLSMALLINT,
    ) -> SQLRETURN;
}
extern "C" {
    pub fn SQLDriverConnectW(
        hdbc: SQLHDBC,
        hwnd: SQLHWND,
        szConnStrIn: *mut SQLWCHAR,
        cbConnStrIn: SQLSMALLINT,
        szConnStrOut: *mut SQLWCHAR,
        cbConnStrOutMax: SQLSMALLINT,
        pcbConnStrOut: *mut SQLSMALLINT,
        fDriverCompletion: SQLUSMALLINT,
    ) -> SQLRETURN;
}
extern "C" {
    pub fn SQLBrowseConnectW(
        hdbc: SQLHDBC,
        szConnStrIn: *mut SQLWCHAR,
        cbConnStrIn: SQLSMALLINT,
        szConnStrOut: *mut SQLWCHAR,
        cbConnStrOutMax: SQLSMALLINT,
        pcbConnStrOut: *mut SQLSMALLINT,
    ) -> SQLRETURN;
}
extern "C" {
    pub fn SQLColumnPrivilegesW(
        hstmt: SQLHSTMT,
        szCatalogName: *mut SQLWCHAR,
        cbCatalogName: SQLSMALLINT,
        szSchemaName: *mut SQLWCHAR,
        cbSchemaName: SQLSMALLINT,
        szTableName: *mut SQLWCHAR,
        cbTableName: SQLSMALLINT,
        szColumnName: *mut SQLWCHAR,
        cbColumnName: SQLSMALLINT,
    ) -> SQLRETURN;
}
extern "C" {
    pub fn SQLGetStmtAttrW(
        hstmt: SQLHSTMT,
        fAttribute: SQLINTEGER,
        rgbValue: SQLPOINTER,
        cbValueMax: SQLINTEGER,
        pcbValue: *mut SQLINTEGER,
    ) -> SQLRETURN;
}
extern "C" {
    pub fn SQLSetStmtAttrW(
        hstmt: SQLHSTMT,
        fAttribute: SQLINTEGER,
        rgbValue: SQLPOINTER,
        cbValueMax: SQLINTEGER,
    ) -> SQLRETURN;
}
extern "C" {
    pub fn SQLForeignKeysW(
        hstmt: SQLHSTMT,
        szPkCatalogName: *mut SQLWCHAR,
        cbPkCatalogName: SQLSMALLINT,
        szPkSchemaName: *mut SQLWCHAR,
        cbPkSchemaName: SQLSMALLINT,
        szPkTableName: *mut SQLWCHAR,
        cbPkTableName: SQLSMALLINT,
        szFkCatalogName: *mut SQLWCHAR,
        cbFkCatalogName: SQLSMALLINT,
        szFkSchemaName: *mut SQLWCHAR,
        cbFkSchemaName: SQLSMALLINT,
        szFkTableName: *mut SQLWCHAR,
        cbFkTableName: SQLSMALLINT,
    ) -> SQLRETURN;
}
extern "C" {
    pub fn SQLNativeSqlW(
        hdbc: SQLHDBC,
        szSqlStrIn: *mut SQLWCHAR,
        cbSqlStrIn: SQLINTEGER,
        szSqlStr: *mut SQLWCHAR,
        cbSqlStrMax: SQLINTEGER,
        pcbSqlStr: *mut SQLINTEGER,
    ) -> SQLRETURN;
}
extern "C" {
    pub fn SQLPrimaryKeysW(
        hstmt: SQLHSTMT,
        szCatalogName: *mut SQLWCHAR,
        cbCatalogName: SQLSMALLINT,
        szSchemaName: *mut SQLWCHAR,
        cbSchemaName: SQLSMALLINT,
        szTableName: *mut SQLWCHAR,
        cbTableName: SQLSMALLINT,
    ) -> SQLRETURN;
}
extern "C" {
    pub fn SQLProcedureColumnsW(
        hstmt: SQLHSTMT,
        szCatalogName: *mut SQLWCHAR,
        cbCatalogName: SQLSMALLINT,
        szSchemaName: *mut SQLWCHAR,
        cbSchemaName: SQLSMALLINT,
        szProcName: *mut SQLWCHAR,
        cbProcName: SQLSMALLINT,
        szColumnName: *mut SQLWCHAR,
        cbColumnName: SQLSMALLINT,
    ) -> SQLRETURN;
}
extern "C" {
    pub fn SQLProceduresW(
        hstmt: SQLHSTMT,
        szCatalogName: *mut SQLWCHAR,
        cbCatalogName: SQLSMALLINT,
        szSchemaName: *mut SQLWCHAR,
        cbSchemaName: SQLSMALLINT,
        szProcName: *mut SQLWCHAR,
        cbProcName: SQLSMALLINT,
    ) -> SQLRETURN;
}
extern "C" {
    pub fn SQLExtendedProcedureColumnsW(
        hstmt: SQLHSTMT,
        szCatalogName: *mut SQLWCHAR,
        cbCatalogName: SQLSMALLINT,
        szSchemaName: *mut SQLWCHAR,
        cbSchemaName: SQLSMALLINT,
        szProcName: *mut SQLWCHAR,
        cbProcName: SQLSMALLINT,
        szColumnName: *mut SQLWCHAR,
        cbColumnName: SQLSMALLINT,
        szModuleName: *mut SQLWCHAR,
        cbModuleName: SQLSMALLINT,
    ) -> SQLRETURN;
}
extern "C" {
    pub fn SQLExtendedProceduresW(
        hstmt: SQLHSTMT,
        szCatalogName: *mut SQLWCHAR,
        cbCatalogName: SQLSMALLINT,
        szSchemaName: *mut SQLWCHAR,
        cbSchemaName: SQLSMALLINT,
        szProcName: *mut SQLWCHAR,
        cbProcName: SQLSMALLINT,
        szModuleName: *mut SQLWCHAR,
        cbModuleName: SQLSMALLINT,
    ) -> SQLRETURN;
}
extern "C" {
    pub fn SQLTablePrivilegesW(
        hstmt: SQLHSTMT,
        szCatalogName: *mut SQLWCHAR,
        cbCatalogName: SQLSMALLINT,
        szSchemaName: *mut SQLWCHAR,
        cbSchemaName: SQLSMALLINT,
        szTableName: *mut SQLWCHAR,
        cbTableName: SQLSMALLINT,
    ) -> SQLRETURN;
}
extern "C" {
    pub fn SQLCreateDbW(
        hDbc: SQLHDBC,
        pszDBW: *mut SQLWCHAR,
        cbDB: SQLINTEGER,
        pszCodeSetW: *mut SQLWCHAR,
        cbCodeSet: SQLINTEGER,
        pszModeW: *mut SQLWCHAR,
        cbMode: SQLINTEGER,
    ) -> SQLRETURN;
}
extern "C" {
    pub fn SQLDropDbW(hDbc: SQLHDBC, pszDBW: *mut SQLWCHAR, cbDB: SQLINTEGER) -> SQLRETURN;
}
extern "C" {
    pub fn SQLCreatePkgW(
        hDbc: SQLHDBC,
        szBindFileNameIn: *mut SQLWCHAR,
        cbBindFileNameIn: SQLINTEGER,
        szBindOpts: *mut SQLWCHAR,
        cbBindOpts: SQLINTEGER,
    ) -> SQLRETURN;
}
extern "C" {
    pub fn SQLDropPkgW(
        hDbc: SQLHDBC,
        szCollection: *mut SQLWCHAR,
        cbCollection: SQLINTEGER,
        szPackage: *mut SQLWCHAR,
        cbPackage: SQLINTEGER,
    ) -> SQLRETURN;
}
extern "C" {
    pub fn SQLBindFileToCol(
        hstmt: SQLHSTMT,
        icol: SQLUSMALLINT,
        FileName: *mut SQLCHAR,
        FileNameLength: *mut SQLSMALLINT,
        FileOptions: *mut SQLUINTEGER,
        MaxFileNameLength: SQLSMALLINT,
        StringLength: *mut SQLINTEGER,
        IndicatorValue: *mut SQLINTEGER,
    ) -> SQLRETURN;
}
extern "C" {
    pub fn SQLBindFileToParam(
        hstmt: SQLHSTMT,
        ipar: SQLUSMALLINT,
        fSqlType: SQLSMALLINT,
        FileName: *mut SQLCHAR,
        FileNameLength: *mut SQLSMALLINT,
        FileOptions: *mut SQLUINTEGER,
        MaxFileNameLength: SQLSMALLINT,
        IndicatorValue: *mut SQLINTEGER,
    ) -> SQLRETURN;
}
extern "C" {
    pub fn SQLGetLength(
        hstmt: SQLHSTMT,
        LocatorCType: SQLSMALLINT,
        Locator: SQLINTEGER,
        StringLength: *mut SQLINTEGER,
        IndicatorValue: *mut SQLINTEGER,
    ) -> SQLRETURN;
}
extern "C" {
    pub fn SQLGetPosition(
        hstmt: SQLHSTMT,
        LocatorCType: SQLSMALLINT,
        SourceLocator: SQLINTEGER,
        SearchLocator: SQLINTEGER,
        SearchLiteral: *mut SQLCHAR,
        SearchLiteralLength: SQLINTEGER,
        FromPosition: SQLUINTEGER,
        LocatedAt: *mut SQLUINTEGER,
        IndicatorValue: *mut SQLINTEGER,
    ) -> SQLRETURN;
}
extern "C" {
    pub fn SQLGetSQLCA(
        henv: SQLHENV,
        hdbc: SQLHDBC,
        hstmt: SQLHSTMT,
        pSqlca: *mut sqlca,
    ) -> SQLRETURN;
}
extern "C" {
    pub fn SQLGetSubString(
        hstmt: SQLHSTMT,
        LocatorCType: SQLSMALLINT,
        SourceLocator: SQLINTEGER,
        FromPosition: SQLUINTEGER,
        ForLength: SQLUINTEGER,
        TargetCType: SQLSMALLINT,
        rgbValue: SQLPOINTER,
        cbValueMax: SQLINTEGER,
        StringLength: *mut SQLINTEGER,
        IndicatorValue: *mut SQLINTEGER,
    ) -> SQLRETURN;
}
extern "C" {
    pub fn SQLSetColAttributes(
        hstmt: SQLHSTMT,
        icol: SQLUSMALLINT,
        pszColName: *mut SQLCHAR,
        cbColName: SQLSMALLINT,
        fSQLType: SQLSMALLINT,
        cbColDef: SQLUINTEGER,
        ibScale: SQLSMALLINT,
        fNullable: SQLSMALLINT,
    ) -> SQLRETURN;
}
extern "C" {
    pub fn SQLExtendedProcedures(
        hstmt: SQLHSTMT,
        szCatalogName: *mut SQLCHAR,
        cbCatalogName: SQLSMALLINT,
        szSchemaName: *mut SQLCHAR,
        cbSchemaName: SQLSMALLINT,
        szProcName: *mut SQLCHAR,
        cbProcName: SQLSMALLINT,
        szModuleName: *mut SQLCHAR,
        cbModuleName: SQLSMALLINT,
    ) -> SQLRETURN;
}
extern "C" {
    pub fn SQLExtendedProcedureColumns(
        hstmt: SQLHSTMT,
        szCatalogName: *mut SQLCHAR,
        cbCatalogName: SQLSMALLINT,
        szSchemaName: *mut SQLCHAR,
        cbSchemaName: SQLSMALLINT,
        szProcName: *mut SQLCHAR,
        cbProcName: SQLSMALLINT,
        szColumnName: *mut SQLCHAR,
        cbColumnName: SQLSMALLINT,
        szModuleName: *mut SQLCHAR,
        cbModuleName: SQLSMALLINT,
    ) -> SQLRETURN;
}
extern "C" {
    pub fn SQLReloadConfig(
        config_property: SQLINTEGER,
        DiagInfoString: *mut SQLCHAR,
        BufferLength: SQLSMALLINT,
        StringLengthPtr: *mut SQLSMALLINT,
    ) -> SQLRETURN;
}
extern "C" {
    pub fn SQLReloadConfigW(
        config_property: SQLINTEGER,
        DiagInfoString: *mut SQLWCHAR,
        BufferLength: SQLSMALLINT,
        StringLengthPtr: *mut SQLSMALLINT,
    ) -> SQLRETURN;
}
extern "C" {
    pub fn SQLGetPositionW(
        hStmt: SQLHSTMT,
        fCType: SQLSMALLINT,
        iLocatorIn: SQLINTEGER,
        iPatternLocator: SQLINTEGER,
        pszPatternLiteral: *mut SQLWCHAR,
        cbPatternLiteral: SQLINTEGER,
        iStartSearchAtIn: SQLUINTEGER,
        piLocatedAtIn: *mut SQLUINTEGER,
        piIndicatorValue: *mut SQLINTEGER,
    ) -> SQLRETURN;
}
extern "C" {
    pub fn SQLSetConnection(hdbc: SQLHDBC) -> SQLRETURN;
}
extern "C" {
    pub fn SQLGetEnvAttr(
        henv: SQLHENV,
        Attribute: SQLINTEGER,
        Value: SQLPOINTER,
        BufferLength: SQLINTEGER,
        StringLength: *mut SQLINTEGER,
    ) -> SQLRETURN;
}
extern "C" {
    pub fn SQLSetEnvAttr(
        henv: SQLHENV,
        Attribute: SQLINTEGER,
        Value: SQLPOINTER,
        StringLength: SQLINTEGER,
    ) -> SQLRETURN;
}
extern "C" {
    pub fn SQLBindParam(
        StatementHandle: SQLHSTMT,
        ParameterNumber: SQLUSMALLINT,
        ValueType: SQLSMALLINT,
        ParameterType: SQLSMALLINT,
        LengthPrecision: SQLUINTEGER,
        ParameterScale: SQLSMALLINT,
        ParameterValue: SQLPOINTER,
        StrLen_or_Ind: *mut SQLINTEGER,
    ) -> SQLRETURN;
}
extern "C" {
    pub fn SQLBuildDataLink(
        hStmt: SQLHSTMT,
        pszLinkType: *mut SQLCHAR,
        cbLinkType: SQLINTEGER,
        pszDataLocation: *mut SQLCHAR,
        cbDataLocation: SQLINTEGER,
        pszComment: *mut SQLCHAR,
        cbComment: SQLINTEGER,
        pDataLink: *mut SQLCHAR,
        cbDataLinkMax: SQLINTEGER,
        pcbDataLink: *mut SQLINTEGER,
    ) -> SQLRETURN;
}
extern "C" {
    pub fn SQLGetDataLinkAttr(
        hStmt: SQLHSTMT,
        fAttrType: SQLSMALLINT,
        pDataLink: *mut SQLCHAR,
        cbDataLink: SQLINTEGER,
        pAttribute: SQLPOINTER,
        cbAttributeMax: SQLINTEGER,
        pcbAttribute: *mut SQLINTEGER,
    ) -> SQLRETURN;
}
extern "C" {
    pub fn SQLExtendedPrepare(
        hstmt: SQLHSTMT,
        pszSqlStmt: *mut SQLCHAR,
        cbSqlStmt: SQLINTEGER,
        cPars: SQLINTEGER,
        sStmtType: SQLSMALLINT,
        cStmtAttrs: SQLINTEGER,
        piStmtAttr: *mut SQLINTEGER,
        pvParams: *mut SQLINTEGER,
    ) -> SQLRETURN;
}
extern "C" {
    pub fn SQLExtendedBind(
        hstmt: SQLHSTMT,
        fBindCol: SQLSMALLINT,
        cRecords: SQLSMALLINT,
        pfCType: *mut SQLSMALLINT,
        rgbValue: *mut SQLPOINTER,
        cbValueMax: *mut SQLINTEGER,
        puiPrecisionCType: *mut SQLUINTEGER,
        psScaleCType: *mut SQLSMALLINT,
        pcbValue: *mut *mut SQLINTEGER,
        piIndicatorPtr: *mut *mut SQLINTEGER,
        pfParamType: *mut SQLSMALLINT,
        pfSQLType: *mut SQLSMALLINT,
        pcbColDef: *mut SQLUINTEGER,
        pibScale: *mut SQLSMALLINT,
    ) -> SQLRETURN;
}
extern "C" {
    pub fn SQLExtendedDescribe(
        hStmt: SQLHANDLE,
        fDescribeCol: SQLSMALLINT,
        iNumRecordsAllocated: SQLUSMALLINT,
        pusNumRecords: *mut SQLUSMALLINT,
        pNames: *mut SQLCHAR,
        sNameMaxByteLen: SQLSMALLINT,
        psNameCharLen: *mut SQLSMALLINT,
        psSQLType: *mut SQLSMALLINT,
        pcbColDef: *mut SQLUINTEGER,
        pcbDisplaySize: *mut SQLUINTEGER,
        psScale: *mut SQLSMALLINT,
        psNullable: *mut SQLSMALLINT,
        psParamType: *mut SQLSMALLINT,
        piCardinality: *mut SQLINTEGER,
    ) -> SQLRETURN;
}
extern "C" {
    pub fn SQLDropPkg(
        hDbc: SQLHDBC,
        szCollection: *mut SQLCHAR,
        cbCollection: SQLINTEGER,
        szPackage: *mut SQLCHAR,
        cbPackage: SQLINTEGER,
    ) -> SQLRETURN;
}
pub type __builtin_va_list = [__va_list_tag; 1usize];
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __va_list_tag {
    pub gp_offset: ::std::os::raw::c_uint,
    pub fp_offset: ::std::os::raw::c_uint,
    pub overflow_arg_area: *mut ::std::os::raw::c_void,
    pub reg_save_area: *mut ::std::os::raw::c_void,
}
#[test]
fn bindgen_test_layout___va_list_tag() {
    assert_eq!(
        ::std::mem::size_of::<__va_list_tag>(),
        24usize,
        concat!("Size of: ", stringify!(__va_list_tag))
    );
    assert_eq!(
        ::std::mem::align_of::<__va_list_tag>(),
        8usize,
        concat!("Alignment of ", stringify!(__va_list_tag))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<__va_list_tag>())).gp_offset as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(__va_list_tag),
            "::",
            stringify!(gp_offset)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<__va_list_tag>())).fp_offset as *const _ as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(__va_list_tag),
            "::",
            stringify!(fp_offset)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<__va_list_tag>())).overflow_arg_area as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(__va_list_tag),
            "::",
            stringify!(overflow_arg_area)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<__va_list_tag>())).reg_save_area as *const _ as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(__va_list_tag),
            "::",
            stringify!(reg_save_area)
        )
    );
}
