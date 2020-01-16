use crate::css::{self, St, Style, ToStyle};

#[derive(Clone, Copy, PartialEq, Eq, Display, From)]
pub enum Cursor {
    #[from]
    Alias(css::Alias),
    #[from]
    AllScroll(css::AllScroll),
    #[from]
    Auto(css::Auto),
    #[from]
    Cell(css::Cell),
    #[from]
    ContextMenu(css::ContextMenu),
    #[from]
    ColResize(css::ColResize),
    #[from]
    Copy(css::Copy),
    #[from]
    Crosshair(css::Crosshair),
    #[from]
    Default(css::Default),
    #[from]
    EResize(css::EResize),
    #[from]
    EwResize(css::EwResize),
    #[from]
    Grab(css::Grab),
    #[from]
    Grabbing(css::Grabbing),
    #[from]
    Help(css::Help),
    #[from]
    Move(css::Move),
    #[from]
    NResize(css::NResize),
    #[from]
    NeResize(css::NeResize),
    #[from]
    NeswResize(css::NeswResize),
    #[from]
    NsResize(css::NsResize),
    #[from]
    NwResize(css::NwResize),
    #[from]
    NwseResize(css::NwseResize),
    #[from]
    NoDrop(css::NoDrop),
    #[from]
    None(css::None),
    #[from]
    NotAllowed(css::NotAllowed),
    #[from]
    Pointer(css::Pointer),
    #[from]
    Progress(css::Progress),
    #[from]
    RowResize(css::RowResize),
    #[from]
    SResize(css::SResize),
    #[from]
    SeResize(css::SeResize),
    #[from]
    SwResize(css::SwResize),
    #[from]
    Text(css::Text),
    // TODO: Handle Url value
    // Url(Vec<String>),
    #[from]
    VerticalText(css::VerticalText),
    #[from]
    WResize(css::WResize),
    #[from]
    Wait(css::Wait),
    #[from]
    ZoomIn(css::ZoomIn),
    #[from]
    ZoomOut(css::ZoomOut),
    #[from]
    Initial(css::Initial),
    #[from]
    Inherit(css::Inherit),
}

impl ToStyle for Cursor {
    fn to_style(&self) -> Style {
        Style::new().add(St::Cursor, self)
    }
}
