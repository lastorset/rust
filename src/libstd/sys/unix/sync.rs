// Copyright 2014-2015 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

#![allow(bad_style)]

use libc;

pub use self::os::{PTHREAD_MUTEX_INITIALIZER, PTHREAD_MUTEX_RECURSIVE, pthread_mutex_t,
                   pthread_mutexattr_t};
pub use self::os::{PTHREAD_COND_INITIALIZER, pthread_cond_t};
pub use self::os::{PTHREAD_RWLOCK_INITIALIZER, pthread_rwlock_t};

extern {
    // mutexes
    pub fn pthread_mutex_init(lock: *mut pthread_mutex_t, attr: *const pthread_mutexattr_t)
                            -> libc::c_int;
    pub fn pthread_mutex_destroy(lock: *mut pthread_mutex_t) -> libc::c_int;
    pub fn pthread_mutex_lock(lock: *mut pthread_mutex_t) -> libc::c_int;
    pub fn pthread_mutex_trylock(lock: *mut pthread_mutex_t) -> libc::c_int;
    pub fn pthread_mutex_unlock(lock: *mut pthread_mutex_t) -> libc::c_int;

    pub fn pthread_mutexattr_init(attr: *mut pthread_mutexattr_t) -> libc::c_int;
    pub fn pthread_mutexattr_destroy(attr: *mut pthread_mutexattr_t) -> libc::c_int;
    pub fn pthread_mutexattr_settype(attr: *mut pthread_mutexattr_t, _type: libc::c_int)
                                    -> libc::c_int;

    // cvars
    pub fn pthread_cond_wait(cond: *mut pthread_cond_t,
                             lock: *mut pthread_mutex_t) -> libc::c_int;
    pub fn pthread_cond_timedwait(cond: *mut pthread_cond_t,
                              lock: *mut pthread_mutex_t,
                              abstime: *const libc::timespec) -> libc::c_int;
    pub fn pthread_cond_signal(cond: *mut pthread_cond_t) -> libc::c_int;
    pub fn pthread_cond_broadcast(cond: *mut pthread_cond_t) -> libc::c_int;
    pub fn pthread_cond_destroy(cond: *mut pthread_cond_t) -> libc::c_int;
    pub fn gettimeofday(tp: *mut libc::timeval,
                        tz: *mut libc::c_void) -> libc::c_int;

    // rwlocks
    pub fn pthread_rwlock_destroy(lock: *mut pthread_rwlock_t) -> libc::c_int;
    pub fn pthread_rwlock_rdlock(lock: *mut pthread_rwlock_t) -> libc::c_int;
    pub fn pthread_rwlock_tryrdlock(lock: *mut pthread_rwlock_t) -> libc::c_int;
    pub fn pthread_rwlock_wrlock(lock: *mut pthread_rwlock_t) -> libc::c_int;
    pub fn pthread_rwlock_trywrlock(lock: *mut pthread_rwlock_t) -> libc::c_int;
    pub fn pthread_rwlock_unlock(lock: *mut pthread_rwlock_t) -> libc::c_int;
}

#[cfg(any(target_os = "freebsd",
          target_os = "dragonfly",
          target_os = "bitrig",
          target_os = "netbsd",
          target_os = "openbsd"))]
mod os {
    use libc;

    pub type pthread_mutex_t = *mut libc::c_void;
    pub type pthread_mutexattr_t = *mut libc::c_void;
    pub type pthread_cond_t = *mut libc::c_void;
    pub type pthread_rwlock_t = *mut libc::c_void;

    pub const PTHREAD_MUTEX_INITIALIZER: pthread_mutex_t = 0 as *mut _;
    pub const PTHREAD_COND_INITIALIZER: pthread_cond_t = 0 as *mut _;
    pub const PTHREAD_RWLOCK_INITIALIZER: pthread_rwlock_t = 0 as *mut _;
    pub const PTHREAD_MUTEX_RECURSIVE: libc::c_int = 2;
}

#[cfg(any(target_os = "macos", target_os = "ios"))]
mod os {
    use libc;

    #[cfg(any(target_arch = "x86_64",
              target_arch = "aarch64"))]
    const __PTHREAD_MUTEX_SIZE__: usize = 56;
    #[cfg(any(target_arch = "x86",
              target_arch = "arm"))]
    const __PTHREAD_MUTEX_SIZE__: usize = 40;

    #[cfg(any(target_arch = "x86_64",
              target_arch = "aarch64"))]
    const __PTHREAD_COND_SIZE__: usize = 40;
    #[cfg(any(target_arch = "x86",
              target_arch = "arm"))]
    const __PTHREAD_COND_SIZE__: usize = 24;

    #[cfg(any(target_arch = "x86_64",
              target_arch = "aarch64"))]
    const __PTHREAD_RWLOCK_SIZE__: usize = 192;
    #[cfg(any(target_arch = "x86",
              target_arch = "arm"))]
    const __PTHREAD_RWLOCK_SIZE__: usize = 124;

    const _PTHREAD_MUTEX_SIG_INIT: libc::c_long = 0x32AAABA7;
    const _PTHREAD_COND_SIG_INIT: libc::c_long = 0x3CB0B1BB;
    const _PTHREAD_RWLOCK_SIG_INIT: libc::c_long = 0x2DA8B3B4;

    #[repr(C)]
    pub struct pthread_mutex_t {
        __sig: libc::c_long,
        __opaque: [u8; __PTHREAD_MUTEX_SIZE__],
    }
    #[repr(C)]
    pub struct pthread_mutexattr_t {
        __sig: libc::c_long,
        // note, that this is 16 bytes just to be safe, the actual struct might be smaller.
        __opaque: [u8; 16],
    }
    #[repr(C)]
    pub struct pthread_cond_t {
        __sig: libc::c_long,
        __opaque: [u8; __PTHREAD_COND_SIZE__],
    }
    #[repr(C)]
    pub struct pthread_rwlock_t {
        __sig: libc::c_long,
        __opaque: [u8; __PTHREAD_RWLOCK_SIZE__],
    }

    pub const PTHREAD_MUTEX_INITIALIZER: pthread_mutex_t = pthread_mutex_t {
        __sig: _PTHREAD_MUTEX_SIG_INIT,
        __opaque: [0; __PTHREAD_MUTEX_SIZE__],
    };
    pub const PTHREAD_COND_INITIALIZER: pthread_cond_t = pthread_cond_t {
        __sig: _PTHREAD_COND_SIG_INIT,
        __opaque: [0; __PTHREAD_COND_SIZE__],
    };
    pub const PTHREAD_RWLOCK_INITIALIZER: pthread_rwlock_t = pthread_rwlock_t {
        __sig: _PTHREAD_RWLOCK_SIG_INIT,
        __opaque: [0; __PTHREAD_RWLOCK_SIZE__],
    };

    pub const PTHREAD_MUTEX_RECURSIVE: libc::c_int = 2;
}

