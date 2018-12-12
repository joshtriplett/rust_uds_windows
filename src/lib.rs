//! Unix domain sockets for Windows

#![cfg_attr(test, deny(warnings))]

#[cfg(windows)]
extern crate winapi;

#[cfg(windows)]
extern crate ws2_32;

#[cfg(windows)]
extern crate kernel32;

#[cfg(windows)]
extern crate tempdir;

#[cfg(windows)]
mod stdnet;

#[cfg(windows)]
pub use stdnet::{
    from_path, AcceptAddrs, AcceptAddrsBuf, SocketAddr, UnixListener, UnixListenerExt, UnixStream,
    UnixStreamExt,
};
