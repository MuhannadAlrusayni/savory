use crate::css::{values as val, St, Style, ToStyle};

#[derive(Clone, Copy, PartialEq, Eq, Display, From)]
pub enum Display {
    #[from]
    Inline(val::Inline),
    #[from]
    Block(val::Block),
    #[from]
    Contents(val::Contents),
    #[from]
    Flex(val::Flex),
    #[from]
    Grid(val::Grid),
    #[from]
    InlineBlock(val::InlineBlock),
    #[from]
    InlineFlex(val::InlineFlex),
    #[from]
    InlineGrid(val::InlineGrid),
    #[from]
    InlineTable(val::InlineTable),
    #[from]
    ListItem(val::ListItem),
    #[from]
    RunIn(val::RunIn),
    #[from]
    Table(val::Table),
    #[from]
    TableCaption(val::TableCaption),
    #[from]
    TableColumnGroup(val::TableColumnGroup),
    #[from]
    TableHeaderGroup(val::TableHeaderGroup),
    #[from]
    TableFooterGroup(val::TableFooterGroup),
    #[from]
    TableRowGroup(val::TableRowGroup),
    #[from]
    TableCell(val::TableCell),
    #[from]
    TableColumn(val::TableColumn),
    #[from]
    TableRow(val::TableRow),
    #[from]
    None(val::None),
    #[from]
    Initial(val::Initial),
    #[from]
    Inherit(val::Inherit),
}

impl ToStyle for Display {
    fn to_style(&self) -> Style {
        Style::new().add(St::Display, self)
    }
}
