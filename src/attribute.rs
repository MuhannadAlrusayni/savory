//! Types and functions used to create HTML attributes.

use crate::{css::unit::*, prelude::*};
use seed::prelude::{AsAtValue, At};
use std::borrow::Cow;

macro_rules! create_attributes {
    ( @shortcut $name:ident $fn_name:ident $(,)? ) => {
        pub fn $fn_name(value: impl Into<$name>) -> $name {
            value.into()
        }
    };
    ( @shortcut $name:ident update_el $expr:expr  $(;)? ) => {
        impl<Msg> UpdateEl<Msg> for $name {
            fn update_el(self, el: &mut El<Msg>) {
                let closure = $expr;
                if let Some(val) = closure(self) {
                    el.attrs.add(At::$name, val);
                }
            }
        }
    };
    ( $(
        $( #[$attrs:meta] )*
        $name:ident $( ( $( $( #[$ty_attrs:meta] )* $ty:ty )* ) )? {
            update_el: $expr:expr,
            $( $fn_name:ident, )?
            // $( $attr_name:ident $(: $shortcuts:tt )* $(,)? )*
        }
    )* ) => {
        $(
            $( #[$attrs] )*
            $( pub struct $name( $( $( #[$ty_attrs] )* $ty )* ); )?
            // $(
            //     create_attributes!(@shortcut $name $attr_name $($shortcuts)*);
            // )*
            create_attributes!(@shortcut $name update_el $expr);
            $( create_attributes!(@shortcut $name $fn_name); )?
        )*
    };
}

create_attributes! {
    // TODO: should we change inner type to something like mime::Mime ?
    #[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Clone, From)]
    Accept(#[from(forward)] Cow<'static, str>) {
        update_el: |accept: Self| Some(accept.0),
        accept,
    }

    // TODO: should we change inner type to something like murdoch::CharacterSetEnum ?
    #[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Clone, From)]
    AcceptCharset(#[from(forward)] Cow<'static, str>) {
        update_el: |charset: Self| Some(charset.0),
        accept_charset,
    }

    #[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Clone, From)]
    AccessKey(#[from(forward)] Cow<'static, str>) {
        update_el: |key: Self| Some(key.0),
        access_key,
    }

    // TODO: should we change inner type to something like url::Url ?
    #[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Clone, From)]
    Action(#[from(forward)] Cow<'static, str>) {
        update_el: |action: Self| Some(action.0),
        action,
    }

    #[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Clone, From)]
    Alt(#[from(forward)] Cow<'static, str>) {
        update_el: |alt: Self| Some(alt.0),
        alt,
    }

    #[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Copy, Clone, From)]
    AutoFocus(bool) {
        update_el: |auto: Self| Some(auto.0.as_at_value()),
        auto_focus,
    }

    #[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Copy, Clone, From)]
    AutoPlay(bool) {
        update_el: |auto: Self| Some(auto.0.as_at_value()),
        auto_play,
    }

    #[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Copy, Clone, From)]
    Checked(bool) {
        update_el: |checked: Self| Some(checked.0.as_at_value()),
        checked,
    }

    // TODO: should we change inner type to something like url::Url ?
    #[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Clone, From)]
    Cite(#[from(forward)] Cow<'static, str>) {
        update_el: |cite: Self| Some(cite.0),
        cite,
    }

    #[derive(Debug, Clone)]
    Class(Vec<Cow<'static, str>>) {
        update_el: |class: Self| -> Option<String> {
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

    #[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Copy, Clone, From)]
    Cols(usize) {
        update_el: |cols: Self| Some(cols.0),
        cols,
    }

    #[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Copy, Clone, From)]
    Rows(usize) {
        update_el: |rows: Self| Some(rows.0),
        rows,
    }

    #[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Copy, Clone, From)]
    Span(usize) {
        update_el: |span: Self| Some(span.0),
        span,
    }

    #[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Copy, Clone, From)]
    ColSpan(usize) {
        update_el: |col: Self| Some(col.0),
        col_span,
    }

    #[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Copy, Clone, From)]
    RowSpan(usize) {
        update_el: |row: Self| Some(row.0),
        row_span,
    }

    #[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Copy, Clone, From)]
    ContentEditable(bool) {
        update_el: |value: Self| Some(value.0),
        content_editable,
    }

    #[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Copy, Clone, From)]
    Controls(bool) {
        update_el: |controls: Self| Some(controls.0),
        controls,
    }

    // TODO: use chrono::DateTime
    #[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Clone, From)]
    DateTime(#[from(forward)] Cow<'static, str>) {
        update_el: |datetime: Self| Some(datetime.0),
        date_time,
    }

    #[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Copy, Clone, From)]
    Disabled(bool) {
        update_el: |dis: Self| Some(dis.0.as_at_value()),
        disabled,
    }

    #[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Clone, From)]
    For(#[from(forward)] Cow<'static, str>) {
        update_el: |f: Self| Some(f.0),
        for_id,
    }

    #[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Clone, From)]
    Form(#[from(forward)] Cow<'static, str>) {
        update_el: |form: Self| Some(form.0),
        form,
    }

    #[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Clone, From)]
    Headers(#[from(forward)] Cow<'static, str>) {
        update_el: |headers: Self| Some(headers.0),
        headers,
    }

    // TODO: should we change inner type to something like url::Url ?
    #[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Clone, From)]
    FormAction(#[from(forward)] Cow<'static, str>) {
        update_el: |action: Self| Some(action.0),
        form_action,
    }

    #[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Copy, Clone, From)]
    Hidden(bool) {
        update_el: |hidden: Self| Some(hidden.0.as_at_value()),
        hidden,
    }

    #[derive(Debug, PartialEq, PartialOrd, Copy, Clone, From)]
    High(f32) {
        update_el: |high: Self| Some(high.0),
        high,
    }

    #[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Copy, Clone, From)]
    Height(usize) {
        update_el: |height: Self| Some(height.0),
        height,
    }

    #[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Copy, Clone, From)]
    Width(usize) {
        update_el: |width: Self| Some(width.0),
        width,
    }

    #[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Clone, From)]
    Href(#[from(forward)] Cow<'static, str>) {
        update_el: |href: Self| Some(href.0),
        href,
    }

    // TODO: should we change inner type to enum that contains all ISO lang code ?
    #[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Clone, From)]
    HrefLang(#[from(forward)] Cow<'static, str>) {
        update_el: |lang: Self| Some(lang.0),
        href_lang,
    }

    #[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Clone, From)]
    Id(#[from(forward)] Cow<'static, str>) {
        update_el: |id: Self| Some(id.0),
        id,
    }

    #[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Copy, Clone, From)]
    IsMap(bool) {
        update_el: |is_map: Self| Some(is_map.0.as_at_value()),
        is_map,
    }

    #[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Clone, From)]
    Label(#[from(forward)] Cow<'static, str>) {
        update_el: |label: Self| Some(label.0),
        label,
    }

    // TODO: should we change inner type to enum that contains all ISO lang code ?
    #[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Clone, From)]
    Lang(#[from(forward)] Cow<'static, str>) {
        update_el: |lang: Self| Some(lang.0),
        lang,
    }

    #[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Clone, From)]
    List(#[from(forward)] Cow<'static, str>) {
        update_el: |list: Self| Some(list.0),
        list,
    }

    #[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Copy, Clone, From)]
    Loop(bool) {
        update_el: |l: Self| Some(l.0.as_at_value()),
        looping,
    }

    #[derive(Debug, PartialEq, PartialOrd, Copy, Clone, From)]
    Low(f32) {
        update_el: |low: Self| Some(low.0),
        low,
    }

    #[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Copy, Clone, From)]
    MaxLength(usize) {
        update_el: |max_len: Self| Some(max_len.0),
        max_length,
    }

    // TODO: should we use enum that work the html expect ?
    #[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Clone, From)]
    Media(#[from(forward)] Cow<'static, str>) {
        update_el: |media: Self| Some(media.0),
        media,
    }

    #[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Copy, Clone, From)]
    Multiple(bool) {
        update_el: |multiple: Self| Some(multiple.0.as_at_value()),
        multiple,
    }

    #[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Copy, Clone, From)]
    Muted(bool) {
        update_el: |muted: Self| Some(muted.0.as_at_value()),
        muted,
    }

    #[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Clone, From)]
    Name(#[from(forward)] Cow<'static, str>) {
        update_el: |name: Self| Some(name.0),
        name,
    }

    #[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Copy, Clone, From)]
    NoValidate(bool) {
        update_el: |no_validate: Self| Some(no_validate.0.as_at_value()),
        no_validate,
    }

    #[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Copy, Clone, From)]
    Open(bool) {
        update_el: |open: Self| Some(open.0.as_at_value()),
        open,
    }

    #[derive(Debug, PartialEq, PartialOrd, Copy, Clone, From)]
    Optimum(f32) {
        update_el: |optimum: Self| Some(optimum.0),
        optimum,
    }

    #[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Clone, From)]
    Placeholder(#[from(forward)] Cow<'static, str>) {
        update_el: |placeholder: Self| Some(placeholder.0),
        placeholder,
    }

    #[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Clone, From)]
    Poster(#[from(forward)] Cow<'static, str>) {
        update_el: |poster: Self| Some(poster.0),
        poster,
    }

    #[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Copy, Clone, From)]
    ReadOnly(bool) {
        update_el: |read_only: Self| Some(read_only.0.as_at_value()),
        read_only,
    }

    #[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Copy, Clone, From)]
    Required(bool) {
        update_el: |required: Self| Some(required.0.as_at_value()),
        required,
    }

    #[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Copy, Clone, From)]
    Reversed(bool) {
        update_el: |reversed: Self| Some(reversed.0.as_at_value()),
        reversed,
    }

    #[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Copy, Clone, From)]
    Sandbox(bool) {
        update_el: |sand_box: Self| Some(sand_box.0.as_at_value()),
        sandbox,
    }

    #[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Copy, Clone, From)]
    Selected(bool) {
        update_el: |selected: Self| Some(selected.0.as_at_value()),
        selected,
    }

    #[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Copy, Clone, From)]
    Size(usize) {
        update_el: |size: Self| Some(size.0),
        size,
    }

    #[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Copy, Clone, From)]
    SpellCheck(bool) {
        update_el: |spell_check: Self| Some(spell_check.0.as_at_value()),
        spell_check,
    }

    // TODO: should we change inner type to something like url::Url ?
    #[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Clone, From)]
    Src(#[from(forward)] Cow<'static, str>) {
        update_el: |src: Self| Some(src.0),
        src,
    }

    #[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Clone, From)]
    SrcDoc(#[from(forward)] Cow<'static, str>) {
        update_el: |src_doc: Self| Some(src_doc.0),
        src_doc,
    }

    // TODO: should we change inner type to enum that contains all ISO lang code ?
    #[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Clone, From)]
    SrcLang(#[from(forward)] Cow<'static, str>) {
        update_el: |src_lang: Self| Some(src_lang.0),
        src_lang,
    }

    // TODO: should we change inner type to something like url::Url ?
    #[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Clone, From)]
    SrcSet(#[from(forward)] Cow<'static, str>) {
        update_el: |src_set: Self| Some(src_set.0),
        src_set,
    }

    #[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Copy, Clone, From)]
    Start(usize) {
        update_el: |start: Self| Some(start.0),
        start,
    }

    #[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Clone, From)]
    Style(#[from(forward)] Cow<'static, str>) {
        update_el: |style: Self| Some(style.0),
        style,
    }

    #[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Copy, Clone, From)]
    TabIndex(usize) {
        update_el: |tab_index: Self| Some(tab_index.0),
        tab_index,
    }

    #[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Clone, From)]
    Title(#[from(forward)] Cow<'static, str>) {
        update_el: |title: Self| Some(title.0),
        title,
    }

    #[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Clone, From)]
    UseMap(#[from(forward)] Cow<'static, str>) {
        update_el: |use_map: Self| Some(use_map.0),
        use_map,
    }

    #[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Clone, From)]
    Value(#[from(forward)] Cow<'static, str>) {
        update_el: |value: Self| Some(value.0),
        value,
    }

    Pattern {
        update_el: |pattern: Self| Some(pattern),
        pattern,
    }

    Type {
        update_el: |ty: Self| Some(ty),
        ty,
    }

    Max {
        update_el: |max: Self| Some(max),
        max,
    }

    Min {
        update_el: |min: Self| Some(min),
        min,
    }

    AutoComplete {
        update_el: |auto: Self| Some(auto),
        auto_complete,
    }

    Coords {
        update_el: |coords: Self| Some(coords),
        coords,
    }

    Dir {
        update_el: |dir: Self| Some(dir),
        dir,
    }

    Draggable {
        update_el: |draggable: Self| Some(draggable),
        draggable,
    }

    EncType {
        update_el: |enc_type: Self| Some(enc_type),
        enc_type,
    }

    Kind {
        update_el: |kind: Self| Some(kind),
        kind,
    }

    Preload {
        update_el: |preload: Self| Some(preload),
        preload,
    }

    Rel {
        update_el: |rel: Self| Some(rel),
        rel,
    }

    Scope {
        update_el: |scope: Self| Some(scope),
        scope,
    }

    Step {
        update_el: |step: Self| Some(step),
        step,
    }

    Target {
        update_el: |target: Self| Some(target),
        target,
    }

    Wrap {
        update_el: |wrap: Self| Some(wrap),
        wrap,
    }

    Cx {
        update_el: |cx: Self| Some(cx),
        cx,
    }

    Cy {
        update_el: |cy: Self| Some(cy),
        cy,
    }

    R {
        update_el: |r: Self| Some(r),
        r,
    }

    Rx {
        update_el: |rx: Self| Some(rx),
        rx,
    }

    Ry {
        update_el: |ry: Self| Some(ry),
        ry,
    }

    X {
        update_el: |x: Self| Some(x),
        x,
    }

    Y {
        update_el: |y: Self| Some(y),
        y,
    }

    ViewBox {
        update_el: |view_box: Self| Some(view_box),
    }
}

impl Extend<Class> for Class {
    fn extend<T: IntoIterator<Item = Class>>(&mut self, iter: T) {
        for class in iter {
            self.0.extend(class.0)
        }
    }
}

impl From<&'static str> for Class {
    fn from(source: &'static str) -> Self {
        if source.is_empty() {
            Class(vec![])
        } else {
            Class(vec![source.into()])
        }
    }
}

impl From<String> for Class {
    fn from(source: String) -> Self {
        if source.is_empty() {
            Class(vec![])
        } else {
            Class(vec![source.into()])
        }
    }
}

impl From<Option<&'static str>> for Class {
    fn from(source: Option<&'static str>) -> Self {
        match source {
            Some(class) if !class.is_empty() => Class(vec![class.into()]),
            _ => Class(vec![]),
        }
    }
}

impl From<Option<String>> for Class {
    fn from(source: Option<String>) -> Self {
        match source {
            Some(class) if !class.is_empty() => Class(vec![class.into()]),
            _ => Class(vec![]),
        }
    }
}

impl From<Vec<&'static str>> for Class {
    fn from(source: Vec<&'static str>) -> Self {
        Class(
            source
                .into_iter()
                .filter_map(|c| {
                    let c = c.to_string();
                    if c.is_empty() {
                        None
                    } else {
                        Some(c.into())
                    }
                })
                .collect(),
        )
    }
}

impl From<Vec<String>> for Class {
    fn from(source: Vec<String>) -> Self {
        Class(
            source
                .into_iter()
                .filter_map(|c| if c.is_empty() { None } else { Some(c.into()) })
                .collect(),
        )
    }
}

impl From<Vec<Option<&'static str>>> for Class {
    fn from(source: Vec<Option<&'static str>>) -> Self {
        Class(
            source
                .into_iter()
                .filter_map(|class| match class {
                    Some(class) if !class.is_empty() => Some(class.into()),
                    _ => None,
                })
                .collect(),
        )
    }
}

impl From<Vec<Option<String>>> for Class {
    fn from(source: Vec<Option<String>>) -> Self {
        Class(
            source
                .into_iter()
                .filter_map(|class| match class {
                    Some(class) if !class.is_empty() => Some(class.into()),
                    _ => None,
                })
                .collect(),
        )
    }
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
    Number(f32),
    #[from]
    Date(chrono::NaiveDate),
}

#[derive(Debug, PartialOrd, PartialEq, Copy, Clone, From, Display)]
pub enum Min {
    #[from]
    Number(f32),
    #[from]
    Date(chrono::NaiveDate),
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
    #[display(fmt = "auto")]
    Auto,
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
pub enum Method {
    #[display(fmt = "get")]
    Get,
    #[display(fmt = "post")]
    Post,
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

#[derive(Debug, Display, Eq, PartialEq, Ord, PartialOrd, Copy, Clone, From)]
pub enum Rel {
    #[display(fmt = "bookmark")]
    BookMark,
    #[display(fmt = "external")]
    External,
    #[display(fmt = "help")]
    Help,
    #[display(fmt = "license")]
    License,
    #[display(fmt = "next")]
    Next,
    #[display(fmt = "nofollow")]
    NoFollow,
    #[display(fmt = "noreferrer")]
    NoReferrer,
    #[display(fmt = "noopener")]
    NoOpener,
    #[display(fmt = "prev")]
    Prev,
    #[display(fmt = "search")]
    Search,
    #[display(fmt = "tag")]
    Tag,
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
    #[display(fmt = "{}", _0)]
    Number(f32),
    #[display(fmt = "any")]
    Any,
}

#[derive(Debug, Display, PartialEq, PartialOrd, Clone, From)]
pub enum Target {
    #[display(fmt = "blank")]
    Blank,
    /// reslove to `self`
    #[display(fmt = "self")]
    CurrentFrame,
    #[display(fmt = "parent")]
    Parent,
    #[display(fmt = "top")]
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
