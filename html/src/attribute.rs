//! Types and functions used to create HTML attributes.

use crate::{css::unit::*, prelude::*};
use derive_rich::Rich;
use indexmap::IndexSet;
use seed::prelude::{AsAtValue, At};
use std::borrow::Cow;

macro_rules! create_attributes {
    ( @shortcut $name:ident $fn_name:ident $(,)? ) => {
        pub fn $fn_name(value: impl Into<$name>) -> $name {
            value.into()
        }
    };
    ( @shortcut $name:ident update_el $expr:expr $(;)? ) => {
        impl<Msg> UpdateEl<Msg> for $name {
            fn update_el(self, el: &mut El<Msg>) {
                let closure = $expr;
                if let Some(val) = closure(self) {
                    el.attrs.add(At::$name, val);
                }
            }
        }
    };
    ( @shortcut custom $html_name:literal $name:ident update_el $expr:expr $(;)? ) => {
        impl<Msg> UpdateEl<Msg> for $name {
            fn update_el(self, el: &mut El<Msg>) {
                let closure = $expr;
                if let Some(val) = closure(self) {
                    el.attrs.add(At::from($html_name), val);
                }
            }
        }
    };
    ( @shortcut $( #[$attrs:meta] )* $name:ident $( $( #[$ty_attrs:meta] )* $ty:ty )? ) => {
        $( #[$attrs] )*
        $(
            pub struct $name( $( #[$ty_attrs] )* $ty );

            impl $name {
                pub fn into_inner(self) -> $ty {
                    self.0
                }
            }
        )?
    };
    ( $(
        $( #[$attrs:meta] )*
        $name:ident $( ( $( #[$ty_attrs:meta] )* $ty:ty ) )? {
            update_el: $expr:expr,
            $( $fn_name:ident, )?
            $( { $( $tokens:tt )* } )?
        }
    )* ) => {
        $(
            create_attributes!(@shortcut $( #[$attrs] )* $name $( $( #[$ty_attrs] )* $ty )? );
            create_attributes!(@shortcut $( $($tokens)* )? $name update_el $expr );
            $( create_attributes!(@shortcut $name $fn_name); )?
        )*
        // Sum type that contains all attributes types
        #[derive(Debug, Clone, From)]
        pub enum Attribute {
            $(
                #[from]
                $name($name),
            )*
        }

        impl<Msg> UpdateEl<Msg> for Attribute {
            fn update_el(self, el: &mut El<Msg>) {
                match self {
                    $(
                        Attribute::$name(attr) => attr.update_el(el),
                    )*
                }
            }
        }

        // type that contains at most one value for each attribute
        #[derive(Rich, Default, Debug, Clone)]
        pub struct Attributes {
            #[rich(write(style = compose))]
            pub custom_datas: Vec<Custom>,
            #[rich(write, write(option))]
            pub view_box: Option<ViewBox>,
            #[rich(write(rename = type_), write(option))]
            pub type_: Option<Type>,
            $(
                $(
                    #[rich(write, write(option))]
                    pub $fn_name: Option<$name>,
                )?
            )*
        }

        impl<Msg> UpdateEl<Msg> for Attributes {
            fn update_el(self, el: &mut El<Msg>) {
                for custom_data in self.custom_datas.into_iter() {
                    custom_data.update_el(el);
                }

                if let Some(view_box) = self.view_box {
                    view_box.update_el(el);
                }

                if let Some(ty) = self.type_ {
                    ty.update_el(el);
                }

                $(
                    $(
                        if let Some(val) = self.$fn_name {
                            val.update_el(el);
                        }
                    )?
                )*
            }
        }

        impl Attributes {
            pub fn add_custom_data(
                &mut self,
                name: impl Into<Cow<'static, str>>,
                value: impl Into<Cow<'static, str>>
            ) -> &mut Self {
                self.custom_datas.push(custom_data(name, value));
                self
            }
        }
    };
}

create_attributes! {
    #[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Clone, From)]
    Abbr(#[from(forward)] Cow<'static, str>) {
        update_el: |abbr: Abbr| Some(abbr.0),
        abbr,
        { custom "abbr" }
    }

    // TODO: should we change inner type to something like mime::Mime ?
    #[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Clone, From)]
    Accept(#[from(forward)] Cow<'static, str>) {
        update_el: |accept: Accept| Some(accept.0),
        accept,
    }

    // TODO: should we change inner type to something like murdoch::CharacterSetEnum ?
    #[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Clone, From)]
    AcceptCharset(#[from(forward)] Cow<'static, str>) {
        update_el: |charset: AcceptCharset| Some(charset.0),
        accept_charset,
    }

    #[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Clone, From)]
    AccessKey(#[from(forward)] Cow<'static, str>) {
        update_el: |key: AccessKey| Some(key.0),
        access_key,
    }

    // TODO: should we change inner type to something like url::Url ?
    #[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Clone, From)]
    Action(#[from(forward)] Cow<'static, str>) {
        update_el: |action: Action| Some(action.0),
        action,
    }

    #[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Clone, From)]
    Allow(#[from(forward)] Cow<'static, str>) {
        update_el: |allow: Allow| Some(allow.0),
        allow,
        { custom "allow" }
    }

    #[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Copy, Clone, From)]
    AllowFullScreen(bool) {
        update_el: |full_screen: AllowFullScreen| Some(full_screen.0.as_at_value()),
        allow_full_screen,
        { custom "allowfullscreen" }
    }

    #[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Copy, Clone, From)]
    AllowPaymentRequest(bool) {
        update_el: |val: AllowPaymentRequest| Some(val.0.as_at_value()),
        allow_payment_request,
        { custom "allowpaymentrequest" }
    }

    #[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Clone, From)]
    Alt(#[from(forward)] Cow<'static, str>) {
        update_el: |alt: Alt| Some(alt.0),
        alt,
    }

    #[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Clone, From)]
    As(#[from(forward)] Cow<'static, str>) {
        update_el: |val: As| Some(val.0),
        as_,
        { custom "as"}
    }

    AutoCapitalize {
        update_el: |val: AutoCapitalize| Some(val),
        auto_capitalize,
        { custom "autocapitalize"}
    }

    AutoComplete {
        update_el: |auto: AutoComplete| Some(auto),
        auto_complete,
    }

    #[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Copy, Clone, From)]
    AutoFocus(bool) {
        update_el: |auto: AutoFocus| Some(auto.0.as_at_value()),
        auto_focus,
    }

    #[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Copy, Clone, From)]
    AutoPlay(bool) {
        update_el: |auto: AutoPlay| Some(auto.0.as_at_value()),
        auto_play,
    }

    // #[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Clone, From)]
    // Charset(#[from(forward)] Cow<'static, str>) {
    //     update_el: |charset: Charset| Some(charset.0),
    //     charset,
    // }

    #[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Copy, Clone, From)]
    Checked(bool) {
        update_el: |checked: Checked| Some(checked.0.as_at_value()),
        checked,
    }

    // TODO: should we change inner type to something like url::Url ?
    #[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Clone, From)]
    Cite(#[from(forward)] Cow<'static, str>) {
        update_el: |cite: Cite| Some(cite.0),
        cite,
    }

    #[derive(IntoIterator, Debug, Clone, Display, Eq, PartialEq)]
    #[display(fmt = "{}", "_0.join(\" \")")]
    Class(Vec<Cow<'static, str>>) {
        update_el: |class: Class| -> Option<String> {
            if class.0.is_empty() {
                None
            } else {
                let class = class
                    .0
                    .into_iter()
                    .filter(|c| !c.is_empty())
                    .collect::<Vec<Cow<'static, str>>>()
                    .join(" ");
                Some(class)
            }
        },
        class,
    }

    #[derive(Debug, PartialEq, Copy, Clone, From)]
    Color(#[from(forward)] crate::css::Color) {
        update_el: |cols: Color| Some(cols.0),
        color,
    }

    #[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Copy, Clone, From)]
    Cols(u32) {
        update_el: |cols: Cols| Some(cols.0),
        cols,
    }

    #[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Copy, Clone, From)]
    ColSpan(u32) {
        update_el: |col: ColSpan| Some(col.0),
        col_span,
    }

    #[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Clone, From)]
    Content(#[from(forward)] Cow<'static, str>) {
        update_el: |content: Content| Some(content.0),
        content,
    }

    #[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Copy, Clone, From)]
    ContentEditable(bool) {
        update_el: |value: ContentEditable| Some(value.0),
        content_editable,
    }

    #[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Copy, Clone, From)]
    Controls(bool) {
        update_el: |controls: Controls| Some(controls.0.as_at_value()),
        controls,
    }

    Coords {
        update_el: |coords: Coords| Some(coords),
        coords,
    }

    CrossOrigin {
        update_el: |val: CrossOrigin| Some(val),
        cross_origin,
        { custom "crossorigin" }
    }

    #[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Clone, From)]
    Data(#[from(forward)] Cow<'static, str>) {
        update_el: |data: Data| Some(data.0),
        data,
    }

    // TODO: use chrono::DateTime
    #[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Clone, From)]
    DateTime(#[from(forward)] Cow<'static, str>) {
        update_el: |datetime: DateTime| Some(datetime.0),
        date_time,
    }

    Decoding {
        update_el: |decoding: Decoding| Some(decoding),
        decoding,
        { custom "decoding" }
    }

    Loading {
        update_el: |loading: Loading| Some(loading),
        loading,
        { custom "loading" }
    }

    #[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Copy, Clone, From)]
    Default(bool) {
        update_el: |val: Default| Some(val.0.as_at_value()),
        default,
    }

    #[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Copy, Clone, From)]
    Defer(bool) {
        update_el: |val: Defer| Some(val.0.as_at_value()),
        defer,
    }

    Dir {
        update_el: |dir: Dir| Some(dir),
        dir,
    }

    #[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Clone, From)]
    DirName(#[from(forward)] Cow<'static, str>) {
        update_el: |val: DirName| Some(val.0),
        dir_name,
    }

    #[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Copy, Clone, From)]
    Disabled(bool) {
        update_el: |dis: Disabled| Some(dis.0.as_at_value()),
        disabled,
    }

    #[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Clone, From)]
    Download(#[from(forward)] Cow<'static, str>) {
        update_el: |val: Download| Some(val.0),
        download,
    }

    Draggable {
        update_el: |draggable: Draggable| Some(draggable),
        draggable,
    }

    EncType {
        update_el: |enc_type: EncType| Some(enc_type),
        enc_type,
    }

    EnterKeyHint {
        update_el: |val: EnterKeyHint| Some(val),
        enter_key_hint,
        { custom "entry_key_hint" }
    }

    #[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Clone, From)]
    For(#[from(forward)] Vec<Id>) {
        update_el: |f: For| {
            Some(f.0
             .into_iter()
             .map(|id| id.0.to_string())
             .collect::<Vec<String>>()
             .join(" "))
        },
        for_id,
    }

    #[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Clone, From)]
    Form(#[from(forward)] Id) {
        update_el: |form: Form| Some((form.0).0),
        form,
    }

    // TODO: should we change inner type to something like url::Url ?
    #[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Clone, From)]
    FormAction(#[from(forward)] Cow<'static, str>) {
        update_el: |action: FormAction| Some(action.0),
        form_action,
    }

    FormEncType {
        update_el: |val: FormEncType| Some(val),
        form_enc_action,
        { custom "fromenctype" }
    }

    FormMethod {
        update_el: |val: FormMethod| Some(val),
        form_method,
        { custom "frommethod" }
    }

    #[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Copy, Clone, From)]
    FormNoValidate(bool) {
        update_el: |val: FormNoValidate| Some(val.0.as_at_value()),
        form_no_validate,
        { custom "formnovalidate" }
    }

    #[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Clone, From)]
    FormTarget(#[from(forward)] Cow<'static, str>) {
        update_el: |val: FormTarget| Some(val.0),
        form_target,
        { custom " formtarget" }
    }

    #[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Clone, From)]
    Headers(#[from(forward)] Vec<Id>) {
        update_el: |headers: Headers| {
            Some(headers
                .0
                .into_iter()
                .map(|id| id.0.to_string())
                .collect::<Vec<String>>()
                .join(" "))
        },
        headers,
    }

    #[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Copy, Clone, From)]
    Height(u32) {
        update_el: |height: Height| Some(height.0),
        height,
    }

    #[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Copy, Clone, From)]
    Hidden(bool) {
        update_el: |hidden: Hidden| Some(hidden.0.as_at_value()),
        hidden,
    }

    #[derive(Debug, PartialEq, PartialOrd, Copy, Clone, From)]
    High(f32) {
        update_el: |high: High| Some(high.0),
        high,
    }

    #[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Clone, From)]
    Href(#[from(forward)] Cow<'static, str>) {
        update_el: |href: Href| Some(href.0),
        href,
    }

    // TODO: should we change inner type to enum that contains all ISO lang code ?
    #[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Clone, From)]
    HrefLang(#[from(forward)] Cow<'static, str>) {
        update_el: |lang: HrefLang| Some(lang.0),
        href_lang,
    }

    #[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Clone, From)]
    Id(#[from(forward)] Cow<'static, str>) {
        update_el: |id: Id| Some(id.0),
        id,
    }

    // TODO: ImageSizes, ImageSrcSet

    InputMode {
        update_el: |val: InputMode| Some(val),
        input_mode,
        { custom "inputmode" }
    }

    #[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Clone, From)]
    Integrity(#[from(forward)] Cow<'static, str>) {
        update_el: |val: Integrity| Some(val.0),
        integrity,
        { custom "integrity" }
    }

    #[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Clone, From)]
    Is(#[from(forward)] Cow<'static, str>) {
        update_el: |val: Is| Some(val.0),
        is,
        { custom "is" }
    }

    #[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Copy, Clone, From)]
    IsMap(bool) {
        update_el: |is_map: IsMap| Some(is_map.0.as_at_value()),
        is_map,
    }

    #[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Clone, From)]
    ItemId(#[from(forward)] Cow<'static, str>) {
        update_el: |val: ItemId| Some(val.0),
        item_id,
        { custom "itemid" }
    }

    #[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Clone, From)]
    ItemGroup(#[from(forward)] Cow<'static, str>) {
        update_el: |val: ItemGroup| Some(val.0),
        item_group,
        { custom "itemgroup" }
    }

    #[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Clone, From)]
    ItemRef(#[from(forward)] Cow<'static, str>) {
        update_el: |val: ItemRef| Some(val.0),
        item_ref,
        { custom "itemref" }
    }

    #[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Copy, Clone, From)]
    ItemScope(bool) {
        update_el: |val: ItemScope| Some(val.0.as_at_value()),
        item_scope,
        { custom "itemscope" }
    }

    #[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Clone, From)]
    ItemType(#[from(forward)] Cow<'static, str>) {
        update_el: |val: ItemType| Some(val.0),
        item_type,
        { custom "itemtype" }
    }

    Kind {
        update_el: |kind: Kind| Some(kind),
        kind,
    }

    #[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Clone, From)]
    Label(#[from(forward)] Cow<'static, str>) {
        update_el: |label: Label| Some(label.0),
        label,
    }

    // TODO: should we change inner type to enum that contains all ISO lang code ?
    #[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Clone, From)]
    Lang(#[from(forward)] Cow<'static, str>) {
        update_el: |lang: Lang| Some(lang.0),
        lang,
    }

    #[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Clone, From)]
    List(#[from(forward)] Id) {
        update_el: |val: List| Some((val.0).0),
        list,
    }

    #[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Copy, Clone, From)]
    Loop(bool) {
        update_el: |l: Loop| Some(l.0.as_at_value()),
        looping,
    }

    #[derive(Debug, PartialEq, PartialOrd, Copy, Clone, From)]
    Low(f32) {
        update_el: |low: Low| Some(low.0),
        low,
    }

    #[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Clone, From)]
    Manifest(#[from(forward)] Cow<'static, str>) {
        update_el: |val: Manifest| Some(val.0),
        manifest,
        { custom "manifest" }
    }

    Max {
        update_el: |max: Max| Some(max),
        max,
    }

    #[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Copy, Clone)]
    MaxLength(i32) {
        update_el: |val: MaxLength| Some(val.0),
        max_length,
    }

    // TODO: should we use enum that work the html expect ?
    #[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Clone, From)]
    Media(#[from(forward)] Cow<'static, str>) {
        update_el: |media: Media| Some(media.0),
        media,
    }

    Method {
        update_el: |val: Method| Some(val),
        method,
    }

    Min {
        update_el: |min: Min| Some(min),
        min,
    }

    #[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Copy, Clone)]
    MinLength(i32) {
        update_el: |val: MinLength| Some(val.0),
        min_length,
    }

    #[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Copy, Clone, From)]
    Multiple(bool) {
        update_el: |multiple: Multiple| Some(multiple.0.as_at_value()),
        multiple,
    }

    #[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Copy, Clone, From)]
    Muted(bool) {
        update_el: |muted: Muted| Some(muted.0.as_at_value()),
        muted,
    }

    #[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Clone, From)]
    Name(#[from(forward)] Cow<'static, str>) {
        update_el: |name: Name| Some(name.0),
        name,
    }

    #[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Copy, Clone, From)]
    NoModule(bool) {
        update_el: |val: NoModule| Some(val.0.as_at_value()),
        no_module,
        { custom "nomodule" }
    }

    #[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Clone, From)]
    Nonce(#[from(forward)] Cow<'static, str>) {
        update_el: |val: Nonce| Some(val.0),
        nonce,
        { custom "nonce" }
    }

    #[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Copy, Clone, From)]
    NoValidate(bool) {
        update_el: |no_validate: NoValidate| Some(no_validate.0.as_at_value()),
        no_validate,
    }

    #[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Copy, Clone, From)]
    Open(bool) {
        update_el: |open: Open| Some(open.0.as_at_value()),
        open,
    }

    #[derive(Debug, PartialEq, PartialOrd, Copy, Clone, From)]
    Optimum(f32) {
        update_el: |optimum: Optimum| Some(optimum.0),
        optimum,
    }

    Pattern {
        update_el: |pattern: Pattern| Some(pattern),
        pattern,
    }

    #[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Clone, From)]
    Placeholder(#[from(forward)] Cow<'static, str>) {
        update_el: |placeholder: Placeholder| Some(placeholder.0),
        placeholder,
    }

    #[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Copy, Clone, From)]
    PlaysInLine(bool) {
        update_el: |val: PlaysInLine| Some(val.0.as_at_value()),
        plays_in_line,
        { custom "playsinline" }
    }

    #[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Clone, From)]
    Poster(#[from(forward)] Cow<'static, str>) {
        update_el: |poster: Poster| Some(poster.0),
        poster,
    }

    Preload {
        update_el: |preload: Preload| Some(preload),
        preload,
    }

    #[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Copy, Clone, From)]
    ReadOnly(bool) {
        update_el: |read_only: ReadOnly| Some(read_only.0.as_at_value()),
        read_only,
    }

    ReferrerPolicy {
        update_el: |val: ReferrerPolicy| Some(val),
        referrer_policy,
        { custom "referrerpolicy" }
    }

    #[derive(Debug, Eq, PartialEq, Clone, From)]
    Rel(#[from(forward)] indexmap::IndexSet<RelValue>) {
        update_el: |rel: Rel| {
            let val = rel
                .0
                .into_iter()
                .map(|val| val.to_string())
                .collect::<Vec<String>>()
                .join(" ");
            Some(val)
        },
        rel,
    }

    #[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Copy, Clone, From)]
    Required(bool) {
        update_el: |required: Required| Some(required.0.as_at_value()),
        required,
    }

    #[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Copy, Clone, From)]
    Reversed(bool) {
        update_el: |reversed: Reversed| Some(reversed.0.as_at_value()),
        reversed,
    }

    #[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Copy, Clone)]
    Rows(u32) {
        update_el: |rows: Rows| Some(rows.0),
        rows,
    }

    #[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Copy, Clone, From)]
    RowSpan(u32) {
        update_el: |row: RowSpan| Some(row.0),
        row_span,
    }

    #[derive(Debug, Eq, PartialEq, Clone, From)]
    Sandbox(#[from(forward)] IndexSet<SandboxValue>) {
        update_el: |sand_box: Sandbox| {
            let vals = sand_box
                .0
                .into_iter()
                .map(|val| val.to_string())
                .collect::<Vec<String>>()
                .join(" ");
            Some(vals)
        },
        sandbox,
    }

    Scope {
        update_el: |scope: Scope| Some(scope),
        scope,
    }

    #[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Copy, Clone, From)]
    Selected(bool) {
        update_el: |selected: Selected| Some(selected.0.as_at_value()),
        selected,
    }

    Shape {
        update_el: |val: Shape| Some(val),
        shape,
    }

    #[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Copy, Clone)]
    Size(u32) {
        update_el: |size: Size| Some(size.0),
        size,
    }

    // TODO: add Sizes

    #[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Clone, From)]
    Slot(#[from(forward)] Cow<'static, str>) {
        update_el: |val: Slot| Some(val.0),
        slot,
        { custom "slot" }
    }

    #[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Copy, Clone, From)]
    Span(u32) {
        update_el: |span: Span| Some(span.0),
        span,
    }

    #[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Copy, Clone, From)]
    SpellCheck(bool) {
        update_el: |spell_check: SpellCheck| Some(spell_check.0),
        spell_check,
    }

    // TODO: should we change inner type to something like url::Url ?
    #[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Clone, From)]
    Src(#[from(forward)] Cow<'static, str>) {
        update_el: |src: Src| Some(src.0),
        src,
    }

    #[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Clone, From)]
    SrcDoc(#[from(forward)] Cow<'static, str>) {
        update_el: |src_doc: SrcDoc| Some(src_doc.0),
        src_doc,
    }

    // TODO: should we change inner type to enum that contains all ISO lang code ?
    #[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Clone, From)]
    SrcLang(#[from(forward)] Cow<'static, str>) {
        update_el: |src_lang: SrcLang| Some(src_lang.0),
        src_lang,
    }

    // TODO: should we change inner type to something like url::Url ?
    #[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Clone, From)]
    SrcSet(#[from(forward)] Cow<'static, str>) {
        update_el: |src_set: SrcSet| Some(src_set.0),
        src_set,
    }

    #[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Copy, Clone, From)]
    Start(i32) {
        update_el: |start: Start| Some(start.0),
        start,
    }

    Step {
        update_el: |step: Step| Some(step),
        step,
    }

    #[derive(Debug, PartialEq, Clone, From)]
    Style(#[from(forward)] crate::css::Style) {
        update_el: |style: Style| style.0.to_css(),
        style,
    }

    #[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Copy, Clone, From)]
    TabIndex(i32) {
        update_el: |tab_index: TabIndex| Some(tab_index.0),
        tab_index,
    }

    Target {
        update_el: |target: Target| Some(target),
        target,
    }

    #[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Clone, From)]
    Title(#[from(forward)] Cow<'static, str>) {
        update_el: |title: Title| Some(title.0),
        title,
    }

    Translate {
        update_el: |val: Translate| Some(val),
        translate,
    }

    Type {
        update_el: |ty: Type| Some(ty),
    }

    #[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Clone, From)]
    UseMap(#[from(forward)] Cow<'static, str>) {
        update_el: |use_map: UseMap| Some(use_map.0),
        use_map,
    }

    #[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Clone, From)]
    Value(#[from(forward)] Cow<'static, str>) {
        update_el: |value: Value| Some(value.0),
        value,
    }

    #[derive(Debug, PartialEq, PartialOrd, Copy, Clone, From)]
    Width(u32) {
        update_el: |val: Width| Some(val.0),
        width,
    }

    Wrap {
        update_el: |wrap: Wrap| Some(wrap),
        wrap,
    }

    Cx {
        update_el: |cx: Cx| Some(cx),
        cx,
    }

    Cy {
        update_el: |cy: Cy| Some(cy),
        cy,
    }

    R {
        update_el: |r: R| Some(r),
        r,
    }

    Rx {
        update_el: |rx: Rx| Some(rx),
        rx,
    }

    Ry {
        update_el: |ry: Ry| Some(ry),
        ry,
    }

    X {
        update_el: |x: X| Some(x),
        x,
    }

    Y {
        update_el: |y: Y| Some(y),
        y,
    }

    ViewBox {
        update_el: |view_box: ViewBox| Some(view_box),
    }
}

impl From<u32> for Size {
    fn from(source: u32) -> Self {
        match source {
            // NOTE: we do this, since size doesn't accept negative value and
            // must be greater than 0
            x if x >= 1 => Self(x),
            _ => Self(1),
        }
    }
}

impl From<u32> for Rows {
    fn from(source: u32) -> Self {
        match source {
            // NOTE: we do this, since rows doesn't accept negative value and
            // must be greater than 0
            x if x >= 1 => Rows(x),
            _ => Rows(1),
        }
    }
}

impl From<i32> for MaxLength {
    fn from(source: i32) -> Self {
        match source {
            // NOTE: we do this, since max_length doesn't accept negative value
            x if x >= 0 => MaxLength(x),
            _ => MaxLength(0),
        }
    }
}

impl From<i32> for MinLength {
    fn from(source: i32) -> Self {
        match source {
            // NOTE: we do this, since max_length doesn't accept negative value
            x if x >= 0 => MinLength(x),
            _ => MinLength(0),
        }
    }
}

impl From<Id> for Headers {
    fn from(source: Id) -> Self {
        Headers(vec![source])
    }
}

impl From<Id> for For {
    fn from(source: Id) -> Self {
        For(vec![source])
    }
}

impl Class {
    pub fn contains(&self, class: &Class) -> bool {
        class.0.iter().all(|c| self.0.iter().any(|s| s == c))
    }

    pub fn push(&mut self, class: impl Into<Class>) -> &mut Self {
        self.0.extend(class.into());
        self
    }
}

impl From<Class> for Cow<'static, str> {
    fn from(source: Class) -> Self {
        source.0.join(" ").into()
    }
}

impl Extend<Class> for Class {
    fn extend<T: IntoIterator<Item = Class>>(&mut self, iter: T) {
        for class in iter {
            self.0.extend(class.0)
        }
    }
}

impl Extend<&'static str> for Class {
    fn extend<T: IntoIterator<Item = &'static str>>(&mut self, iter: T) {
        for class in iter {
            let class = Class::from(class);
            self.0.extend(class);
        }
    }
}

impl Extend<String> for Class {
    fn extend<T: IntoIterator<Item = String>>(&mut self, iter: T) {
        for class in iter {
            let class = Class::from(class);
            self.0.extend(class);
        }
    }
}

impl Extend<Option<&'static str>> for Class {
    fn extend<T: IntoIterator<Item = Option<&'static str>>>(&mut self, iter: T) {
        for class in iter {
            let class = Class::from(class);
            self.0.extend(class);
        }
    }
}

impl Extend<Option<String>> for Class {
    fn extend<T: IntoIterator<Item = Option<String>>>(&mut self, iter: T) {
        for class in iter {
            let class = Class::from(class);
            self.0.extend(class);
        }
    }
}

impl From<&'static str> for Class {
    fn from(source: &'static str) -> Self {
        if source.is_empty() {
            Class(vec![])
        } else {
            Class(
                source
                    .split(char::is_whitespace)
                    .map(|s| s.into())
                    .collect(),
            )
        }
    }
}

impl From<String> for Class {
    fn from(source: String) -> Self {
        if source.is_empty() {
            Class(vec![])
        } else {
            Class(
                source
                    .split(char::is_whitespace)
                    .map(|s| s.to_string())
                    .map(|s| Cow::from(s))
                    .collect::<Vec<_>>(),
            )
        }
    }
}

impl From<Option<&'static str>> for Class {
    fn from(source: Option<&'static str>) -> Self {
        match source {
            Some(class) => class.into(),
            _ => Class(vec![]),
        }
    }
}

impl From<Option<String>> for Class {
    fn from(source: Option<String>) -> Self {
        match source {
            Some(class) => class.into(),
            _ => Class(vec![]),
        }
    }
}

impl From<Vec<&'static str>> for Class {
    fn from(source: Vec<&'static str>) -> Self {
        let mut class = Class(vec![]);
        class.extend(source);
        class
    }
}

impl From<Vec<String>> for Class {
    fn from(source: Vec<String>) -> Self {
        let mut class = Class(vec![]);
        class.extend(source);
        class
    }
}

impl From<Vec<Option<&'static str>>> for Class {
    fn from(source: Vec<Option<&'static str>>) -> Self {
        let mut class = Class(vec![]);
        class.extend(source);
        class
    }
}

