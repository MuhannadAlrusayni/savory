use crate::prelude::*;
use instant::Instant;
use savory::prelude::*;
use savory_style::{self as style, prelude::*};
use std::{any::Any, ops::Deref};

// TODO: add support for timeline, so we can repeate the animation.
#[derive(Rich, Element)]
#[element(config_bound = "S: 'static")]
pub struct Animator<S> {
    #[element(config(required))]
    init_state: Box<S>,
    last_instant: Instant,
    #[element(config(default = "vec![]", no_pub))]
    timeline: Vec<(u32, Box<S>)>,
    #[element(config(default = "1000u32", no_pub))]
    replay_delay: u32,
    playing: Playing<S>,
}

pub enum Playing<S> {
    Off,
    On {
        cursor: Cursor,
        queued: Vec<(u32, Box<S>)>,
        replay: bool,
    },
}

pub enum Cursor {
    AtInit,
    At(usize),
}

pub enum Msg {
    Update(Box<dyn Any>),
    Replay,
    ReplayLoop,
    ReplayDelay(u32),
    Stop,
    ResetTimeline,
    Playing(Cursor),
}

impl<S: 'static> Element for Animator<S> {
    type Message = Msg;
    type Config = Config<S>;

    fn init(config: Self::Config, _: &mut impl Orders<Msg>, env: &Env) -> Self {
        Self {
            init_state: config.init_state,
            last_instant: Instant::now(),
            timeline: config.timeline,
            replay_delay: 1000,
            playing: Playing::Off,
        }
    }

    fn update(&mut self, msg: Msg, orders: &mut impl Orders<Msg>) {
        match msg {
            Msg::Update(val) => {
                if let Ok(val) = val.downcast::<S>() {
                    let now = Instant::now();
                    let duration = now.duration_since(self.last_instant);
                    self.last_instant = now;
                    let ms = duration.as_millis() as u32;
                    match self.playing {
                        Playing::Off => self.timeline.push((ms, val)),
                        Playing::On { ref mut queued, .. } => queued.push((ms, val)),
                    }
                }
            }
            Msg::Playing(cursor) => self.playing(cursor, orders),
            Msg::Replay => self.replay(false, orders),
            Msg::ReplayDelay(ms) => self.replay_delay = ms,
            Msg::ReplayLoop => self.replay(true, orders),
            Msg::Stop => self.stop(),
            Msg::ResetTimeline => self.reset_timeline(),
        }
    }
}

impl<S> Animator<S> {
    pub fn animate<PMsg>(
        &self,
        node: Node<PMsg>,
        styler: impl FnOnce(&S, style::Style) -> style::Style,
    ) -> Node<PMsg> {
        let style = styler(self.current(), style::Style::default());
        node.style(style)
    }

    pub fn current(&self) -> &S {
        let last = self.timeline.last().map(|(_, s)| s);
        match self.playing {
            Playing::Off => last.unwrap_or(&self.init_state),
            Playing::On { ref cursor, .. } => match cursor {
                Cursor::AtInit => &self.init_state,
                Cursor::At(index) => self
                    .timeline
                    .get(*index)
                    .map(|(_, s)| s)
                    .unwrap_or_else(|| last.unwrap_or(&self.init_state)),
            },
        }
    }

    pub fn previous(&self) -> Option<&S> {
        match self.playing {
            Playing::Off => match self.timeline.len() {
                0 => None,
                1 => Some(&self.init_state),
                len => self.timeline.get(len - 1).map(|(_, s)| s.deref()),
            },
            Playing::On { ref cursor, .. } => match cursor {
                Cursor::AtInit | Cursor::At(0) => None,
                Cursor::At(index) => match index {
                    0 => Some(&self.init_state),
                    index => self.timeline.get(*index).map(|(_, s)| s.deref()),
                },
            },
        }
    }

    fn replay(&mut self, replay: bool, orders: &mut impl Orders<Msg>) {
        if let Playing::Off = self.playing {
            if self.timeline.is_empty() {
                return;
            }

            orders.send(Msg::Playing(Cursor::AtInit));

            self.playing = Playing::On {
                queued: vec![],
                replay,
                cursor: Cursor::AtInit,
            };
        }
    }

    fn stop(&mut self) {
        if let Playing::On { ref mut queued, .. } = self.playing {
            self.timeline.extend(queued.drain(..));
            self.playing = Playing::Off;
        }
    }

    fn playing(&mut self, val: Cursor, orders: &mut impl Orders<Msg>) {
        if let Playing::On {
            ref mut cursor,
            replay,
            ..
        } = self.playing
        {
            *cursor = match val {
                // first state
                Cursor::AtInit => {
                    if let Some((ms, _)) = self.timeline.first() {
                        orders.send_after(*ms, || Msg::Playing(Cursor::At(0)));
                        val
                    } else {
                        // stop playing if there is no other state
                        self.stop();
                        return;
                    }
                }
                // other states
                Cursor::At(index) => {
                    if let Some((ms, _)) = self.timeline.get(index + 1) {
                        orders.send_after(*ms, move || Msg::Playing(Cursor::At(index + 1)));
                        val
                    } else {
                        if replay {
                            orders.send_after(self.replay_delay, move || {
                                Msg::Playing(Cursor::AtInit)
                            });
                            val
                        } else {
                            // stop when there is no other states
                            self.stop();
                            return;
                        }
                    }
                }
            };
        }
    }

    fn reset_timeline(&mut self) {
        self.last_instant = Instant::now();
        self.stop();
        self.timeline = vec![];
    }
}

impl<S> Config<S> {
    pub fn timeline(mut self, states: impl IntoIterator<Item = (u32, S)>) -> Self {
        self.timeline = states
            .into_iter()
            .map(|(ms, s)| (ms, Box::new(s)))
            .collect();
        self
    }

    pub fn replay_delay(mut self, ms: u32) -> Self {
        self.replay_delay = ms;
        self
    }
}

impl Msg {
    pub fn update<S: 'static>(state: S) -> Msg {
        Msg::Update(Box::new(state))
    }

    pub fn play() -> Msg {
        Msg::Replay
    }

    pub fn replay_loop() -> Msg {
        Msg::ReplayLoop
    }

    pub fn stop() -> Msg {
        Msg::Stop
    }

    pub fn rest_timeline() -> Msg {
        Msg::ResetTimeline
    }

    pub fn playing(cursor: Cursor) -> Msg {
        Msg::Playing(cursor)
    }
}
