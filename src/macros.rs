pub macro builder_functions( $( $attr:ident($t:ty) $(,)? )* ) {
    $(
        pub fn $attr(mut self, value: impl Into<$t>) -> Self {
            self.$attr = Some(value.into());
            self
        }
    )*
}

pub macro builder_enum_functions( $( $attr:ident { $( $fn:ident() => $value:expr $(,)? )* } $(,)? )* ) {
    $(
        $(
            pub fn $fn(mut self) -> Self {
                self.$attr = Some($value.into());
                self
            }
        )*
    )*
}

pub macro value_functions( $( $attr:ident { $( $fn:ident() => $value:expr $(,)? )* } $(,)? )* ) {
    $(
        $(
            pub fn $fn(mut self) -> Self {
                self.$attr = $value.into();
                self
            }
        )*
    )*
}

pub macro composition_functions( $( $attr:ident: $ty:ident $(,)? )* ) {
    $(
        pub fn $attr(mut self, get_new_value: impl FnOnce($ty) -> $ty) -> Self {
            self.$attr = get_new_value(self.$attr);
            self
        }
    )*
}

pub macro option_composition_functions( $( $attr:ident: $ty:ident $(,)? )* ) {
    $(
        pub fn $attr(mut self, get_new_value: impl FnOnce($ty) -> $ty) -> Self {
            self.$attr = Some(get_new_value(self.$attr.unwrap_or_default()));
            self
        }
    )*
}
