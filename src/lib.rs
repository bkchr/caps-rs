//! A pure-Rust library to work with Linux capabilities.
//!
//! It provides support for manipulating capabilities available
//! in modern Linux kernel. It supports traditional POSIX sets
//! (Effective, Inheritable, Permitted) as well as Linux-specific
//! Ambient and Bounding capabilities sets.
//!
//! ```rust
//! use caps::{Capability, CapSet};
//!
//! fn manipulate_caps() {
//!     if caps::has_cap(None, CapSet::Permitted, Capability::CAP_SYS_NICE).unwrap() {
//!         caps::drop(None, CapSet::Effective, Capability::CAP_SYS_NICE).unwrap();
//!         let s = caps::read(None, CapSet::Effective).unwrap();
//!         assert_eq!(s.contains(&Capability::CAP_SYS_NICE), false);
//!         caps::clear(None, CapSet::Effective).unwrap();
//!     };
//! }
//! ```

#![recursion_limit="500"]
extern crate libc;
#[macro_use]
extern crate error_chain;
#[macro_use]
extern crate custom_derive;
#[macro_use]
extern crate enum_derive;

// Error wrapping (public interface)
mod errors;
pub use errors::*;

// All kernel-related constants
mod nr;

// Implementation of POSIX sets
mod base;

// Implementation of Ambient set
mod ambient;

// Implementation of Bounding set
mod bounding;

/// Linux capabilities sets.
///
/// All capabilities sets supported by Linux, including standard
/// POSIX and custom ones. See `capabilities(7)`.
#[derive(Debug,Clone,Copy)]
pub enum CapSet {
    /// Ambient capabilities set (from Linux 4.3).
    Ambient,
    /// Bounding capabilities set (from Linux 2.6.25)
    Bounding,
    /// Effective capabilities set (from POSIX)
    Effective,
    /// Inheritable capabilities set (from POSIX)
    Inheritable,
    /// Permitted capabilities set (from POSIX)
    Permitted,
}

custom_derive! {
    /// Linux capabilities.
    ///
    /// All capabilities supported by Linux, including standard
    /// POSIX and custom ones. See `capabilities(7)`.
    #[derive(PartialEq,Eq,Hash,Debug,Clone,Copy,
             IterVariants(CapsVariants), IterVariantNames(CapsVariantNames))]
    #[allow(non_camel_case_types)]
    #[repr(u8)]
    pub enum Capability {
        /// CAP_CHOWN (from POSIX)
        CAP_CHOWN = nr::CAP_CHOWN,
        /// CAP_DAC_OVERRIDE (from POSIX)
        CAP_DAC_OVERRIDE = nr::CAP_DAC_OVERRIDE,
        /// CAP_DAC_READ_SEARCH (from POSIX)
        CAP_DAC_READ_SEARCH = nr::CAP_DAC_READ_SEARCH,
        /// CAP_FOWNER (from POSIX)
        CAP_FOWNER = nr::CAP_FOWNER,
        /// CAP_FSETID (from POSIX)
        CAP_FSETID = nr::CAP_FSETID,
        /// CAP_KILL (from POSIX)
        CAP_KILL = nr::CAP_KILL,
        /// CAP_SETGID (from POSIX)
        CAP_SETGID = nr::CAP_SETGID,
        /// CAP_SETUID (from POSIX)
        CAP_SETUID = nr::CAP_SETUID,
        /// CAP_SETPCAP (from Linux)
        CAP_SETPCAP = nr::CAP_SETPCAP,
        CAP_LINUX_IMMUTABLE = nr::CAP_LINUX_IMMUTABLE,
        CAP_NET_BIND_SERVICE = nr::CAP_NET_BIND_SERVICE,
        CAP_NET_BROADCAST = nr::CAP_NET_BROADCAST,
        CAP_NET_ADMIN = nr::CAP_NET_ADMIN,
        CAP_NET_RAW = nr::CAP_NET_RAW,
        CAP_IPC_LOCK = nr::CAP_IPC_LOCK,
        CAP_IPC_OWNER = nr::CAP_IPC_OWNER,
        /// CAP_SYS_MODULE (from Linux)
        CAP_SYS_MODULE = nr::CAP_SYS_MODULE,
        /// CAP_SYS_RAWIO (from Linux)
        CAP_SYS_RAWIO = nr::CAP_SYS_RAWIO,
        /// CAP_SYS_CHROOT (from Linux)
        CAP_SYS_CHROOT = nr::CAP_SYS_CHROOT,
        /// CAP_SYS_PTRACE (from Linux)
        CAP_SYS_PTRACE = nr::CAP_SYS_PTRACE,
        /// CAP_SYS_PACCT (from Linux)
        CAP_SYS_PACCT = nr::CAP_SYS_PACCT,
        /// CAP_SYS_ADMIN (from Linux)
        CAP_SYS_ADMIN = nr::CAP_SYS_ADMIN,
        /// CAP_SYS_BOOT (from Linux)
        CAP_SYS_BOOT = nr::CAP_SYS_BOOT,
        /// CAP_SYS_NICE (from Linux)
        CAP_SYS_NICE = nr::CAP_SYS_NICE,
        /// CAP_SYS_RESOURCE (from Linux)
        CAP_SYS_RESOURCE = nr::CAP_SYS_RESOURCE,
        /// CAP_SYS_TIME (from Linux)
        CAP_SYS_TIME = nr::CAP_SYS_TIME,
        /// CAP_SYS_TTY_CONFIG (from Linux)
        CAP_SYS_TTY_CONFIG = nr::CAP_SYS_TTY_CONFIG,
        /// CAP_SYS_MKNOD (from Linux 2.4)
        CAP_MKNOD = nr::CAP_MKNOD,
        /// CAP_LEASE (from Linux 2.4)
        CAP_LEASE = nr::CAP_LEASE,
        CAP_AUDIT_WRITE = nr::CAP_AUDIT_WRITE,
        /// CAP_AUDIT_CONTROL (from Linux 2.6.11)
        CAP_AUDIT_CONTROL = nr::CAP_AUDIT_CONTROL,
        CAP_SETFCAP = nr::CAP_SETFCAP,
        CAP_MAC_OVERRIDE = nr::CAP_MAC_OVERRIDE,
        CAP_MAC_ADMIN = nr::CAP_MAC_ADMIN,
        /// CAP_SYSLOG (from Linux 2.6.37)
        CAP_SYSLOG = nr::CAP_SYSLOG,
        // CAP_WAKE_ALARM (from Linux 3.0)
        CAP_WAKE_ALARM = nr::CAP_WAKE_ALARM,
        CAP_BLOCK_SUSPEND = nr::CAP_BLOCK_SUSPEND,
        /// CAP_AUDIT_READ (from Linux 3.16).
        CAP_AUDIT_READ = nr::CAP_AUDIT_READ,
    }
}

