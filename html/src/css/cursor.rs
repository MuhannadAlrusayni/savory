use crate::css::{values as val, St, StyleValues, UpdateStyleValues};

#[derive(Clone, Debug, Copy, PartialEq, Eq, Display, From)]
pub enum Cursor {
    #[from]
    Alias(val::Alias),
    #[from]
    AllScroll(val::AllScroll),
    #[from]
    Auto(val::Auto),
    #[from]
    Cell(val::Cell),
    #[from]
    ContextMenu(val::ContextMenu),
    #[from]
    ColResize(val::ColResize),
    #[from]
    Copy(val::Copy),
    #[from]
    Crosshair(val::Crosshair),
    #[from]
    Default(val::Default),
    #[from]
    EResize(val::EResize),
    #[from]
    EwResize(val::EwResize),
    #[from]
    Grab(val::Grab),
    #[from]
    Grabbing(val::Grabbing),
    #[from]
    Help(val::Help),
    #[from]
    Move(val::Move),
    #[from]
    NResize(val::NResize),
    #[from]
    NeResize(val::NeResize),
    #[from]
    NeswResize(val::NeswResize),
    #[from]
    NsResize(val::NsResize),
    #[from]
    NwResize(val::NwResize),
    #[from]
    NwseResize(val::NwseResize),
    #[from]
    NoDrop(val::NoDrop),
    #[from]
    None(val::None),
    #[from]
    NotAllowed(val::NotAllowed),
    #[from]
    Pointer(val::Pointer),
    #[from]
    Progress(val::Progress),
    #[from]
    RowResize(val::RowResize),
    #[from]
    SResize(val::SResize),
    #[from]
    SeResize(val::SeResize),
    #[from]
    SwResize(val::SwResize),
    #[from]
    Text(val::Text),
    // TODO: Handle Url value
    // Url(Vec<String>),
    #[from]
    VerticalText(val::VerticalText),
    #[from]
    WResize(val::WResize),
    #[from]
    Wait(val::Wait),
    #[from]
    ZoomIn(val::ZoomIn),
    #[from]
    ZoomOut(val::ZoomOut),
    #[from]
    Initial(val::Initial),
    #[from]
    Inherit(val::Inherit),
}

impl UpdateStyleValues for Cursor {
    fn update_style_values(self, values: StyleValues) -> StyleValues {
        values.add(St::Cursor, self)
    }
}
