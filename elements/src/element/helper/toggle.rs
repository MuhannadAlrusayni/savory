use crate::prelude::*;
use savory_core::prelude::*;

#[derive(Rich, Element)]
pub struct Toggle<PMsg> {
    #[element(props(required))]
    msg_mapper: MsgMapper<Msg, PMsg>,
    #[element(props(default = "State::Closed"))]
    #[rich(read(copy))]
    state: State,
    #[element(props(default = "Action::AfterNextRender"))]
    closing: Action,
    #[element(props(default = "Action::AfterNextRender"))]
    opening: Action,
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

impl<PMsg: 'static> Element<PMsg> for Toggle<PMsg> {
    type Message = Msg;
    type Props = Props<PMsg>;

    fn init(props: Self::Props, _: &mut impl Orders<PMsg>) -> Self {
        Self {
            msg_mapper: props.msg_mapper,
            state: props.state,
            closing: props.closing,
            opening: props.opening,
        }
    }

    fn update(&mut self, msg: Msg, p_orders: &mut impl Orders<PMsg>) {
        let mut orders = p_orders.proxy_with(&self.msg_mapper);

        match msg {
            Msg::Toggled(true) => match self.state {
                State::Opened => {}
                State::Closed | State::Closing => {
                    self.state = State::Opening;
                    match self.opening {
                        Action::AfterMs(ms) => orders.perform_cmd_after(ms, || Msg::Toggled(true)),
                        Action::AfterNextRender => orders.after_next_render(|_| Msg::Toggled(true)),
                    };
                }
                State::Opening => self.state = State::Opened,
            },
            Msg::Toggled(false) => match self.state {
                State::Closed => {}
                State::Opened | State::Opening => {
                    self.state = State::Closing;
                    match self.closing {
                        Action::AfterMs(ms) => orders.perform_cmd_after(ms, || Msg::Toggled(false)),
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
                State::Opened | State::Opening => self.update(Msg::Toggled(false), p_orders),
                State::Closed | State::Closing => self.update(Msg::Toggled(true), p_orders),
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

impl<PMsg: 'static> Props<PMsg> {
    pub fn init(self, orders: &mut impl Orders<PMsg>) -> Toggle<PMsg> {
        Toggle::init(self, orders)
    }
}

impl<PMsg: 'static> Toggle<PMsg> {
    pub fn is_toggled(&self) -> bool {
        match self.state {
            State::Opened | State::Opening => true,
            State::Closed | State::Closing => false,
        }
    }

    pub fn toggle(&mut self, orders: &mut impl Orders<PMsg>) {
        self.update(Msg::Toggle, orders);
    }

    pub fn toggled(&mut self, val: bool, orders: &mut impl Orders<PMsg>) {
        self.update(Msg::Toggled(val), orders);
    }
}