impl From<Vec<Option<String>>> for Class {
    fn from(source: Vec<Option<String>>) -> Self {
        let mut class = Class(vec![]);
        class.extend(source);
        class
    }
}

#[derive(Debug, Copy, PartialEq, PartialOrd, Clone, From, Display)]
pub enum Loading {
    #[display(fmt = "lazy")]
    Lazy,
    #[display(fmt = "eager")]
    Eager,
}

#[derive(Debug, Copy, PartialEq, PartialOrd, Clone, From, Display)]
pub enum Decoding {
    #[display(fmt = "sync")]
    Sync,
    #[display(fmt = "async")]
    Async,
    #[display(fmt = "auto")]
    Auto,
}

#[derive(Debug, Copy, PartialEq, PartialOrd, Clone, From, Display)]
pub enum AutoCapitalize {
    #[display(fmt = "on")]
    On,
    #[display(fmt = "off")]
    Off,
    #[display(fmt = "none")]
    None,
    #[display(fmt = "sentences")]
    Sentences,
    #[display(fmt = "words")]
    Words,
    #[display(fmt = "characters")]
    Characters,
}

#[derive(Debug, Clone, From, Display)]
pub enum Pattern {
    #[from]
    RegexpStr(Cow<'static, str>),
    #[from]
    Regexp(regex::Regex),
}

impl From<String> for Pattern {
    fn from(source: String) -> Self {
        Self::RegexpStr(source.into())
    }
}

impl From<&'static str> for Pattern {
    fn from(source: &'static str) -> Self {
        Self::RegexpStr(source.into())
    }
}

