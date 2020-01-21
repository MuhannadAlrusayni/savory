use crate::render::Render;
use seed::prelude::*;

pub trait Model<Msg, GMsg>
where
    Msg: 'static,
    GMsg: 'static,
{
    fn update(&mut self, _: Msg, _: &mut impl Orders<Msg, GMsg>);
    fn sink(&mut self, _: GMsg, _: &mut impl Orders<Msg, GMsg>) {}
}