impl Capability {
    /// Returns the bitmask corresponding to this capability value.
    pub fn bitmask(&self) -> u64 {
        1u64 << (*self as u8)
    }
    /// Returns the index of this capability, i.e. its kernel-defined value.
    pub fn index(&self) -> u8 {
        (*self as u8)
    }
}

/// An `HashSet` specialized on `Capability`.
pub type CapsHashSet = std::collections::HashSet<Capability>;

/// Check if a thread contains a capability in a set.
///
/// Check if set `cset` for thread `tid` contains capability `cap`.
/// If `tid` is `None`, this operates on current thread (tid=0).
/// It cannot check Ambient or Bounding capabilities of other processes.
pub fn has_cap(tid: Option<i32>, cset: CapSet, cap: Capability) -> Result<bool> {
    let t = tid.unwrap_or(0);
    return match cset {
        CapSet::Ambient if t == 0 => ambient::has_cap(cap),
        CapSet::Bounding if t == 0 => bounding::has_cap(cap),
        CapSet::Effective | CapSet::Inheritable | CapSet::Permitted => base::has_cap(t, cset, cap),
        _ => bail!("operation not supported"),
    };
}

/// Return all capabilities in a set for a thread.
///
/// Return current content of set `cset` for thread `tid`.
/// If `tid` is `None`, this operates on current thread (tid=0).
/// It cannot read Ambient or Bounding capabilities of other processes.
pub fn read(tid: Option<i32>, cset: CapSet) -> Result<CapsHashSet> {
    let t = tid.unwrap_or(0);
    return match cset {
        CapSet::Ambient if t == 0 => ambient::read(),
        CapSet::Bounding if t == 0 => bounding::read(),
        CapSet::Effective | CapSet::Inheritable | CapSet::Permitted => base::read(t, cset),
        _ => bail!("operation not supported"),
    };

}

/// Set a capability set for a thread to a new value.
///
/// All and only capabilities in `value` will be set for set `cset` for thread `tid`.
/// If `tid` is `None`, this operates on current thread (tid=0).
/// It cannot manipulate Ambient set of other processes.
/// Capabilities cannot be set in Bounding set.
pub fn set(tid: Option<i32>, cset: CapSet, value: CapsHashSet) -> Result<()> {
    let t = tid.unwrap_or(0);
    return match cset {
        CapSet::Ambient if t == 0 => ambient::set(value),
        CapSet::Effective | CapSet::Inheritable | CapSet::Permitted => base::set(t, cset, value),
        _ => bail!("operation not supported"),
    };
}


/// Clear all capabilities in a set for a thread.
///
/// All capabilities will be cleared from set `cset` for thread `tid`.
/// If `tid` is `None`, this operates on current thread (tid=0).
/// It cannot manipulate Ambient or Bounding set of other processes.
pub fn clear(tid: Option<i32>, cset: CapSet) -> Result<()> {
    let t = tid.unwrap_or(0);
    return match cset {
        CapSet::Ambient if t == 0 => ambient::clear(),
        CapSet::Bounding if t == 0 => bounding::clear(),
        CapSet::Effective | CapSet::Permitted | CapSet::Inheritable => base::clear(t, cset),
        _ => bail!("operation not supported"),
    };
}

/// Raise a single capability in a set for a thread.
///
/// Capabilities `cap` will be raised from set `cset` of thread `tid`.
/// If `tid` is `None`, this operates on current thread (tid=0).
/// It cannot manipulate Ambient set of other processes.
/// Capabilities cannot be raised in Bounding set.
pub fn raise(tid: Option<i32>, cset: CapSet, cap: Capability) -> Result<()> {
    let t = tid.unwrap_or(0);
    return match cset {
        CapSet::Ambient if t == 0 => ambient::raise(cap),
        CapSet::Effective | CapSet::Permitted | CapSet::Inheritable => base::raise(t, cset, cap),
        _ => bail!("operation not supported"),
    };
}

/// Drop a single capability from a set for a thread.
///
/// Capabilities `cap` will be dropped from set `cset` of thread `tid`.
/// If `tid` is `None`, this operates on current thread (tid=0).
/// It cannot manipulate Ambient and Bounding sets of other processes.
pub fn drop(tid: Option<i32>, cset: CapSet, cap: Capability) -> Result<()> {
    let t = tid.unwrap_or(0);
    return match cset {
        CapSet::Ambient if t == 0 => ambient::drop(cap),
        CapSet::Bounding if t == 0 => bounding::drop(cap),
        CapSet::Effective | CapSet::Permitted | CapSet::Inheritable => base::drop(t, cset, cap),
        _ => bail!("operation not supported"),
    };
}
