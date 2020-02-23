pub use seed::prelude::At;
use seed::prelude::UpdateEl;
use seed::prelude::*;
use std::borrow::Cow;

macro_rules! update_el_for_attrs {
    ( $( $name:ident => $expr:expr $(,)? )* ) => {
        $(
            impl<Msg> UpdateEl<El<Msg>> for $name {
                fn update(self, el: &mut El<Msg>) {
                    let closure = $expr;
                    el.attrs.add(At::$name, closure(self));
                }
            }
        )*
    }
}

// TODO: should we change inner type to something like mime::Mime ?
#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Clone, From)]
pub struct Accept(Cow<'static, str>);

// TODO: should we change inner type to something like murdoch::CharacterSetEnum ?
#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Clone, From)]
pub struct AcceptCharset(Cow<'static, str>);

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Clone, From)]
pub struct AccessKey(Cow<'static, str>);

// TODO: should we change inner type to something like url::Url ?
#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Clone, From)]
pub struct Action(Cow<'static, str>);

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Clone, From)]
pub struct Alt(Cow<'static, str>);

// TODO: should this be an enum with all posible values ?
#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Clone, From)]
pub struct AutoComplete(Cow<'static, str>);

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Copy, Clone, From)]
pub struct AutoFocus(bool);

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Copy, Clone, From)]
pub struct AutoPlay(bool);

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Copy, Clone, From)]
pub struct Checked(bool);

// TODO: should we change inner type to something like url::Url ?
#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Clone, From)]
pub struct Cite(Cow<'static, str>);

#[derive(Debug, Clone, From)]
pub struct Class(Vec<Cow<'static, str>>);

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Copy, Clone, From)]
pub struct Cols(usize);

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Copy, Clone, From)]
pub struct Rows(usize);

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Copy, Clone, From)]
pub struct Span(usize);

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Copy, Clone, From)]
pub struct ColSpan(usize);

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Copy, Clone, From)]
pub struct RowSpan(usize);

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Copy, Clone, From)]
pub struct ContentEditable(bool);

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Copy, Clone, From)]
pub struct Controls(bool);

#[derive(Debug, PartialOrd, PartialEq, Clone, From, Display)]
pub enum Coords {
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
    #[display(fmt = "{},{},{}", x, y, radius)]
    Circle { x: f32, y: f32, radius: f32 },
    #[display(
        fmt = "{}",
        "edges.iter().map(|(x, y)| format!(\"{},{}\", x, y)).collect::<Vec<String>>().join(\",\")"
    )]
    Polygon { edges: Vec<(f32, f32)> },
}

// TODO: check if value and name are valid
#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Clone, From)]
pub struct CustomData {
    name: Cow<'static, str>,
    value: Cow<'static, str>,
}

impl<Msg> UpdateEl<El<Msg>> for CustomData {
    fn update(self, el: &mut El<Msg>) {
        el.attrs.add(At::Custom(self.name), self.value);
    }
}

// TODO: use chrono::DateTime
#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Clone, From)]
pub struct DateTime(Cow<'static, str>);

#[derive(Debug, PartialEq, PartialOrd, Copy, Clone, From, Display)]
pub enum Dir {
    #[display(fmt = "ltr")]
    LTR,
    #[display(fmt = "rtl")]
    RTL,
    #[display(fmt = "auto")]
    Auto,
}

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Copy, Clone, From)]
pub struct Disabled(bool);

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
pub enum DropZone {
    #[display(fmt = "copy")]
    Copy,
    #[display(fmt = "move")]
    Move,
    #[display(fmt = "link")]
    Link,
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

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Clone, From)]
pub struct For(Cow<'static, str>);

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Clone, From)]
pub struct Form(Cow<'static, str>);

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Clone, From)]
pub struct Headers(Cow<'static, str>);

// TODO: should we change inner type to something like url::Url ?
#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Clone, From)]
pub struct FormAction(Cow<'static, str>);

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Copy, Clone, From)]
pub struct Hidden(bool);

