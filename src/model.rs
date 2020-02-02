use crate::render::Render;
use seed::prelude::*;

pub trait Model<PMsg, GMsg>: Render<PMsg>
where
    PMsg: 'static,
{
    type Message;

    fn update(&mut self, _: Self::Message, _: &mut impl Orders<PMsg, GMsg>);
    fn sink(&mut self, _: GMsg, _: &mut impl Orders<PMsg, GMsg>) {}
}
