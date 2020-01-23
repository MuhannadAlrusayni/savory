use crate::render::Render;
use seed::prelude::*;

pub trait Model<Msg, PMsg, GMsg>: Render<PMsg>
where
    PMsg: 'static,
    GMsg: 'static,
{
    fn update(&mut self, _: Msg, _: &mut impl Orders<PMsg, GMsg>);
    fn sink(&mut self, _: GMsg, _: &mut impl Orders<PMsg, GMsg>) {}
}