#[derive(Debug, PartialEq, PartialOrd, Copy, Clone, From)]
pub struct High(f32);

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Copy, Clone, From)]
pub struct Height(usize);

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Copy, Clone, From)]
pub struct Width(usize);

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Clone, From)]
pub struct Href(Cow<'static, str>);

// TODO: should we change inner type to enum that contains all ISO lang code ?
#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Clone, From)]
pub struct HrefLang(Cow<'static, str>);

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Clone, From)]
pub struct Id(Cow<'static, str>);

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Copy, Clone, From)]
pub struct IsMap(bool);

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

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Clone, From)]
pub struct Label(Cow<'static, str>);

// TODO: should we change inner type to enum that contains all ISO lang code ?
#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Clone, From)]
pub struct Lang(Cow<'static, str>);

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Clone, From)]
pub struct List(Cow<'static, str>);

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Copy, Clone, From)]
pub struct Loop(bool);

#[derive(Debug, PartialEq, PartialOrd, Copy, Clone, From)]
pub struct Low(f32);

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Copy, Clone, From)]
pub struct MaxLength(usize);

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Clone, From)]
pub struct Max(Cow<'static, str>);

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Clone, From)]
pub struct Min(Cow<'static, str>);

// TODO: should we use enum that work the html expect ?
#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Clone, From)]
pub struct Media(Cow<'static, str>);

#[derive(Debug, Display, Eq, PartialEq, Ord, PartialOrd, Copy, Clone, From)]
pub enum Method {
    #[display(fmt = "get")]
    Get,
    #[display(fmt = "post")]
    Post,
}

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Copy, Clone, From)]
pub struct Multiple(bool);

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Copy, Clone, From)]
pub struct Muted(bool);

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Clone, From)]
pub struct Name(Cow<'static, str>);

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Copy, Clone, From)]
pub struct NoValidate(bool);

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Copy, Clone, From)]
pub struct Open(bool);

#[derive(Debug, PartialEq, PartialOrd, Copy, Clone, From)]
pub struct Optimum(f32);

// TODO: should we use regex::Regex ?
#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Clone, From)]
pub struct Pattern(Cow<'static, str>);

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Clone, From)]
pub struct Placeholder(Cow<'static, str>);

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Clone, From)]
pub struct Poster(Cow<'static, str>);

#[derive(Debug, Display, Eq, PartialEq, Ord, PartialOrd, Copy, Clone, From)]
pub enum Preload {
    #[display(fmt = "auto")]
    Auto,
    #[display(fmt = "metadata")]
    Metadata,
    #[display(fmt = "none")]
    None,
}

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Copy, Clone, From)]
pub struct ReadOnly(bool);

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

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Copy, Clone, From)]
pub struct Required(bool);

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Copy, Clone, From)]
pub struct Reversed(bool);

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Copy, Clone, From)]
pub struct Sandbox(bool);

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

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Copy, Clone, From)]
pub struct Selected(bool);

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

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Copy, Clone, From)]
pub struct Size(usize);

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Copy, Clone, From)]
pub struct SpellCheck(bool);

