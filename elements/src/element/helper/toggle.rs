use crate::prelude::*;
use savory_core::prelude::*;

#[derive(Rich, Element)]
pub struct Toggle {
    #[element(config(default = "State::Closed"))]
    #[rich(read(copy))]
    state: State,
    #[element(config(default = "Action::AfterNextRender"))]
    close_after: Action,
    #[element(config(default = "Action::AfterNextRender"))]
    open_after: Action,
}

impl Clone for Toggle {
    fn clone(&self) -> Self {
        Self {
            state: self.state.clone(),
            close_after: self.close_after.clone(),
            open_after: self.open_after.clone(),
        }
    }
}

#[derive(Clone, Copy, From)]
pub enum Action {
    AfterNextRender,
    AfterMs(u32),
}

#[derive(Copy, Clone)]
pub enum Msg {
    Toggled(bool),
    Toggle,
}

impl Element for Toggle {
    type Message = Msg;
    type Config = Config;

    fn init(config: Self::Config, _: &mut impl Orders<Msg>) -> Self {
        Self {
            state: config.state,
            close_after: config.close_after,
            open_after: config.open_after,
        }
    }

    fn update(&mut self, msg: Msg, orders: &mut impl Orders<Msg>) {
        match msg {
            Msg::Toggled(true) => match self.state {
                State::Opened => {}
                State::Closed | State::Closing => {
                    self.state = State::Opening;
                    match self.open_after {
                        Action::AfterMs(ms) => orders.send_after(ms, || Msg::Toggled(true)),
                        Action::AfterNextRender => orders.after_next_render(|_| Msg::Toggled(true)),
                    };
                }
                State::Opening => self.state = State::Opened,
            },
            Msg::Toggled(false) => match self.state {
                State::Closed => {}
                State::Opened | State::Opening => {
                    self.state = State::Closing;
                    match self.close_after {
                        Action::AfterMs(ms) => orders.send_after(ms, || Msg::Toggled(false)),
                        Action::AfterNextRender => {
                            orders.after_next_render(|_| Msg::Toggled(false))
                        }
                    };
                }
                State::Closing => {
                    self.state = State::Closed;
                }
            },
            Msg::Toggle => match self.state {
                State::Opened | State::Opening => self.update(Msg::Toggled(false), orders),
                State::Closed | State::Closing => self.update(Msg::Toggled(true), orders),
            },
        }
    }
}

#[derive(Copy, Clone)]
pub enum State {
    Opened,
    Opening,
    Closed,
    Closing,
}

impl Config {
    pub fn init(self, orders: &mut impl Orders<Msg>) -> Toggle {
        Toggle::init(self, orders)
    }

    pub fn opened(mut self) -> Self {
        self.state = State::Opened;
        self
    }

    pub fn closed(mut self) -> Self {
        self.state = State::Closed;
        self
    }
}

impl Toggle {
    pub fn is_toggled(&self) -> bool {
        match self.state {
            State::Opened | State::Opening => true,
            State::Closed | State::Closing => false,
        }
    }

    pub fn toggle(&mut self, orders: &mut impl Orders<Msg>) {
        self.update(Msg::Toggle, orders);
    }

    pub fn toggled(&mut self, val: bool, orders: &mut impl Orders<Msg>) {
        self.update(Msg::Toggled(val), orders);
    }
}
