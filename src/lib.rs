//! A simple library for warning callbacks.

#![warn(missing_docs)]

#[macro_use] extern crate log;

use std::any::Any;
use std::fmt;
use std::marker::PhantomData;
use std::mem;

/// Trait for objects that can accept warnings.
pub trait Warn<W> {
    /// This method is the receiver of the warnings.
    fn warn(&mut self, warning: W);
}

impl<W> Warn<W> for Vec<W> {
    fn warn(&mut self, warning: W) {
        self.push(warning);
    }
}

/// Struct that will ignore all the warnings it gets passed.
#[derive(Clone, Copy, Debug, Eq, Ord, Hash, PartialEq, PartialOrd)]
pub struct Ignore;

impl<W> Warn<W> for Ignore {
    fn warn(&mut self, warning: W) {
        let _ = warning;
    }
}

/// Struct that will panic on any warning it encounters.
///
/// This should probably only be used within tests.
#[derive(Clone, Copy, Debug, Eq, Ord, Hash, PartialEq, PartialOrd)]
pub struct Panic;

impl<W: Any+fmt::Debug+Send> Warn<W> for Panic {
    fn warn(&mut self, warning: W) {
        panic!("{:?}", warning);
    }
}

/// Struct that logs each warning it encounters.
///
/// Logging is done via the `log` crate.
#[derive(Clone, Copy, Debug, Eq, Ord, Hash, PartialEq, PartialOrd)]
pub struct Log;

impl<W: fmt::Debug> Warn<W> for Log {
    fn warn(&mut self, warning: W) {
        warn!("{:?}", warning);
    }
}

/// Helper struct for the `wrap` function.
pub struct Wrap<WT, W: Warn<WT>> {
    warn: W,
    phantom: PhantomData<WT>,
}

/// Wraps a `Warn` struct so it can receive more warning types.
pub fn wrap<WT, W: Warn<WT>>(warn: &mut W) -> &mut Wrap<WT, W> {
    unsafe {
        mem::transmute(warn)
    }
}

impl<WT, WF: Into<WT>, W: Warn<WT>> Warn<WF> for Wrap<WT, W> {
    fn warn(&mut self, warning: WF) {
        self.warn.warn(warning.into());
    }
}

#[cfg(test)]
mod test {
    use super::Ignore;
    use super::Log;
    use super::Panic;
    use super::Warn;

    const WARNING: &'static str = "unique_string";

    #[test]
    #[should_panic="unique_string"]
    fn panic() {
        Panic.warn(WARNING);
    }

    #[test]
    fn ignore() {
        Ignore.warn(WARNING);
    }

    #[test]
    fn vec() {
        let mut vec = vec![];
        vec.warn(WARNING);
        assert_eq!(vec, [WARNING]);
    }

    #[test]
    fn log() {
        Log.warn(WARNING);
    }
}