// TODO: should we change inner type to something like url::Url ?
#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Clone, From)]
pub struct Src(Cow<'static, str>);

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Clone, From)]
pub struct SrcDoc(Cow<'static, str>);

// TODO: should we change inner type to enum that contains all ISO lang code ?
#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Clone, From)]
pub struct SrcLang(Cow<'static, str>);

// TODO: should we change inner type to something like url::Url ?
#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Clone, From)]
pub struct SrcSet(Cow<'static, str>);

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Copy, Clone, From)]
pub struct Start(usize);

#[derive(Debug, Display, PartialEq, PartialOrd, Copy, Clone, From)]
pub enum Step {
    #[from]
    #[display(fmt = "{}", _0)]
    Number(f32),
    #[display(fmt = "any")]
    Any,
}

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Clone, From)]
pub struct Style(Cow<'static, str>);

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Copy, Clone, From)]
pub struct TabIndex(usize);

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
    #[display(fmt = "{}", _0)]
    FrameName(Cow<'static, str>),
}

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Clone, From)]
pub struct Title(Cow<'static, str>);

// TODO: should we use enum that containes all posible values that can be in
// type ?
#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Clone, From)]
pub struct Type(Cow<'static, str>);

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Clone, From)]
pub struct UseMap(Cow<'static, str>);

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Clone, From)]
pub struct Value(Cow<'static, str>);

#[derive(Debug, Display, PartialEq, PartialOrd, Clone, From)]
pub enum Wrap {
    #[display(fmt = "soft")]
    Soft,
    #[display(fmt = "hard")]
    Hard,
}

update_el_for_attrs! {
    Accept => |accept: Self| accept.0,
    AcceptCharset => |charset: Self| charset.0,
    AccessKey => |key: Self| key.0,
    Action => |action: Self| action.0,
    Alt => |alt: Self| alt.0,
    AutoComplete => |auto: Self| auto.0,
    AutoFocus => |auto: Self| auto.0.as_at_value(),
    AutoPlay => |auto: Self| auto.0.as_at_value(),
    Checked => |checked: Self| checked.0.as_at_value(),
    Cite => |cite: Self| cite.0,
    Class => |class: Self| class.0.join(" "),
    Cols => |cols: Self| cols.0,
    Rows => |rows: Self| rows.0,
    Span => |span: Self| span.0,
    ColSpan => |col: Self| col.0,
    RowSpan => |row: Self| row.0,
    ContentEditable => |value: Self| value.0,
    Controls => |controls: Self| controls.0,
    Coords => |coords: Self| coords,
    DateTime => |datetime: Self| datetime.0,
    Dir => |dir: Self| dir,
    Disabled => |dis: Self| dis.0.as_at_value(),
    Draggable => |draggable: Self| draggable,
    DropZone => |drop_zone: Self| drop_zone,
    EncType => |enc_type: Self| enc_type,
    For => |f: Self| f.0,
    Form => |form: Self| form.0,
    Headers => |headers: Self| headers.0,
    FormAction => |action: Self| action.0,
    Height => |height: Self| height.0,
    Width => |width: Self| width.0,
    Hidden => |hidden: Self| hidden.0.as_at_value(),
    High => |high: Self| high.0,
    Href => |href: Self| href.0,
    HrefLang => |lang: Self| lang.0,
    Id => |id: Self| id.0,
    IsMap => |is_map: Self| is_map.0.as_at_value(),
    Kind => |kind: Self| kind,
    Label => |label: Self| label.0,
    Lang => |lang: Self| lang.0,
    List => |list: Self| list.0,
    Loop => |l: Self| l.0.as_at_value(),
    Low => |low: Self| low.0,
    MaxLength => |max_len: Self| max_len.0,
    Max => |max: Self| max.0,
    Min => |min: Self| min.0,
    Media => |media: Self| media.0,
    Method => |method: Self| method,
    Multiple => |multiple: Self| multiple.0.as_at_value(),
    Muted => |muted: Self| muted.0.as_at_value(),
    Name => |name: Self| name.0,
    NoValidate => |no_validate: Self| no_validate.0.as_at_value(),
    Open => |open: Self| open.0.as_at_value(),
    Optimum => |optimum: Self| optimum.0,
    Pattern => |pattern: Self| pattern.0,
    Placeholder => |placeholder: Self| placeholder.0,
    Poster => |poster: Self| poster.0,
    Preload => |preload: Self| preload,
    ReadOnly => |read_only: Self| read_only.0.as_at_value(),
    Rel => |rel: Self| rel,
    Required => |required: Self| required.0.as_at_value(),
    Reversed => |reversed: Self| reversed.0.as_at_value(),
    Sandbox => |sand_box: Self| sand_box.0.as_at_value(),
    Scope => |scope: Self| scope,
    Selected => |selected: Self| selected.0.as_at_value(),
    Size => |size: Self| size.0,
    SpellCheck => |spell_check: Self| spell_check.0.as_at_value(),
    Src => |src: Self| src.0,
    SrcDoc => |src_doc: Self| src_doc.0,
    SrcLang => |src_lang: Self| src_lang.0,
    SrcSet => |src_set: Self| src_set.0,
    Start => |start: Self| start.0,
    Step => |step: Self| step,
    Style => |style: Self| style.0,
    TabIndex => |tab_index: Self| tab_index.0,
    Target => |target: Self| target,
    Title => |title: Self| title.0,
    Type => |ty: Self| ty.0,
    UseMap => |use_map: Self| use_map.0,
    Value => |value: Self| value.0,
    Wrap => |wrap: Self| wrap,
}
