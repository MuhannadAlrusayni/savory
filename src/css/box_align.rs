use crate::css::{St, values as val, self, ToStyle, Style};

#[derive(Clone, Debug, Copy, PartialEq, Eq, Display, From)]
pub enum JustifyContent {
    #[from]
    Normal(val::Normal),
    #[from]
    SpaceBetween(val::SpaceBetween),
    #[from]
    SpaceAround(val::SpaceAround),
    #[from]
    SpaceEvenly(val::SpaceEvenly),
    #[from]
    Stretch(val::Stretch),
    #[from]
    Center(val::Center),
    #[from]
    SafeCenter(val::SafeCenter),
    #[from]
    UnsafeCenter(val::UnsafeCenter),
    #[from]
    Start(val::Start),
    #[from]
    SafeStart(val::SafeStart),
    #[from]
    UnsafeStart(val::UnsafeStart),
    #[from]
    End(val::End),
    #[from]
    SafeEnd(val::SafeEnd),
    #[from]
    UnsafeEnd(val::UnsafeEnd),
    #[from]
    FlexStart(val::FlexStart),
    #[from]
    SafeFlexStart(val::SafeFlexStart),
    #[from]
    UnsafeFlexStart(val::UnsafeFlexStart),
    #[from]
    FlexEnd(val::FlexEnd),
    #[from]
    SafeFlexEnd(val::SafeFlexEnd),
    #[from]
    UnsafeFlexEnd(val::UnsafeFlexEnd),
    #[from]
    Left(val::Left),
    #[from]
    SafeLeft(val::SafeLeft),
    #[from]
    UnsafeLeft(val::UnsafeLeft),
    #[from]
    Right(val::Right),
    #[from]
    SafeRight(val::SafeRight),
    #[from]
    UnsafeRight(val::UnsafeRight),
}

impl ToStyle for JustifyContent {
    fn to_style(&self) -> Style {
        Style::new().add(St::JustifyContent, self)
    }
}

#[derive(Clone, Debug, Copy, PartialEq, Eq, Display, From)]
pub enum AlignContent {
    #[from]
    Normal(val::Normal),
    #[from]
    Baseline(val::Baseline),
    #[from]
    FirstBaseline(val::FirstBaseline),
    #[from]
    LastBaseline(val::LastBaseline),
    #[from]
    SpaceBetween(val::SpaceBetween),
    #[from]
    SpaceAround(val::SpaceAround),
    #[from]
    SpaceEvenly(val::SpaceEvenly),
    #[from]
    Stretch(val::Stretch),
    #[from]
    Center(val::Center),
    #[from]
    SafeCenter(val::SafeCenter),
    #[from]
    UnsafeCenter(val::UnsafeCenter),
    #[from]
    Start(val::Start),
    #[from]
    SafeStart(val::SafeStart),
    #[from]
    UnsafeStart(val::UnsafeStart),
    #[from]
    End(val::End),
    #[from]
    SafeEnd(val::SafeEnd),
    #[from]
    UnsafeEnd(val::UnsafeEnd),
    #[from]
    FlexStart(val::FlexStart),
    #[from]
    SafeFlexStart(val::SafeFlexStart),
    #[from]
    UnsafeFlexStart(val::UnsafeFlexStart),
    #[from]
    FlexEnd(val::FlexEnd),
    #[from]
    SafeFlexEnd(val::SafeFlexEnd),
    #[from]
    UnsafeFlexEnd(val::UnsafeFlexEnd),
}

impl ToStyle for AlignContent {
    fn to_style(&self) -> Style {
        Style::new().add(St::AlignContent, self)
    }
}

#[derive(Clone, Debug, Copy, PartialEq, Eq, Display, From)]
pub enum AlignItems {
    #[from]
    Normal(val::Normal),
    #[from]
    Stretch(val::Stretch),
    #[from]
    Baseline(val::Baseline),
    #[from]
    FirstBaseline(val::FirstBaseline),
    #[from]
    LastBaseline(val::LastBaseline),
    #[from]
    Center(val::Center),
    #[from]
    SafeCenter(val::SafeCenter),
    #[from]
    UnsafeCenter(val::UnsafeCenter),
    #[from]
    Start(val::Start),
    #[from]
    SafeStart(val::SafeStart),
    #[from]
    UnsafeStart(val::UnsafeStart),
    #[from]
    End(val::End),
    #[from]
    SafeEnd(val::SafeEnd),
    #[from]
    UnsafeEnd(val::UnsafeEnd),
    #[from]
    SelfStart(val::SelfStart),
    #[from]
    SafeSelfStart(val::SafeSelfStart),
    #[from]
    UnsafeSelfStart(val::UnsafeSelfStart),
    #[from]
    SelfEnd(val::SelfEnd),
    #[from]
    SafeSelfEnd(val::SafeSelfEnd),
    #[from]
    UnsafeSelfEnd(val::UnsafeSelfEnd),
    #[from]
    FlexStart(val::FlexStart),
    #[from]
    SafeFlexStart(val::SafeFlexStart),
    #[from]
    UnsafeFlexStart(val::UnsafeFlexStart),
    #[from]
    FlexEnd(val::FlexEnd),
    #[from]
    SafeFlexEnd(val::SafeFlexEnd),
    #[from]
    UnsafeFlexEnd(val::UnsafeFlexEnd),
}

