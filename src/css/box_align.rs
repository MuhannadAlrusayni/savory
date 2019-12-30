use crate::css::{St, self, ToStyle, Style};

#[derive(Clone, Debug, Copy, PartialEq, Eq, Display, From)]
pub enum JustifyContent {
    #[from]
    Normal(css::Normal),
    #[from]
    SpaceBetween(css::SpaceBetween),
    #[from]
    SpaceAround(css::SpaceAround),
    #[from]
    SpaceEvenly(css::SpaceEvenly),
    #[from]
    Stretch(css::Stretch),
    #[from]
    Center(css::Center),
    #[from]
    SafeCenter(css::SafeCenter),
    #[from]
    UnsafeCenter(css::UnsafeCenter),
    #[from]
    Start(css::Start),
    #[from]
    SafeStart(css::SafeStart),
    #[from]
    UnsafeStart(css::UnsafeStart),
    #[from]
    End(css::End),
    #[from]
    SafeEnd(css::SafeEnd),
    #[from]
    UnsafeEnd(css::UnsafeEnd),
    #[from]
    FlexStart(css::FlexStart),
    #[from]
    SafeFlexStart(css::SafeFlexStart),
    #[from]
    UnsafeFlexStart(css::UnsafeFlexStart),
    #[from]
    FlexEnd(css::FlexEnd),
    #[from]
    SafeFlexEnd(css::SafeFlexEnd),
    #[from]
    UnsafeFlexEnd(css::UnsafeFlexEnd),
    #[from]
    Left(css::Left),
    #[from]
    SafeLeft(css::SafeLeft),
    #[from]
    UnsafeLeft(css::UnsafeLeft),
    #[from]
    Right(css::Right),
    #[from]
    SafeRight(css::SafeRight),
    #[from]
    UnsafeRight(css::UnsafeRight),
}

impl ToStyle for JustifyContent {
    fn to_style(&self) -> Style {
        Style::new().add(St::JustifyContent, self)
    }
}

#[derive(Clone, Debug, Copy, PartialEq, Eq, Display, From)]
pub enum AlignContent {
    #[from]
    Normal(css::Normal),
    #[from]
    Baseline(css::Baseline),
    #[from]
    FirstBaseline(css::FirstBaseline),
    #[from]
    LastBaseline(css::LastBaseline),
    #[from]
    SpaceBetween(css::SpaceBetween),
    #[from]
    SpaceAround(css::SpaceAround),
    #[from]
    SpaceEvenly(css::SpaceEvenly),
    #[from]
    Stretch(css::Stretch),
    #[from]
    Center(css::Center),
    #[from]
    SafeCenter(css::SafeCenter),
    #[from]
    UnsafeCenter(css::UnsafeCenter),
    #[from]
    Start(css::Start),
    #[from]
    SafeStart(css::SafeStart),
    #[from]
    UnsafeStart(css::UnsafeStart),
    #[from]
    End(css::End),
    #[from]
    SafeEnd(css::SafeEnd),
    #[from]
    UnsafeEnd(css::UnsafeEnd),
    #[from]
    FlexStart(css::FlexStart),
    #[from]
    SafeFlexStart(css::SafeFlexStart),
    #[from]
    UnsafeFlexStart(css::UnsafeFlexStart),
    #[from]
    FlexEnd(css::FlexEnd),
    #[from]
    SafeFlexEnd(css::SafeFlexEnd),
    #[from]
    UnsafeFlexEnd(css::UnsafeFlexEnd),
}

impl ToStyle for AlignContent {
    fn to_style(&self) -> Style {
        Style::new().add(St::AlignContent, self)
    }
}

#[derive(Clone, Debug, Copy, PartialEq, Eq, Display, From)]
pub enum AlignItems {
    #[from]
    Normal(css::Normal),
    #[from]
    Stretch(css::Stretch),
    #[from]
    Baseline(css::Baseline),
    #[from]
    FirstBaseline(css::FirstBaseline),
    #[from]
    LastBaseline(css::LastBaseline),
    #[from]
    Center(css::Center),
    #[from]
    SafeCenter(css::SafeCenter),
    #[from]
    UnsafeCenter(css::UnsafeCenter),
    #[from]
    Start(css::Start),
    #[from]
    SafeStart(css::SafeStart),
    #[from]
    UnsafeStart(css::UnsafeStart),
    #[from]
    End(css::End),
    #[from]
    SafeEnd(css::SafeEnd),
    #[from]
    UnsafeEnd(css::UnsafeEnd),
    #[from]
    SelfStart(css::SelfStart),
    #[from]
    SafeSelfStart(css::SafeSelfStart),
    #[from]
    UnsafeSelfStart(css::UnsafeSelfStart),
    #[from]
    SelfEnd(css::SelfEnd),
    #[from]
    SafeSelfEnd(css::SafeSelfEnd),
    #[from]
    UnsafeSelfEnd(css::UnsafeSelfEnd),
    #[from]
    FlexStart(css::FlexStart),
    #[from]
    SafeFlexStart(css::SafeFlexStart),
    #[from]
    UnsafeFlexStart(css::UnsafeFlexStart),
    #[from]
    FlexEnd(css::FlexEnd),
    #[from]
    SafeFlexEnd(css::SafeFlexEnd),
    #[from]
    UnsafeFlexEnd(css::UnsafeFlexEnd),
}