pub fn type_(value: impl Into<Type>) -> Type {
    value.into()
}

#[derive(Debug, PartialEq, PartialOrd, Clone, From, Display)]
pub enum Type {
    #[from]
    MediaType(mime::Mime),
    #[display(fmt = "button")]
    Button,
    #[display(fmt = "submit")]
    Submit,
    #[display(fmt = "reset")]
    Reset,
    #[display(fmt = "checkbox")]
    Checkbox,
    #[display(fmt = "color")]
    Color,
    #[display(fmt = "date")]
    Date,
    #[display(fmt = "datetime-local")]
    DatetimeLocal,
    #[display(fmt = "email")]
    Email,
    #[display(fmt = "file")]
    File,
    #[display(fmt = "hidden")]
    Hidden,
    #[display(fmt = "image")]
    Image,
    #[display(fmt = "month")]
    Month,
    #[display(fmt = "number")]
    Number,
    #[display(fmt = "password")]
    Password,
    #[display(fmt = "radio")]
    Radio,
    #[display(fmt = "range")]
    Range,
    #[display(fmt = "search")]
    Search,
    #[display(fmt = "tel")]
    Tel,
    #[display(fmt = "text")]
    Text,
    #[display(fmt = "time")]
    Time,
    #[display(fmt = "url")]
    Url,
    #[display(fmt = "week")]
    Week,
    #[display(fmt = "list")]
    List,
    #[display(fmt = "context")]
    Context,
    #[display(fmt = "toolbar")]
    Toolbar,
}