#[cfg(target_os = "linux")]
mod os {
    use libc;

    // minus 8 because we have an 'align' field
    #[cfg(target_arch = "x86_64")]
    const __SIZEOF_PTHREAD_MUTEX_T: usize = 40 - 8;
    #[cfg(any(target_arch = "x86",
              target_arch = "arm",
              target_arch = "mips",
              target_arch = "mipsel",
              target_arch = "powerpc"))]
    const __SIZEOF_PTHREAD_MUTEX_T: usize = 24 - 8;
    #[cfg(target_arch = "aarch64")]
    const __SIZEOF_PTHREAD_MUTEX_T: usize = 48 - 8;

    #[cfg(any(target_arch = "x86_64",
              target_arch = "x86",
              target_arch = "arm",
              target_arch = "aarch64",
              target_arch = "mips",
              target_arch = "mipsel",
              target_arch = "powerpc"))]
    const __SIZEOF_PTHREAD_COND_T: usize = 48 - 8;

    #[cfg(any(target_arch = "x86_64",
              target_arch = "aarch64"))]
    const __SIZEOF_PTHREAD_RWLOCK_T: usize = 56 - 8;

    #[cfg(any(target_arch = "x86",
              target_arch = "arm",
              target_arch = "mips",
              target_arch = "mipsel",
              target_arch = "powerpc"))]
    const __SIZEOF_PTHREAD_RWLOCK_T: usize = 32 - 8;

    #[repr(C)]
    pub struct pthread_mutex_t {
        __align: libc::c_longlong,
        size: [u8; __SIZEOF_PTHREAD_MUTEX_T],
    }
    #[repr(C)]
    pub struct pthread_mutexattr_t {
        __align: libc::c_longlong,
        // note, that this is 16 bytes just to be safe, the actual struct might be smaller.
        size: [u8; 16],
    }
    #[repr(C)]
    pub struct pthread_cond_t {
        __align: libc::c_longlong,
        size: [u8; __SIZEOF_PTHREAD_COND_T],
    }
    #[repr(C)]
    pub struct pthread_rwlock_t {
        __align: libc::c_longlong,
        size: [u8; __SIZEOF_PTHREAD_RWLOCK_T],
    }

    pub const PTHREAD_MUTEX_INITIALIZER: pthread_mutex_t = pthread_mutex_t {
        __align: 0,
        size: [0; __SIZEOF_PTHREAD_MUTEX_T],
    };
    pub const PTHREAD_COND_INITIALIZER: pthread_cond_t = pthread_cond_t {
        __align: 0,
        size: [0; __SIZEOF_PTHREAD_COND_T],
    };
    pub const PTHREAD_RWLOCK_INITIALIZER: pthread_rwlock_t = pthread_rwlock_t {
        __align: 0,
        size: [0; __SIZEOF_PTHREAD_RWLOCK_T],
    };
    pub const PTHREAD_MUTEX_RECURSIVE: libc::c_int = 1;
}
#[cfg(target_os = "android")]
mod os {
    use libc;

    #[repr(C)]
    pub struct pthread_mutex_t { value: libc::c_int }
    pub type pthread_mutexattr_t = libc::c_long;
    #[repr(C)]
    pub struct pthread_cond_t { value: libc::c_int }
    #[repr(C)]
    pub struct pthread_rwlock_t {
        lock: pthread_mutex_t,
        cond: pthread_cond_t,
        numLocks: libc::c_int,
        writerThreadId: libc::c_int,
        pendingReaders: libc::c_int,
        pendingWriters: libc::c_int,
        reserved: [*mut libc::c_void; 4],
    }

    pub const PTHREAD_MUTEX_INITIALIZER: pthread_mutex_t = pthread_mutex_t {
        value: 0,
    };
    pub const PTHREAD_COND_INITIALIZER: pthread_cond_t = pthread_cond_t {
        value: 0,
    };
    pub const PTHREAD_RWLOCK_INITIALIZER: pthread_rwlock_t = pthread_rwlock_t {
        lock: PTHREAD_MUTEX_INITIALIZER,
        cond: PTHREAD_COND_INITIALIZER,
        numLocks: 0,
        writerThreadId: 0,
        pendingReaders: 0,
        pendingWriters: 0,
        reserved: [0 as *mut _; 4],
    };
    pub const PTHREAD_MUTEX_RECURSIVE: libc::c_int = 1;
}
