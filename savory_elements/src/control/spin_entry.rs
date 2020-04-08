use crate::prelude::*;
use derive_rich::Rich;
use savory::prelude::*;
use savory_html::prelude::*;

// TODO: add way to accept custom format (e.g. `100%`, `45$`)
#[derive(Rich, Element)]
pub struct SpinEntry<PMsg> {
    el_ref: ElRef<web_sys::HtmlInputElement>,
    msg_mapper: MsgMapper<Msg, PMsg>,
    #[rich(
        read(
            /// Return reference for the local events.
        ),
        write(
            /// add and remove local events, changes will be applied to the DOM
            /// after next render
            style = compose
        ))]
    local_events: Events<Msg>,
    #[rich(
        read(
            /// Return reference for the storde parent events.
        ),
        write(
            /// add and remove parent events, changes will be applied to the DOM
            /// after next render
            style = compose
        ))]
    events: Events<PMsg>,
    #[rich(
        read(
            /// Return reference to the user style.
        ),
        write(
            /// User style, used to customize the spin entry if you need to,
            /// changes on this style will overrides the theme style.
            style = compose
        ))]
    #[element(theme_lens)]
    style: Option<Style>,
    #[rich(read(
        /// Return the current value if there is any.
        copy
    ))]
    value: Option<f64>,
    // this is internal API, and shouldn't get exposed
    vis_value: String,
    #[rich(read(
        /// Return the max value if there is any.
        copy
    ))]
    max: Option<f64>,
    #[rich(read(
        /// Return the min value if there is any.
        copy
    ))]
    min: Option<f64>,
    #[rich(read(
        /// Return the step value.
        copy
    ))]
    step: f64,
    #[rich(read(
        /// Return the used placeholder if there is any.
        copy
    ))]
    placeholder: Option<f64>,
    #[rich(read(
        /// Return `true` if spin entry is disabled
        copy, rename = is_disabled
    ),)]
    #[element(theme_lens)]
    disabled: bool,
    #[rich(read(
        /// Return `true` if spin entry is focused
        copy, rename = is_focused
    ))]
    #[element(theme_lens)]
    focus: bool,
    #[rich(read(
        /// Return `true` if mouse is over
        copy, rename = is_mouse_over
    ))]
    #[element(theme_lens)]
    mouse_over: bool,

    // children elements
    #[rich(write(
        /// Customize the increment button. Note by doing so you may override
        /// the default behavior for this button.
        style = compose
    ))]
    pub increment_button: Button<Msg>,
    #[rich(write(
        /// Customize the decrement button. Note by doing so you may override
        /// the default behavior for this button.
        style = compose
    ))]
    pub decrement_button: Button<Msg>,
}

crate::style_type! {
    input,
    container,
    increment_button,
    decrement_button,

    {
        increment_button() -> button::Style { button: increment_button }
        decrement_button() -> button::Style { button: decrement_button }
    }
}

crate::events_type! {
    input,
    container,
}

pub enum Msg {
    MouseEnter,
    MouseLeave,
    Focus,
    Blur,
    Increment,
    Decrement,
    Input,
    IncrementButton(button::Msg),
    DecrementButton(button::Msg),
}

impl<PMsg: 'static, GMsg: 'static> Element<PMsg, GMsg> for SpinEntry<PMsg> {
    type Message = Msg;

    fn init(
        msg_mapper: impl Into<MsgMapper<Msg, PMsg>>,
        orders: &mut impl Orders<PMsg, GMsg>,
    ) -> Self {
        todo!()
    }

    fn update(&mut self, msg: Msg, orders: &mut impl Orders<PMsg, GMsg>) {
        todo!()
        // let mut orders = orders.proxy_with(&self.msg_mapper);

        // match msg {
        //     Msg::MouseEnter => self.mouse_over = true,
        //     Msg::MouseLeave => self.mouse_over = false,
        //     Msg::Focus => self.focus = true,
        //     Msg::Blur => self.focus = false,
        //     Msg::Increment => self.increment(),
        //     Msg::Decrement => self.decrement(),
        //     Msg::Input => self.handle_input(),
        //     Msg::IncrementButton(msg) => self.increment_button.update(msg, &mut orders),
        //     Msg::DecrementButton(msg) => self.decrement_button.update(msg, &mut orders),
        // }
    }
}

impl<PMsg> View for SpinEntry<PMsg> {
    type Output = Node<PMsg>;

