use crate::{model::Model, view::View, theme::Theme};
use seed::prelude::*;


#[derive(Debug, Clone)]
pub enum Msg {
    Clicked(f32, f32),
    Render(Option<RenderTimestampDelta>),
    Done,
}

pub struct Ripple {
    animating: bool,
    orgin: (f32, f32),
    size: f32,
}

impl Ripple {
    pub fn new(x: f32, y: f32) -> Self {
        Self {
            animating: false,
            orgin: (x, y),
            size: 0.0
        }
    }

    fn orgin(&mut self, point: impl Into<(f32, f32)>) -> &mut Self {
        self.orgin = point.into();
        self
    }

    fn scale(&mut self, scale: impl Into<f32>) -> &mut Self {
        self.size += scale.into();
        self
    }
}

impl<GMsg: 'static> Model<Msg, GMsg> for Ripple {
    fn update(&mut self, msg: Msg, orders: &mut impl Orders<Msg, GMsg>) {
        match msg {
            Msg::Clicked(x, y) => {
                *self = Ripple::new(x, y);
                orders.after_next_render(Msg::Render);
            }
            Msg::Render(delta) => {
                if self.size < 20.0 {
                    self.scale(2.0);
                    orders.after_next_render(Msg::Render);
                } else {
                    orders.send_msg(Msg::Done);
                }
            }
            Msg::Done => {
                self.animating = false;
            }
        }
    }
}

impl View<Msg> for Ripple {
    fn view(&self, theme: &impl Theme) -> Node<Msg> {
        if self.animating {
            div! [
                style! [
                    St::Position => "relative",
                    St::AnimationName => "slide",
                    St::AnimationDuration => "3s",
                ]
            ]
        } else {
            empty![]
        }
    }
}
