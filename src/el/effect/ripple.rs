use crate::{css::{self, Background, Border, Size, Style}, model::Model, render::Render, theme::Theme};
use seed::prelude::*;
use palette::Hsla;


#[derive(Debug, Copy, Clone)]
pub enum Msg {
    Clicked(f64, f64),
    Render(Option<RenderTimestampDelta>),
    Done,
}

#[derive(Debug, Clone)]
pub struct Ripple {
    animating: bool,
    orgin: (f64, f64),
    size: f64,
    start: f64,
}

impl Ripple {
    pub fn new(x: f64, y: f64) -> Self {
        Self {
            animating: false,
            orgin: (x, y),
            size: 0.0,
            start: 0.,
        }
    }

    fn orgin(&mut self, point: impl Into<(f64, f64)>) -> &mut Self {
        self.orgin = point.into();
        self
    }

    fn scale(&mut self, scale: impl Into<f64>) -> &mut Self {
        self.size += scale.into();
        self
    }
}

impl<GMsg: 'static> Model<Msg, GMsg> for Ripple {
    fn update(&mut self, msg: Msg, orders: &mut impl Orders<Msg, GMsg>) {
        match msg {
            Msg::Clicked(x, y) => {
                *self = Ripple::new(x, y);
                self.animating = true;
                orders.after_next_render(Msg::Render);
            }
            Msg::Render(delta) => {
                let delta: f64 = delta.unwrap_or_default().into();
                if self.start == 0. {
                    self.start = delta;
                }
                let progress = delta - self.start;
                if self.size < 1.0 {
                    self.scale(progress / 1000.);
                    orders.after_next_render(Msg::Render);
                } else {
                    orders.send_msg(Msg::Done);
                }
            }
            Msg::Done => {
                self.start = 0.;
                self.animating = false;
            }
        }
    }
}

impl Render<Msg> for Ripple {
    type View = Node<Msg>;


    fn render(&self, theme: &impl Theme) -> Self::View {
        let background = Background::default()
            .color(Hsla::new(0., 0., 1., 0.8));

        let border = Border::default()
            .radius(1.);

        let size = Size::default()
            .resize(self.size as f32, self.size as f32);

        let style = Style::default()
            .add(St::Position, "relative")
            .add(St::Left, self.orgin.0)
            .add(St::Top, self.orgin.1)
            .merge(&background)
            .merge(&border)
            .merge(&size);

        if self.animating {
            div! [style]
        } else {
            empty![]
        }
    }
}
