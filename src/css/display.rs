use crate::css::{self, unit::*, St, Style, ToStyle};

#[derive(Clone, Copy, PartialEq, Eq, Display, From)]
pub enum Display {
    #[from]
    Inline(css::Inline),
    #[from]
    Block(css::Block),
    #[from]
    Contents(css::Contents),
    #[from]
    Flex(css::Flex),
    #[from]
    Grid(css::Grid),
    #[from]
    InlineBlock(css::InlineBlock),
    #[from]
    InlineFlex(css::InlineFlex),
    #[from]
    InlineGrid(css::InlineGrid),
    #[from]
    InlineTable(css::InlineTable),
    #[from]
    ListItem(css::ListItem),
    #[from]
    RunIn(css::RunIn),
    #[from]
    Table(css::Table),
    #[from]
    TableCaption(css::TableCaption),
    #[from]
    TableColumnGroup(css::TableColumnGroup),
    #[from]
    TableHeaderGroup(css::TableHeaderGroup),
    #[from]
    TableFooterGroup(css::TableFooterGroup),
    #[from]
    TableRowGroup(css::TableRowGroup),
    #[from]
    TableCell(css::TableCell),
    #[from]
    TableColumn(css::TableColumn),
    #[from]
    TableRow(css::TableRow),
    #[from]
    None(css::None),
    #[from]
    Initial(css::Initial),
    #[from]
    Inherit(css::Inherit),
}

impl ToStyle for Display {
    fn to_style(&self) -> Style {
        Style::new().add(St::Display, self)
    }
}
