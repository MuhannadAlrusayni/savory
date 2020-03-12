//! Types and functions for working with elements events.

use crate::prelude::{El, UpdateEl};
use seed::prelude::{ev, keyboard_ev, mouse_ev, pointer_ev, Ev, EventHandler};
use wasm_bindgen::JsCast;

pub struct Events<Msg> {
    pub events: Vec<EventHandler<Msg>>,
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

macro_rules! mouse_events {
    ( $( $event:ident: $ty:ty { $( $(#[$doc:meta])? $name:ident = $ev:expr $(,)? )* } $(,)? )* ) => {
        $(
            impl<Msg> Events<Msg> {
                $(
                    $( #[$doc] )?
                    pub fn $name(
                        &mut self,
                        handler: impl FnOnce($ty) -> Msg + 'static + Clone,
                    ) -> &mut Self {
                        self.events.push($event($ev, handler));
                        self
                    }
                )*
            }
        )*
    }
}

mouse_events! {
    ev: web_sys::Event {
        load = Ev::Load,
        unload = Ev::Unload,
        abort = Ev::Abort,
        error = Ev::Error,
        select = Ev::Select,
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
}

macro_rules! event_creator{
    ( $( $(#[$doc:meta])? $name:ident($ty:ty) $(,)? )* ) => {
        $(
            $( #[$doc] )?
            fn $name<Ms>(
                trigger: impl Into<Ev>,
                handler: impl FnOnce($ty) -> Ms + 'static + Clone,
            ) -> EventHandler<Ms> {
                let closure_handler = move |event: web_sys::Event| {
                    (handler.clone())(event.dyn_ref::<$ty>().unwrap().clone())
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
    input_ev(web_sys::InputEvent)
}
