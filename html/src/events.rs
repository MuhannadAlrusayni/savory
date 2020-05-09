//! Types and functions for working with elements events.

use crate::prelude::*;
use seed::prelude::{ev, keyboard_ev, mouse_ev, pointer_ev, Ev, EventHandler};
use std::any::{Any, TypeId};
use wasm_bindgen::JsCast;

macro_rules! events_api {
    ( $( $event:ident: $ty:ty { $( $(#[$doc:meta])* $name:ident = $ev:expr $(,)? )* } $(,)? )* ) => {
        pub trait EventsApi<Msg: 'static> {
            $(
                $(
                    $( #[$doc] )*
                    fn $name<EMsg: 'static>(self, handler: impl FnOnce($ty) -> EMsg + 'static + Clone) -> Self;
                )*
            )*
        }

        impl<Msg: 'static> EventsApi<Msg> for El<Msg> {
            $(
                $(
                    fn $name<EMsg: 'static>(mut self, handler: impl FnOnce($ty) -> EMsg + 'static + Clone) -> Self {
                        self.add_event_handler($event($ev, handler));
                        self
                    }
                )*
            )*
        }

        impl<Msg: 'static> EventsApi<Msg> for Node<Msg> {
            $(
                $(
                    fn $name<EMsg: 'static>(self, handler: impl FnOnce($ty) -> EMsg + 'static + Clone) -> Self {
                        self.and_element(|el| el.$name(handler))
                    }
                )*
            )*
        }
    }
}

events_api! {
    ev: web_sys::Event {
        on_scroll = Ev::Scroll,
        on_after_print = Ev::AfterPrint,
        on_before_print = Ev::BeforePrint,
        on_app_installed = Ev::from("appinstalled"),
        on_seeked = Ev::Seeked,
        on_seeking = Ev::Seeking,
        on_play = Ev::Play,
        on_playing = Ev::Playing,
        on_rate_change = Ev::RateChange,
        on_can_play = Ev::CanPlay,
        on_can_play_through = Ev::CanPlayThrough,
        on_reset = Ev::from("reset"),
        on_change = Ev::Change,
        on_load = Ev::Load,
        on_unload = Ev::Unload,
        on_abort = Ev::Abort,
        on_error = Ev::Error,
        on_emptied = Ev::Emptied,
        on_ended = Ev::Ended,
        on_full_screen_change = Ev::FullScreenChange,
        on_full_screen_error = Ev::FullScreenError,
        on_invalid = Ev::from("invalid"),
        on_offline = Ev::Offline,
        on_online = Ev::Online,
        on_select_start = Ev::from("selectstart"),
        on_selectionchange = Ev::from("selectionchange"),
        on_submit = Ev::Submit,
    }
    focus_ev: web_sys::FocusEvent {
        on_blur = Ev::Blur,
        on_focus = Ev::Focus,
        on_focus_in = Ev::from("focusin"),
        on_focus_out = Ev::from("focusout"),
    }
    mouse_ev: web_sys::MouseEvent {
        on_aux_click = Ev::AuxClick,
        on_click = Ev::Click,
        on_double_click = Ev::DblClick,
        on_mouse_down = Ev::MouseDown,
        on_mouse_enter = Ev::MouseEnter,
        on_mouse_leave = Ev::MouseLeave,
        on_mouse_move = Ev::MouseMove,
        on_mouse_out = Ev::MouseOut,
        on_mouse_over = Ev::MouseOver,
        on_mouse_up = Ev::MouseUp,
        on_context_menu = Ev::ContextMenu,
    }
    pointer_ev: web_sys::PointerEvent {
        on_pointer_cancel = Ev::PointerCancel,
        on_pointer_down = Ev::PointerDown,
        on_pointer_enter = Ev::PointerEnter,
        on_pointer_leave = Ev::PointerLeave,
        on_pointer_move = Ev::PointerMove,
        on_pointer_out = Ev::PointerOut,
        on_pointer_over = Ev::PointerOver,
        on_pointer_up = Ev::PointerUp,
        on_lost_pointer_capture = Ev::LostPointerCapture,
        on_got_pointer_capture = Ev::GotPointerCapture,
    }
    wheel_ev: web_sys::WheelEvent {
        on_wheel = Ev::Wheel,
    }
    // NOTE: that `InputEvent` doesn't provied access to data property yet, use
    // seed::browser::util::{get_value, set_value} to access these value for
    // now.
    input_ev: web_sys::InputEvent {
        on_before_input = Ev::from("beforeinput")
        on_input = Ev::Input,
    }
    keyboard_ev: web_sys::KeyboardEvent {
        on_key_down = Ev::KeyDown,
        on_key_up = Ev::KeyUp,
    }
    composition_ev: web_sys::CompositionEvent {
        on_composition_start = Ev::CompositionStart,
        on_composition_update = Ev::CompositionUpdate,
        on_composition_end = Ev::CompositionEnd,
    }
    drag_ev: web_sys::DragEvent {
        on_drag = Ev::Drag,
        on_drag_end = Ev::DragEnd,
        on_drag_enter = Ev::DragEnter,
        on_drag_leave = Ev::DragLeave,
        on_drag_over = Ev::DragOver,
        on_drag_start = Ev::DragStart,
        on_drop = Ev::Drop,
    }
    ui_ev: web_sys::UiEvent {
        on_resize = Ev::Resize,
        on_select = Ev::Select,
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
                // TODO: Supoort Option<Ms> as in Seed API
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