#[derive(Debug, PartialEq, PartialOrd, Copy, Clone, From, Display)]
pub enum Max {
    #[from]
    Number(f64),
    #[from]
    DateTime(chrono::NaiveDateTime),
    #[from]
    Date(chrono::NaiveDate),
    #[from]
    Time(chrono::NaiveTime),
}

#[derive(Debug, PartialOrd, PartialEq, Copy, Clone, From, Display)]
pub enum Min {
    #[from]
    Number(f64),
    #[from]
    DateTime(chrono::NaiveDateTime),
    #[from]
    Date(chrono::NaiveDate),
    #[from]
    Time(chrono::NaiveTime),
}

#[derive(Debug, Eq, Ord, PartialOrd, PartialEq, Copy, Clone, From, Display)]
pub enum AutoComplete {
    #[display(fmt = "on")]
    On,
    #[display(fmt = "off")]
    Off,
}

#[derive(Debug, PartialOrd, PartialEq, Clone, From, Display)]
pub enum Coords {
    #[from]
    #[display(
        fmt = "{},{},{},{}",
        "top_left.0",
        "top_left.1",
        "bottom_right.0",
        "bottom_right.1"
    )]
    Rect {
        top_left: (f32, f32),
        bottom_right: (f32, f32),
    },
    #[from]
    #[display(fmt = "{},{},{}", x, y, radius)]
    Circle { x: f32, y: f32, radius: f32 },
    #[from]
    #[display(
        fmt = "{}",
        "edges.iter().map(|(x, y)| format!(\"{},{}\", x, y)).collect::<Vec<String>>().join(\",\")"
    )]
    Polygon { edges: Vec<(f32, f32)> },
}

