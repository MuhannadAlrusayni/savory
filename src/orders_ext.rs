use crate::prelude::*;
use seed::prelude::{cmds, CmdHandle};

pub trait OrdersExt<Ms: 'static, GMs: 'static>: Orders<Ms, GMs> {
    fn perform_cmd_after<MsU: 'static>(
        &mut self,
        ms: u32,
        handler: impl FnOnce() -> MsU + Clone + 'static,
    ) -> &mut Self {
        self.perform_cmd(cmds::timeout(ms, handler))
    }

    fn perform_cmd_after_with_handle<MsU: 'static>(
        &mut self,
        ms: u32,
        handler: impl FnOnce() -> MsU + Clone + 'static,
    ) -> CmdHandle {
        self.perform_cmd_with_handle(cmds::timeout(ms, handler))
    }

    fn perform_g_cmd_after(
        &mut self,
        ms: u32,
        handler: impl FnOnce() -> GMs + Clone + 'static,
    ) -> &mut Self {
        self.perform_g_cmd(cmds::timeout(ms, handler))
    }

    fn perform_g_cmd_after_with_handle(
        &mut self,
        ms: u32,
        handler: impl FnOnce() -> GMs + Clone + 'static,
    ) -> CmdHandle {
        self.perform_g_cmd_with_handle(cmds::timeout(ms, handler))
    }

    fn proxy_with<ChildMs: 'static>(
        &mut self,
        map: &MsgMapper<ChildMs, Ms>,
        // f: impl FnOnce(ChildMs) -> Ms + 'static + Clone
    ) -> seed::app::OrdersProxy<ChildMs, Self::AppMs, Self::Mdl, Self::ElC, GMs> {
        self.proxy(map.map_msg_once())
    }
}

impl<T, Ms: 'static, GMs: 'static> OrdersExt<Ms, GMs> for T where T: Orders<Ms, GMs> {}