    fn view(&self) -> Self::Output {
        todo!()
        // let inc_btn = self
        //     .increment_button
        //     .render_with_style(theme, style["increment-button"])
        //     .map_msg_with(&self.msg_mapper);
        // let dec_btn = self
        //     .decrement_button
        //     .render_with_style(theme, style["decrement-button"])
        //     .map_msg_with(&self.msg_mapper);

        // // input
        // let input = input!()
        //     .el_ref(&self.el_ref)
        //     .set(&self.local_events["input"])
        //     .set(style["input"])
        //     .and_attributes(|conf| {
        //         conf.set_class("input")
        //             .set_disabled(self.disabled)
        //             .set_value(self.vis_value.clone())
        //             .set_step(self.step)
        //             .try_set_max(self.max)
        //             .try_set_min(self.min)
        //             .try_set_placeholder(self.placeholder.map(|val| val.to_string()))
        //     })
        //     .map_msg_with(&self.msg_mapper)
        //     .try_add(self.events.get("input"));

        // // container
        // div!()
        //     .add(att::class("spin-entry"))
        //     .set(style["container"])
        //     .set(&self.local_events["container"])
        //     .map_msg_with(&self.msg_mapper)
        //     .try_add(self.events.get("container"))
        //     .add(vec![input, inc_btn, dec_btn])
    }
}

// TODO: add fn unset_max()
// TODO: add fn unset_min()
// TODO: add fn unset_placeholder()
// impl<PMsg> SpinEntry<PMsg> {
//     pub fn new(msg_mapper: impl Into<MsgMapper<Msg, PMsg>>) -> Self {
//         todo!()
//         // let local_events = Events::default()
//         //     .insert("input", |conf| {
//         //         conf.input(|_| Msg::Input)
//         //             .focus(|_| Msg::Focus)
//         //             .blur(|_| Msg::Blur)
//         //     })
//         //     .insert("container", |conf| {
//         //         conf.mouse_enter(|_| Msg::MouseEnter)
//         //             .mouse_leave(|_| Msg::MouseLeave)
//         //     });

//         // let increment_button = Button::with_label(Msg::IncrementButton, "+")
//         //     .and_events(|conf| conf.insert("button", |conf| conf.click(|_| Msg::Increment)));

//         // let decrement_button = Button::with_label(Msg::DecrementButton, "-")
//         //     .and_events(|conf| conf.insert("button", |conf| conf.click(|_| Msg::Decrement)));

//         // Self {
//         //     el_ref: ElRef::default(),
//         //     msg_mapper: msg_mapper,
//         //     local_events,
//         //     events: Events::default(),
//         //     value: None,
//         //     vis_value: "".into(),
//         //     max: None,
//         //     min: None,
//         //     step: 1.,
//         //     placeholder: None,
//         //     style: None,
//         //     disabled: false,
//         //     focus: false,
//         //     mouse_over: false,
//         //     increment_button,
//         //     decrement_button,
//         // }
//     }

//     pub fn set_min(mut self, min: f64) -> Self {
//         self.set_min_mut(min);
//         self
//     }

//     pub fn set_max(mut self, val: f64) -> Self {
//         self.set_max_mut(val);
//         self
//     }

//     pub fn set_step(mut self, step: f64) -> Self {
//         self.set_step_mut(step);
//         self
//     }

//     pub fn set_placeholder(mut self, value: impl Into<f64>) -> Self {
//         self.set_placeholder_mut(value);
//         self
//     }

//     pub fn set_value(mut self, val: f64) -> Self {
//         self.set_value_mut(val);
//         self
//     }

//     pub fn unset_value(mut self) -> Self {
//         self.unset_value_mut();
//         self
//     }

//     pub fn enable(mut self) -> Self {
//         self.el_ref.get_then(|el| el.set_disabled(false));
//         self.disabled = false;
//         self.increment_button = self.increment_button.enable();
//         self.decrement_button = self.decrement_button.enable();
//         self
//     }

//     pub fn disable(mut self) -> Self {
//         self.el_ref.get_then(|el| el.set_disabled(true));
//         self.disabled = true;
//         self.increment_button = self.increment_button.disable();
//         self.decrement_button = self.decrement_button.disable();
//         self
//     }

//     pub fn set_disabled(self, val: bool) -> Self {
//         if val {
//             self.enable()
//         } else {
//             self.disable()
//         }
//     }

