use crate::view::View;
use seed::prelude::*;

pub trait Model<Msg, GMsg>: View<Msg>
where
    Msg: 'static,
    GMsg: 'static,
{
    fn update(&mut self, _: Msg, _: &mut impl Orders<Msg, GMsg>);
    fn sink(&mut self, _: GMsg, _: &mut impl Orders<Msg, GMsg>) {}
}
