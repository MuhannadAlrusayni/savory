use crate::prelude::*;
use seed::prelude::{cmds, CmdHandle};

pub trait OrdersExt<Ms: 'static>: Orders<Ms> {
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

    fn proxy_with<ChildMs: 'static>(
        &mut self,
        map: &MsgMapper<ChildMs, Ms>,
    ) -> seed::app::OrdersProxy<ChildMs, Self::AppMs, Self::Mdl, Self::INodes> {
        self.proxy(map.map_msg_once())
    }
}

impl<T, Ms: 'static> OrdersExt<Ms> for T where T: Orders<Ms> {}