#[derive(Debug, Eq, Ord, PartialOrd, PartialEq, Copy, Clone, From, Display)]
pub enum CrossOrigin {
    #[display(fmt = "anonymous")]
    Anonymous,
    #[display(fmt = "use-credentials")]
    UseCredentials,
}

#[derive(Debug, PartialEq, PartialOrd, Copy, Clone, From, Display)]
pub enum Dir {
    #[display(fmt = "ltr")]
    LTR,
    #[display(fmt = "rtl")]
    RTL,
    #[display(fmt = "auto")]
    Auto,
}

#[derive(Debug, PartialEq, PartialOrd, Copy, Clone, From, Display)]
pub enum Draggable {
    #[display(fmt = "true")]
    True,
    #[display(fmt = "false")]
    False,
}

#[derive(Debug, PartialEq, PartialOrd, Copy, Clone, From, Display)]
pub enum EnterKeyHint {
    #[display(fmt = "enter")]
    Enter,
    #[display(fmt = "done")]
    Done,
    #[display(fmt = "go")]
    Go,
    #[display(fmt = "next")]
    Next,
    #[display(fmt = "previous")]
    Previous,
    #[display(fmt = "search")]
    Search,
    #[display(fmt = "send")]
    Send,
}

#[derive(Debug, PartialEq, PartialOrd, Copy, Clone, From, Display)]
pub enum FormEncType {
    #[display(fmt = "application/x-www-form-urlencoded")]
    Application,
    #[display(fmt = "multipart/form-data")]
    Multipart,
    #[display(fmt = "text/plain")]
    Text,
}

