//! Synchronization primitives.
//!
//! This module is an async version of [`std::sync`].
//!
//! [`std::sync`]: https://doc.rust-lang.org/std/sync/index.html
//!
//! # Examples
//!
//! Spawn a task that updates an integer protected by a mutex:
//!
//! ```
//! # #![feature(async_await)]
//! # fn main() { async_std::task::block_on(async {
//! #
//! use std::sync::Arc;
//!
//! use async_std::sync::Mutex;
//! use async_std::task;
//!
//! let m1 = Arc::new(Mutex::new(0));
//! let m2 = m1.clone();
//!
//! task::spawn(async move {
//!     *m2.lock().await = 1;
//! })
//! .await;
//!
//! assert_eq!(*m1.lock().await, 1);
//! #
//! # }) }
//! ```

pub use mutex::{Mutex, MutexGuard};
pub use rwlock::{RwLock, RwLockReadGuard, RwLockWriteGuard};

mod mutex;
mod rwlock;
