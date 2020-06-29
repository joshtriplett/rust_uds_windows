//! Unix domain sockets for Windows

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
pub use crate::stdnet::{
    from_path, AcceptAddrs, AcceptAddrsBuf, SocketAddr, UnixListener, UnixListenerExt, UnixStream,
    UnixStreamExt,
};
