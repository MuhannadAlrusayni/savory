//! Types and functions for working with elements events.

use crate::prelude::{El, UpdateEl};
use seed::prelude::{ev, keyboard_ev, mouse_ev, pointer_ev, Ev, EventHandler, MessageMapper};
use std::{
    any::{Any, TypeId},
    rc::Rc,
};
use wasm_bindgen::JsCast;

pub struct Events<Msg> {
    pub events: Vec<EventHandler<Msg>>,
}

impl<Ms: 'static, OtherMs: 'static> MessageMapper<Ms, OtherMs> for Events<Ms> {
    type SelfWithOtherMs = Events<OtherMs>;

    fn map_msg(self, f: impl FnOnce(Ms) -> OtherMs + 'static + Clone) -> Self::SelfWithOtherMs {
        let mut events = vec![];
        for event in self.events.into_iter() {
            events.push(event.map_msg(f.clone()));
        }
        Events { events }
    }
}

impl<Msg> Clone for Events<Msg> {
    fn clone(&self) -> Self {
        Self {
            events: self.events.clone(),
        }
    }
}

impl<Msg> Default for Events<Msg> {
    fn default() -> Self {
        Self { events: vec![] }
    }
}

impl<Msg> UpdateEl<Msg> for Events<Msg> {
    fn update_el(self, el: &mut El<Msg>) {
        self.events.update_el(el)
    }
}