impl ToStyle for AlignItems {
    fn to_style(&self) -> Style {
        Style::new().add(St::AlignItems, self)
    }
}

#[derive(Clone, Debug, Copy, PartialEq, Eq, Display, From)]
pub enum JustifySelf {
    #[from]
    Auto(css::Auto),
    #[from]
    Normal(css::Normal),
    #[from]
    Stretch(css::Stretch),
    #[from]
    Baseline(css::Baseline),
    #[from]
    FirstBaseline(css::FirstBaseline),
    #[from]
    LastBaseline(css::LastBaseline),
    #[from]
    Center(css::Center),
    #[from]
    SafeCenter(css::SafeCenter),
    #[from]
    UnsafeCenter(css::UnsafeCenter),
    #[from]
    Start(css::Start),
    #[from]
    SafeStart(css::SafeStart),
    #[from]
    UnsafeStart(css::UnsafeStart),
    #[from]
    End(css::End),
    #[from]
    SafeEnd(css::SafeEnd),
    #[from]
    UnsafeEnd(css::UnsafeEnd),
    #[from]
    SelfStart(css::SelfStart),
    #[from]
    SafeSelfStart(css::SafeSelfStart),
    #[from]
    UnsafeSelfStart(css::UnsafeSelfStart),
    #[from]
    SelfEnd(css::SelfEnd),
    #[from]
    SafeSelfEnd(css::SafeSelfEnd),
    #[from]
    UnsafeSelfEnd(css::UnsafeSelfEnd),
    #[from]
    FlexStart(css::FlexStart),
    #[from]
    SafeFlexStart(css::SafeFlexStart),
    #[from]
    UnsafeFlexStart(css::UnsafeFlexStart),
    #[from]
    FlexEnd(css::FlexEnd),
    #[from]
    SafeFlexEnd(css::SafeFlexEnd),
    #[from]
    UnsafeFlexEnd(css::UnsafeFlexEnd),
    #[from]
    Left(css::Left),
    #[from]
    SafeLeft(css::SafeLeft),
    #[from]
    UnsafeLeft(css::UnsafeLeft),
    #[from]
    Right(css::Right),
    #[from]
    SafeRight(css::SafeRight),
    #[from]
    UnsafeRight(css::UnsafeRight),
}

impl ToStyle for JustifySelf {
    fn to_style(&self) -> Style {
        Style::new().add(St::JustifySelf, self)
    }
}

#[derive(Clone, Debug, Copy, PartialEq, Eq, Display, From)]
pub enum AlignSelf {
    #[from]
    Auto(css::Auto),
    #[from]
    Normal(css::Normal),
    #[from]
    Stretch(css::Stretch),
    #[from]
    Baseline(css::Baseline),
    #[from]
    FirstBaseline(css::FirstBaseline),
    #[from]
    LastBaseline(css::LastBaseline),
    #[from]
    Center(css::Center),
    #[from]
    SafeCenter(css::SafeCenter),
    #[from]
    UnsafeCenter(css::UnsafeCenter),
    #[from]
    Start(css::Start),
    #[from]
    SafeStart(css::SafeStart),
    #[from]
    UnsafeStart(css::UnsafeStart),
    #[from]
    End(css::End),
    #[from]
    SafeEnd(css::SafeEnd),
    #[from]
    UnsafeEnd(css::UnsafeEnd),
    #[from]
    SelfStart(css::SelfStart),
    #[from]
    SafeSelfStart(css::SafeSelfStart),
    #[from]
    UnsafeSelfStart(css::UnsafeSelfStart),
    #[from]
    SelfEnd(css::SelfEnd),
    #[from]
    SafeSelfEnd(css::SafeSelfEnd),
    #[from]
    UnsafeSelfEnd(css::UnsafeSelfEnd),
    #[from]
    FlexStart(css::FlexStart),
    #[from]
    SafeFlexStart(css::SafeFlexStart),
    #[from]
    UnsafeFlexStart(css::UnsafeFlexStart),
    #[from]
    FlexEnd(css::FlexEnd),
    #[from]
    SafeFlexEnd(css::SafeFlexEnd),
    #[from]
    UnsafeFlexEnd(css::UnsafeFlexEnd),
}

impl ToStyle for AlignSelf {
    fn to_style(&self) -> Style {
        Style::new().add(St::AlignSelf, self)
    }
}