impl ToStyle for AlignItems {
    fn to_style(&self) -> Style {
        Style::new().add(St::AlignItems, self)
    }
}

#[derive(Clone, Debug, Copy, PartialEq, Eq, Display, From)]
pub enum JustifySelf {
    #[from]
    Auto(val::Auto),
    #[from]
    Normal(val::Normal),
    #[from]
    Stretch(val::Stretch),
    #[from]
    Baseline(val::Baseline),
    #[from]
    FirstBaseline(val::FirstBaseline),
    #[from]
    LastBaseline(val::LastBaseline),
    #[from]
    Center(val::Center),
    #[from]
    SafeCenter(val::SafeCenter),
    #[from]
    UnsafeCenter(val::UnsafeCenter),
    #[from]
    Start(val::Start),
    #[from]
    SafeStart(val::SafeStart),
    #[from]
    UnsafeStart(val::UnsafeStart),
    #[from]
    End(val::End),
    #[from]
    SafeEnd(val::SafeEnd),
    #[from]
    UnsafeEnd(val::UnsafeEnd),
    #[from]
    SelfStart(val::SelfStart),
    #[from]
    SafeSelfStart(val::SafeSelfStart),
    #[from]
    UnsafeSelfStart(val::UnsafeSelfStart),
    #[from]
    SelfEnd(val::SelfEnd),
    #[from]
    SafeSelfEnd(val::SafeSelfEnd),
    #[from]
    UnsafeSelfEnd(val::UnsafeSelfEnd),
    #[from]
    FlexStart(val::FlexStart),
    #[from]
    SafeFlexStart(val::SafeFlexStart),
    #[from]
    UnsafeFlexStart(val::UnsafeFlexStart),
    #[from]
    FlexEnd(val::FlexEnd),
    #[from]
    SafeFlexEnd(val::SafeFlexEnd),
    #[from]
    UnsafeFlexEnd(val::UnsafeFlexEnd),
    #[from]
    Left(val::Left),
    #[from]
    SafeLeft(val::SafeLeft),
    #[from]
    UnsafeLeft(val::UnsafeLeft),
    #[from]
    Right(val::Right),
    #[from]
    SafeRight(val::SafeRight),
    #[from]
    UnsafeRight(val::UnsafeRight),
}

impl ToStyle for JustifySelf {
    fn to_style(&self) -> Style {
        Style::new().add(St::JustifySelf, self)
    }
}

#[derive(Clone, Debug, Copy, PartialEq, Eq, Display, From)]
pub enum AlignSelf {
    #[from]
    Auto(val::Auto),
    #[from]
    Normal(val::Normal),
    #[from]
    Stretch(val::Stretch),
    #[from]
    Baseline(val::Baseline),
    #[from]
    FirstBaseline(val::FirstBaseline),
    #[from]
    LastBaseline(val::LastBaseline),
    #[from]
    Center(val::Center),
    #[from]
    SafeCenter(val::SafeCenter),
    #[from]
    UnsafeCenter(val::UnsafeCenter),
    #[from]
    Start(val::Start),
    #[from]
    SafeStart(val::SafeStart),
    #[from]
    UnsafeStart(val::UnsafeStart),
    #[from]
    End(val::End),
    #[from]
    SafeEnd(val::SafeEnd),
    #[from]
    UnsafeEnd(val::UnsafeEnd),
    #[from]
    SelfStart(val::SelfStart),
    #[from]
    SafeSelfStart(val::SafeSelfStart),
    #[from]
    UnsafeSelfStart(val::UnsafeSelfStart),
    #[from]
    SelfEnd(val::SelfEnd),
    #[from]
    SafeSelfEnd(val::SafeSelfEnd),
    #[from]
    UnsafeSelfEnd(val::UnsafeSelfEnd),
    #[from]
    FlexStart(val::FlexStart),
    #[from]
    SafeFlexStart(val::SafeFlexStart),
    #[from]
    UnsafeFlexStart(val::UnsafeFlexStart),
    #[from]
    FlexEnd(val::FlexEnd),
    #[from]
    SafeFlexEnd(val::SafeFlexEnd),
    #[from]
    UnsafeFlexEnd(val::UnsafeFlexEnd),
}

impl ToStyle for AlignSelf {
    fn to_style(&self) -> Style {
        Style::new().add(St::AlignSelf, self)
    }
}