macro_rules! events_functions {
    ( $( $event:ident: $ty:ty { $( $(#[$doc:meta])? $name:ident = $ev:expr $(,)? )* } $(,)? )* ) => {
        $(
            impl<Ms: 'static> Events<Ms> {
                $(
                    $( #[$doc] )?
                    pub fn $name(
                        mut self,
                        handler: impl FnOnce($ty) -> Ms + 'static + Clone,
                    ) -> Self {
                        self.events.push($event($ev, handler));
                        self
                    }
                )*
            }
        )*
    }
}

events_functions! {
    ev: web_sys::Event {
        scroll = Ev::Scroll,
        after_print = Ev::AfterPrint,
        before_print = Ev::BeforePrint,
        app_installed = Ev::from("appinstalled"),
        seeked = Ev::Seeked,
        seeking = Ev::Seeking,
        play = Ev::Play,
        playing = Ev::Playing,
        rate_change = Ev::RateChange,
        can_play = Ev::CanPlay,
        can_play_through = Ev::CanPlayThrough,
        reset = Ev::from("reset"),
        change = Ev::Change,
        load = Ev::Load,
        unload = Ev::Unload,
        abort = Ev::Abort,
        error = Ev::Error,
        emptied = Ev::Emptied,
        ended = Ev::Ended,
        full_screen_change = Ev::FullScreenChange,
        full_screen_error = Ev::FullScreenError,
        invalid = Ev::from("invalid"),
        offline = Ev::Offline,
        online = Ev::Online,
        select_start = Ev::from("selectstart"),
        selectionchange = Ev::from("selectionchange"),
        submit = Ev::Submit,
    }
    focus_ev: web_sys::FocusEvent {
        blur = Ev::Blur,
        focus = Ev::Focus,
        focus_in = Ev::from("focusin"),
        focus_out = Ev::from("focusout"),
    }
    mouse_ev: web_sys::MouseEvent {
        aux_click = Ev::AuxClick,
        click = Ev::Click,
        double_click = Ev::DblClick,
        mouse_down = Ev::MouseDown,
        mouse_enter = Ev::MouseEnter,
        mouse_leave = Ev::MouseLeave,
        mouse_move = Ev::MouseMove,
        mouse_out = Ev::MouseOut,
        mouse_over = Ev::MouseOver,
        mouse_up = Ev::MouseUp,
        context_menu = Ev::ContextMenu,
    }
    pointer_ev: web_sys::PointerEvent {
        pointer_cancel = Ev::PointerCancel,
        pointer_down = Ev::PointerDown,
        pointer_enter = Ev::PointerEnter,
        pointer_leave = Ev::PointerLeave,
        pointer_move = Ev::PointerMove,
        pointer_out = Ev::PointerOut,
        pointer_over = Ev::PointerOver,
        pointer_up = Ev::PointerUp,
        lost_pointer_capture = Ev::LostPointerCapture,
        got_pointer_capture = Ev::GotPointerCapture,
    }
    wheel_ev: web_sys::WheelEvent {
        wheel = Ev::Wheel,
    }
    // NOTE: that `InputEvent` doesn't provied access to data property yet, use
    // seed::browser::util::{get_value, set_value} access these value for now.
    input_ev: web_sys::InputEvent {
        before_input = Ev::from("beforeinput")
        input = Ev::Input,
    }
    keyboard_ev: web_sys::KeyboardEvent {
        key_down = Ev::KeyDown,
        key_up = Ev::KeyUp,
    }
    composition_ev: web_sys::CompositionEvent {
        composition_start = Ev::CompositionStart,
        composition_update = Ev::CompositionUpdate,
        composition_end = Ev::CompositionEnd,
    }
    drag_ev: web_sys::DragEvent {
        drag = Ev::Drag,
        drag_end = Ev::DragEnd,
        drag_enter = Ev::DragEnter,
        drag_leave = Ev::DragLeave,
        drag_over = Ev::DragOver,
        drag_start = Ev::DragStart,
        drop = Ev::Drop,
    }
    ui_ev: web_sys::UiEvent {
        resize = Ev::Resize,
        select = Ev::Select,
    }
}

macro_rules! event_creator{
    ( $( $(#[$doc:meta])? $name:ident($ty:ty) $(,)? )* ) => {
        $(
            $( #[$doc] )?
            fn $name<Ms: 'static, HandlerMs: 'static>(
                trigger: impl Into<Ev>,
                handler: impl FnOnce($ty) -> HandlerMs + 'static + Clone,
            ) -> EventHandler<Ms> {
                let msg_type = TypeId::of::<HandlerMs>();
                if msg_type != TypeId::of::<Ms>() && msg_type != TypeId::of::<()>() {
                    panic!("Handler can return only Ms or ()!");
                }

                let closure_handler = move |event: web_sys::Event| {
                    let output = &mut Some(handler.clone()(event.dyn_ref::<$ty>().unwrap().clone())) as &mut dyn Any;
                    output.downcast_mut::<Option<Ms>>().and_then(Option::take)
                };
                EventHandler::new(trigger, closure_handler)
            }
        )*
    }
}

event_creator! {
    /// create `EventHandler` with `web_sys::WheelEvent`
    wheel_ev(web_sys::WheelEvent),
    /// create `EventHandler` with `web_sys::CompositionEvent`
    composition_ev(web_sys::CompositionEvent),
    /// create `EventHandler` with `web_sys::FocusEvent`
    focus_ev(web_sys::FocusEvent),
    /// create `EventHandler` with `web_sys::DragEvent`
    drag_ev(web_sys::DragEvent),
    /// create `EventHandler` with `web_sys::InputEvent`
    input_ev(web_sys::InputEvent),
    /// create `EventHandler` with `web_sys::UiEvent`
    ui_ev(web_sys::UiEvent),
}

/// Type used to generate events when get called, this type hold function that
/// return events.
pub struct EventsStore<Ev>(Rc<dyn Fn() -> Ev>);

impl<Ev> EventsStore<Ev> {
    pub fn new(f: impl Fn() -> Ev + 'static) -> Self {
        Self(Rc::new(f))
    }

    pub fn get(&self) -> Ev {
        self.0()
    }
}

impl<Ev> From<Rc<EventsStore<Ev>>> for EventsStore<Ev> {
    fn from(val: Rc<EventsStore<Ev>>) -> Self {
        Self(Rc::clone(&val.0))
    }
}

impl<Ev> From<Rc<dyn Fn() -> Ev>> for EventsStore<Ev> {
    fn from(val: Rc<dyn Fn() -> Ev>) -> Self {
        Self(val)
    }
}

impl<Ev, T> From<T> for EventsStore<Ev>
where
    T: Fn() -> Ev + 'static,
{
    fn from(val: T) -> Self {
        Self(Rc::new(val))
    }
}

impl<Ev> std::ops::Deref for EventsStore<Ev> {
    type Target = dyn Fn() -> Ev;

    fn deref(&self) -> &Self::Target {
        self.0.deref()
    }
}

impl<Ev> Clone for EventsStore<Ev> {
    fn clone(&self) -> Self {
        Self(Rc::clone(&self.0))
    }
}

impl<Ev: Default> Default for EventsStore<Ev> {
    fn default() -> Self {
        Self(Rc::new(|| Ev::default()))
    }
}