//     pub fn set_max_mut(&mut self, max: f64) -> &mut Self {
//         match (max, self.min) {
//             (max, Some(min)) if max < min => {
//                 self.max = self.min;
//                 self.min = Some(max);
//             }
//             _ => self.max = Some(max),
//         }
//         // re-calc step and placeholder again
//         self.set_step_mut(self.step);
//         if let Some(placeholder) = self.placeholder {
//             self.set_placeholder_mut(placeholder);
//         }
//         self
//     }

//     fn set_min_mut(&mut self, min: f64) -> &mut Self {
//         match (min, self.max) {
//             (min, Some(max)) if min > max => {
//                 self.min = self.max;
//                 self.max = Some(min);
//             }
//             _ => self.min = Some(min),
//         }
//         // re-calc step and placeholder again
//         self.set_step_mut(self.step);
//         if let Some(placeholder) = self.placeholder {
//             self.set_placeholder_mut(placeholder);
//         }
//         self
//     }

//     fn set_step_mut(&mut self, step: f64) -> &mut Self {
//         self.step = match (step, self.min, self.max) {
//             (step, Some(min), Some(max)) if step.abs() > (min).abs() + (max).abs() => {
//                 (min).abs() + (max).abs()
//             }
//             _ => step.abs(),
//         };
//         self
//     }

//     fn set_placeholder_mut(&mut self, value: impl Into<f64>) -> &mut Self {
//         let placeholder = match (value.into(), self.min, self.max) {
//             (value, _, Some(max)) if value > max => max,
//             (value, Some(min), _) if value < min => min,
//             (value, _, _) => value,
//         };
//         self.placeholder = Some(placeholder);
//         if let Some(input) = self.el_ref.get() {
//             input.set_placeholder(&placeholder.to_string());
//         }
//         self
//     }

//     fn set_value_mut(&mut self, val: f64) -> &mut Self {
//         let val = match (val, self.min, self.max) {
//             (val, _, Some(max)) if val > max => max,
//             (val, Some(min), _) if val < min => min,
//             _ => val,
//         };
//         self.value = Some(val);
//         self.vis_value = val.to_string();
//         if let Some(input) = self.el_ref.get() {
//             input.set_value(&self.vis_value);
//         }
//         self
//     }

//     fn unset_value_mut(&mut self) -> &mut Self {
//         self.value = None;
//         self.vis_value = "".into();
//         if let Some(input) = self.el_ref.get() {
//             input.set_value(&self.vis_value);
//         }
//         self
//     }

//     fn calc_reasonable_value(&self) -> f64 {
//         match (self.value, self.min, self.max) {
//             (Some(value), _, _) => value,
//             (None, Some(min), Some(max)) => (min + max) * 0.5,
//             (None, Some(min), None) if min < 0. => 0.,
//             (None, Some(min), None) => min,
//             (None, None, Some(max)) if max > 0. => 0.,
//             (None, None, Some(max)) => max,
//             _ => 0.,
//         }
//     }

//     fn increment(&mut self) {
//         self.set_value_mut(self.calc_reasonable_value() + self.step);
//     }

//     fn decrement(&mut self) {
//         self.set_value_mut(self.calc_reasonable_value() - self.step);
//     }

//     fn handle_input(&mut self) {
//         if let Some(input) = self.el_ref.get() {
//             let value = input.value();
//             // if value is empty then we set None to self.value
//             if value.is_empty() {
//                 self.unset_value_mut();
//             } else {
//                 match value.as_str() {
//                     // these are the only allowed text when there is no number
//                     // in the input, we don't store these in self.value, but we sotre
//                     // them in self.vis_value
//                     "." => {
//                         self.vis_value = "0.".into();
//                         input.set_value(&self.vis_value);
//                     }
//                     "-." => {
//                         self.vis_value = "-0.".into();
//                         input.set_value(&self.vis_value);
//                     }
//                     "-" => {
//                         self.vis_value = value;
//                         input.set_value(&self.vis_value);
//                     }
//                     _ => {
//                         // parse value to f64
//                         if let Some(v_f64) = value.parse::<f64>().ok() {
//                             self.set_value_mut(v_f64);
//                             // check if value is eq to v_f64
//                             if self.value == Some(v_f64) && value.ends_with(".") {
//                                 // use the input value as vis_value if so, this
//                                 // helps keep the last dot when user enter e.g.
//                                 // `5.`, without this, the input will be
//                                 // converted to `5`
//                                 self.vis_value = value;
//                                 input.set_value(&self.vis_value);
//                             }
//                         }
//                     }
//                 };
//             }
//         }
//     }
// }