#[derive(Debug, PartialEq, PartialOrd, Copy, Clone, From, Display)]
pub enum EncType {
    #[display(fmt = "application/x-www-form-urlencoded")]
    Application,
    #[display(fmt = "multipart/form-data")]
    Multipart,
    #[display(fmt = "text/plain")]
    Text,
}

#[derive(Debug, Display, Eq, PartialEq, Ord, PartialOrd, Copy, Clone, From)]
pub enum Kind {
    #[display(fmt = "captions")]
    Captions,
    #[display(fmt = "chapters")]
    Chapters,
    #[display(fmt = "descriptions")]
    Descriptions,
    #[display(fmt = "metadata")]
    Metadata,
    #[display(fmt = "subtitle")]
    Subtitle,
}

#[derive(Debug, Display, Eq, PartialEq, Ord, PartialOrd, Copy, Clone, From)]
pub enum InputMode {
    #[display(fmt = "none")]
    None,
    #[display(fmt = "text")]
    Text,
    #[display(fmt = "tel")]
    Tel,
    #[display(fmt = "email")]
    Email,
    #[display(fmt = "url")]
    Url,
    #[display(fmt = "numeric")]
    Numeric,
    #[display(fmt = "decimal")]
    Decimal,
    #[display(fmt = "search")]
    Search,
}

