//! Traits extensions that makes working with `Orders` API more convenient.

use seed::prelude::{CmdHandle, StreamHandle};
use std::future::Future;

pub use seed::prelude::{cmds, streams, Orders};

/// Provides convenient to interact with Seed runtime
pub trait OrdersExt<Ms: 'static>: Orders<Ms> {
    /// run `handler` closure every `ms` millisecond
    fn send_every<MsU: 'static>(
        &mut self,
        ms: u32,
        handler: impl FnOnce() -> MsU + Clone + 'static,
    ) -> &mut Self {
        self.stream(streams::interval(ms, handler))
    }

    /// same as `send_every` but return handler to control it's lifetime
    fn send_every_with_handle<MsU: 'static>(
        &mut self,
        ms: u32,
        handler: impl FnOnce() -> MsU + Clone + 'static,
    ) -> StreamHandle {
        self.stream_with_handle(streams::interval(ms, handler))
    }

    /// same as `send_msg` but shorter
    fn send(&mut self, msg: Ms) -> &mut Self {
        self.send_msg(msg)
    }

    /// same as `perform_cmd` but shorter
    fn cmd<MsU: 'static>(&mut self, cmd: impl Future<Output = MsU> + 'static) -> &mut Self {
        self.perform_cmd(cmd)
    }

    /// same as `perform_cmd_with_handle` but shorter
    fn cmd_with_handle<MsU: 'static>(
        &mut self,
        cmd: impl Future<Output = MsU> + 'static,
    ) -> CmdHandle {
        self.perform_cmd_with_handle(cmd)
    }

    /// run `handler` after `ms` millisecond
    fn send_after<MsU: 'static>(
        &mut self,
        ms: u32,
        handler: impl FnOnce() -> MsU + Clone + 'static,
    ) -> &mut Self {
        self.cmd(cmds::timeout(ms, handler))
    }

    /// same as `send_after` but reutrns handler to control it's lifetime
    fn send_after_with_handle<MsU: 'static>(
        &mut self,
        ms: u32,
        handler: impl FnOnce() -> MsU + Clone + 'static,
    ) -> CmdHandle {
        self.cmd_with_handle(cmds::timeout(ms, handler))
    }
}

impl<T, Ms: 'static> OrdersExt<Ms> for T where T: Orders<Ms> {}
