//! Helper types used to map element message to parent message.
use crate::prelude::*;
use std::rc::Rc;

/// Helper trait used to make types implement `seed::prelude::MessageMaper` to
/// map thier messages using `MsgMapper` type
pub trait MessageMapperExt<Msg, OtherMsg>: MessageMapper<Msg, OtherMsg> {
    fn map_msg_with(self, map: &MsgMapper<Msg, OtherMsg>) -> Self::SelfWithOtherMs;
}

impl<T, Msg: 'static, OtherMsg: 'static> MessageMapperExt<Msg, OtherMsg> for T
where
    T: MessageMapper<Msg, OtherMsg>,
{
    fn map_msg_with(self, map: &MsgMapper<Msg, OtherMsg>) -> Self::SelfWithOtherMs {
        self.map_msg(map.map_msg_once().clone())
    }
}

/// Helper type used by stateful elements, it's jobe is to store a closure that
/// maps `Msg` to `OtherMsg`, stateful elements use it to map thier msg to thier
/// parent msg.
pub struct MsgMapper<Msg, OtherMsg>(Rc<dyn Fn(Msg) -> OtherMsg>);

impl<Msg: 'static, OtherMsg: 'static> MsgMapper<Msg, OtherMsg> {
    pub fn new(msg_mapper: impl Fn(Msg) -> OtherMsg + 'static) -> Self {
        Self(Rc::new(msg_mapper))
    }

    /// this method return a closure that is used in `update` and
    /// `render_with_style` methods.
    pub fn map_msg_once(&self) -> impl FnOnce(Msg) -> OtherMsg + 'static + Clone {
        let map = Rc::clone(&self.0);
        move |msg| (map)(msg)
    }
}

impl<T, Msg, OtherMsg> From<T> for MsgMapper<Msg, OtherMsg>
where
    T: Fn(Msg) -> OtherMsg + 'static,
{
    fn from(source: T) -> Self {
        Self(Rc::new(source))
    }
}

impl<Msg, OtherMsg> Clone for MsgMapper<Msg, OtherMsg> {
    fn clone(&self) -> Self {
        Self(Rc::clone(&self.0))
    }
}
