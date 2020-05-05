use crate::prelude::*;
use seed::prelude::{cmds, streams, CmdHandle, StreamHandle};
use std::future::Future;

pub trait OrdersExt<Ms: 'static>: Orders<Ms> {
    fn stream_every<MsU: 'static>(
        &mut self,
        ms: u32,
        handler: impl FnOnce() -> MsU + Clone + 'static,
    ) -> &mut Self {
        self.stream(streams::interval(ms, handler))
    }

    fn stream_every_with_handle<MsU: 'static>(
        &mut self,
        ms: u32,
        handler: impl FnOnce() -> MsU + Clone + 'static,
    ) -> StreamHandle {
        self.stream_with_handle(streams::interval(ms, handler))
    }

    fn send(&mut self, msg: Ms) -> &mut Self {
        self.send_msg(msg)
    }

    fn cmd<MsU: 'static>(&mut self, cmd: impl Future<Output = MsU> + 'static) -> &mut Self {
        self.perform_cmd(cmd)
    }

    fn cmd_with_handle<MsU: 'static>(
        &mut self,
        cmd: impl Future<Output = MsU> + 'static,
    ) -> CmdHandle {
        self.perform_cmd_with_handle(cmd)
    }

    fn send_after<MsU: 'static>(
        &mut self,
        ms: u32,
        handler: impl FnOnce() -> MsU + Clone + 'static,
    ) -> &mut Self {
        self.cmd(cmds::timeout(ms, handler))
    }

    fn send_after_with_handle<MsU: 'static>(
        &mut self,
        ms: u32,
        handler: impl FnOnce() -> MsU + Clone + 'static,
    ) -> CmdHandle {
        self.cmd_with_handle(cmds::timeout(ms, handler))
    }

    fn proxy_with<ChildMs: 'static>(
        &mut self,
        map: &MsgMapper<ChildMs, Ms>,
    ) -> seed::app::OrdersProxy<ChildMs, Self::AppMs, Self::Mdl, Self::INodes> {
        self.proxy(map.map_msg_once())
    }
}

impl<T, Ms: 'static> OrdersExt<Ms> for T where T: Orders<Ms> {}