#[derive(Debug, Display, Eq, PartialEq, Ord, PartialOrd, Copy, Clone, From)]
pub enum FormMethod {
    #[display(fmt = "get")]
    Get,
    #[display(fmt = "post")]
    Post,
    #[display(fmt = "dialog")]
    Dialog,
}

#[derive(Debug, Display, Eq, PartialEq, Ord, PartialOrd, Copy, Clone, From)]
pub enum Method {
    #[display(fmt = "get")]
    Get,
    #[display(fmt = "post")]
    Post,
}

#[derive(Debug, Display, Eq, PartialEq, Ord, PartialOrd, Copy, Clone, From)]
pub enum ReferrerPolicy {
    #[display(fmt = "no-referrer")]
    NoReferrer,
    #[display(fmt = "no-referrer-when-downgrade")]
    NoReferrerWhenDowngrade,
    #[display(fmt = "same-origin")]
    SameOrigin,
    #[display(fmt = "origin")]
    Origin,
    #[display(fmt = "strict-origin")]
    StrictOrigin,
    #[display(fmt = "origin-when-cross-origin")]
    OriginWhenCrossOrigin,
    #[display(fmt = "strict-origin-when-cross-origin")]
    StrictOriginWhenCrossOrigin,
    #[display(fmt = "unsafe-url")]
    UnsafeUrl,
}

#[derive(Debug, Display, Eq, PartialEq, Ord, PartialOrd, Copy, Clone, From)]
pub enum Preload {
    #[display(fmt = "auto")]
    Auto,
    #[display(fmt = "metadata")]
    Metadata,
    #[display(fmt = "none")]
    None,
}

#[derive(Debug, Display, PartialEq, Eq, Hash, Copy, Clone, From)]
pub enum SandboxValue {
    #[display(fmt = "allow-forms")]
    AllowForms,
    #[display(fmt = "allow-modals")]
    AllowModals,
    #[display(fmt = "allow-orientation-lock")]
    AllowOrientationLock,
    #[display(fmt = "allow-pointer-lock")]
    AllowPointerLock,
    #[display(fmt = "allow-popups")]
    AllowPopups,
    #[display(fmt = "allow-popups-to-escape-sandbox")]
    AllowPopupsToEscapeSandbox,
    #[display(fmt = "allow-presentation")]
    AllowPresentation,
    #[display(fmt = "allow-same-origin")]
    AllowSameOrigin,
    #[display(fmt = "allow-scripts")]
    AllowScripts,
    #[display(fmt = "allow-top-navigation")]
    AllowTopNavigation,
}

impl From<SandboxValue> for Sandbox {
    fn from(source: SandboxValue) -> Self {
        let mut set = IndexSet::default();
        set.insert(source);
        Self(set)
    }
}

#[derive(Debug, Display, PartialEq, Eq, Hash, Copy, Clone, From)]
pub enum RelValue {
    #[display(fmt = "alternate")]
    Alternate,
    #[display(fmt = "author")]
    Author,
    #[display(fmt = "bookmark")]
    Bookmark,
    #[display(fmt = "canonical")]
    Canonical,
    #[display(fmt = "dns-prefetch")]
    DnsPrefetch,
    #[display(fmt = "external")]
    External,
    #[display(fmt = "help")]
    Help,
    #[display(fmt = "icon")]
    Icon,
    #[display(fmt = "license")]
    License,
    #[display(fmt = "modulepreload")]
    ModulePreload,
    #[display(fmt = "next")]
    Next,
    #[display(fmt = "nofollow")]
    NoFollow,
    #[display(fmt = "noopener")]
    NoOpener,
    #[display(fmt = "noreferrer")]
    NoReferrer,
    #[display(fmt = "opener")]
    Opener,
    #[display(fmt = "pingback")]
    PingBack,
    #[display(fmt = "preconnect")]
    Preconnect,
    #[display(fmt = "prefetch")]
    Prefetch,
    #[display(fmt = "preload")]
    Preload,
    #[display(fmt = "prerender")]
    Prerender,
    #[display(fmt = "prev")]
    Prev,
    #[display(fmt = "search")]
    Search,
    #[display(fmt = "stylesheet")]
    Stylesheet,
    #[display(fmt = "tag")]
    Tag,
    #[display(fmt = "manifest")]
    Manifest,
}

impl From<RelValue> for Rel {
    fn from(source: RelValue) -> Self {
        let mut set = IndexSet::default();
        set.insert(source);
        Self(set)
    }
}

#[derive(Debug, Display, Eq, PartialEq, Ord, PartialOrd, Copy, Clone, From)]
pub enum Scope {
    #[display(fmt = "col")]
    Col,
    #[display(fmt = "row")]
    Row,
    #[display(fmt = "colgroup")]
    ColGroup,
    #[display(fmt = "rowgroup")]
    RowGroup,
}

#[derive(Debug, Display, Eq, PartialEq, Ord, PartialOrd, Copy, Clone, From)]
pub enum Shape {
    #[display(fmt = "default")]
    Default,
    #[display(fmt = "rect")]
    Rect,
    #[display(fmt = "circle")]
    Circle,
    #[display(fmt = "poly")]
    Poly,
}

#[derive(Debug, Display, PartialEq, PartialOrd, Copy, Clone, From)]
pub enum Step {
    #[from]
    Number(f64),
    #[display(fmt = "any")]
    Any,
}

#[derive(Debug, Display, PartialEq, PartialOrd, Clone, From)]
pub enum Translate {
    #[display(fmt = "yes")]
    Yes,
    #[display(fmt = "no")]
    No,
}

#[derive(Debug, Display, PartialEq, PartialOrd, Clone, From)]
pub enum Target {
    #[display(fmt = "_blank")]
    Blank,
    #[display(fmt = "_self")]
    Self_,
    #[display(fmt = "_parent")]
    Parent,
    #[display(fmt = "_top")]
    Top,
    #[from]
    #[display(fmt = "{}", _0)]
    FrameName(Cow<'static, str>),
}

#[derive(Debug, Display, PartialEq, PartialOrd, Clone, From)]
pub enum Wrap {
    #[display(fmt = "soft")]
    Soft,
    #[display(fmt = "hard")]
    Hard,
}

// TODO: check if value and name are valid
#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Clone)]
pub struct Custom {
    name: Cow<'static, str>,
    value: Cow<'static, str>,
}

pub fn custom_data(
    name: impl Into<Cow<'static, str>>,
    value: impl Into<Cow<'static, str>>,
) -> Custom {
    Custom {
        name: name.into(),
        value: value.into(),
    }
}

impl<Msg> UpdateEl<Msg> for Custom {
    fn update_el(self, el: &mut El<Msg>) {
        el.attrs.add(At::Custom(self.name), self.value);
    }
}

// https://www.w3.org/TR/css-values-4/#lengths
#[derive(Clone, Debug, Copy, PartialEq, Display, From)]
pub enum Cx {
    #[from]
    Length(Length),
    #[from(forward)]
    Percent(Percent),
}

// https://www.w3.org/TR/css-values-4/#lengths
#[derive(Clone, Debug, Copy, PartialEq, Display, From)]
pub enum Cy {
    #[from]
    Length(Length),
    #[from(forward)]
    Percent(Percent),
}

// https://www.w3.org/TR/css-values-4/#lengths
#[derive(Clone, Debug, Copy, PartialEq, Display, From)]
pub enum R {
    #[from]
    Length(Length),
    #[from(forward)]
    Percent(Percent),
}

// https://www.w3.org/TR/css-values-4/#lengths
#[derive(Clone, Debug, Copy, PartialEq, Display, From)]
pub enum Rx {
    #[from]
    Auto,
    #[from]
    Length(Length),
    #[from(forward)]
    Percent(Percent),
}

// https://www.w3.org/TR/css-values-4/#lengths
#[derive(Clone, Debug, Copy, PartialEq, Display, From)]
pub enum Ry {
    #[from]
    Auto,
    #[from]
    Length(Length),
    #[from(forward)]
    Percent(Percent),
}

// https://www.w3.org/TR/css-values-4/#lengths
#[derive(Clone, Debug, Copy, PartialEq, Display, From)]
pub enum X {
    #[from]
    Length(Length),
    #[from(forward)]
    Percent(Percent),
}

// https://www.w3.org/TR/css-values-4/#lengths
#[derive(Clone, Debug, Copy, PartialEq, Display, From)]
pub enum Y {
    #[from]
    Length(Length),
    #[from(forward)]
    Percent(Percent),
}

#[derive(Clone, Debug, Copy, PartialOrd, PartialEq, Display, From)]
#[display(fmt = "{}, {}, {}, {}", min_x, min_y, width, height)]
pub struct ViewBox {
    pub min_x: f32,
    pub min_y: f32,
    pub width: f32,
    pub height: f32,
}

impl ViewBox {
    pub fn new(min_x: f32, min_y: f32, width: f32, height: f32) -> Self {
        ViewBox {
            min_x,
            min_y,
            width,
            height,
        }
    }
}

pub fn view_box(min_x: f32, min_y: f32, width: f32, height: f32) -> ViewBox {
    ViewBox::new(min_x, min_y, width, height)
}
