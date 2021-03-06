use crate::{values as val, St, StyleValues, UpdateStyleValues};

#[derive(Clone, Debug, Copy, PartialEq, Eq, Display, From)]
pub enum JustifyContent {
    Normal(val::Normal),
    SpaceBetween(val::SpaceBetween),
    SpaceAround(val::SpaceAround),
    SpaceEvenly(val::SpaceEvenly),
    Stretch(val::Stretch),
    Center(val::Center),
    SafeCenter(val::SafeCenter),
    UnsafeCenter(val::UnsafeCenter),
    Start(val::Start),
    SafeStart(val::SafeStart),
    UnsafeStart(val::UnsafeStart),
    End(val::End),
    SafeEnd(val::SafeEnd),
    UnsafeEnd(val::UnsafeEnd),
    FlexStart(val::FlexStart),
    SafeFlexStart(val::SafeFlexStart),
    UnsafeFlexStart(val::UnsafeFlexStart),
    FlexEnd(val::FlexEnd),
    SafeFlexEnd(val::SafeFlexEnd),
    UnsafeFlexEnd(val::UnsafeFlexEnd),
    Left(val::Left),
    SafeLeft(val::SafeLeft),
    UnsafeLeft(val::UnsafeLeft),
    Right(val::Right),
    SafeRight(val::SafeRight),
    UnsafeRight(val::UnsafeRight),
}

impl UpdateStyleValues for JustifyContent {
    fn update_style_values(self, values: StyleValues) -> StyleValues {
        values.add(St::JustifyContent, self)
    }
}

#[derive(Clone, Debug, Copy, PartialEq, Eq, Display, From)]
pub enum AlignContent {
    Normal(val::Normal),
    Baseline(val::Baseline),
    FirstBaseline(val::FirstBaseline),
    LastBaseline(val::LastBaseline),
    SpaceBetween(val::SpaceBetween),
    SpaceAround(val::SpaceAround),
    SpaceEvenly(val::SpaceEvenly),
    Stretch(val::Stretch),
    Center(val::Center),
    SafeCenter(val::SafeCenter),
    UnsafeCenter(val::UnsafeCenter),
    Start(val::Start),
    SafeStart(val::SafeStart),
    UnsafeStart(val::UnsafeStart),
    End(val::End),
    SafeEnd(val::SafeEnd),
    UnsafeEnd(val::UnsafeEnd),
    FlexStart(val::FlexStart),
    SafeFlexStart(val::SafeFlexStart),
    UnsafeFlexStart(val::UnsafeFlexStart),
    FlexEnd(val::FlexEnd),
    SafeFlexEnd(val::SafeFlexEnd),
    UnsafeFlexEnd(val::UnsafeFlexEnd),
}

impl UpdateStyleValues for AlignContent {
    fn update_style_values(self, values: StyleValues) -> StyleValues {
        values.add(St::AlignContent, self)
    }
}

#[derive(Clone, Debug, Copy, PartialEq, Eq, Display, From)]
pub enum AlignItems {
    Normal(val::Normal),
    Stretch(val::Stretch),
    Baseline(val::Baseline),
    FirstBaseline(val::FirstBaseline),
    LastBaseline(val::LastBaseline),
    Center(val::Center),
    SafeCenter(val::SafeCenter),
    UnsafeCenter(val::UnsafeCenter),
    Start(val::Start),
    SafeStart(val::SafeStart),
    UnsafeStart(val::UnsafeStart),
    End(val::End),
    SafeEnd(val::SafeEnd),
    UnsafeEnd(val::UnsafeEnd),
    SelfStart(val::SelfStart),
    SafeSelfStart(val::SafeSelfStart),
    UnsafeSelfStart(val::UnsafeSelfStart),
    SelfEnd(val::SelfEnd),
    SafeSelfEnd(val::SafeSelfEnd),
    UnsafeSelfEnd(val::UnsafeSelfEnd),
    FlexStart(val::FlexStart),
    SafeFlexStart(val::SafeFlexStart),
    UnsafeFlexStart(val::UnsafeFlexStart),
    FlexEnd(val::FlexEnd),
    SafeFlexEnd(val::SafeFlexEnd),
    UnsafeFlexEnd(val::UnsafeFlexEnd),
}

impl UpdateStyleValues for AlignItems {
    fn update_style_values(self, values: StyleValues) -> StyleValues {
        values.add(St::AlignItems, self)
    }
}

#[derive(Clone, Debug, Copy, PartialEq, Eq, Display, From)]
pub enum JustifySelf {
    Auto(val::Auto),
    Normal(val::Normal),
    Stretch(val::Stretch),
    Baseline(val::Baseline),
    FirstBaseline(val::FirstBaseline),
    LastBaseline(val::LastBaseline),
    Center(val::Center),
    SafeCenter(val::SafeCenter),
    UnsafeCenter(val::UnsafeCenter),
    Start(val::Start),
    SafeStart(val::SafeStart),
    UnsafeStart(val::UnsafeStart),
    End(val::End),
    SafeEnd(val::SafeEnd),
    UnsafeEnd(val::UnsafeEnd),
    SelfStart(val::SelfStart),
    SafeSelfStart(val::SafeSelfStart),
    UnsafeSelfStart(val::UnsafeSelfStart),
    SelfEnd(val::SelfEnd),
    SafeSelfEnd(val::SafeSelfEnd),
    UnsafeSelfEnd(val::UnsafeSelfEnd),
    FlexStart(val::FlexStart),
    SafeFlexStart(val::SafeFlexStart),
    UnsafeFlexStart(val::UnsafeFlexStart),
    FlexEnd(val::FlexEnd),
    SafeFlexEnd(val::SafeFlexEnd),
    UnsafeFlexEnd(val::UnsafeFlexEnd),
    Left(val::Left),
    SafeLeft(val::SafeLeft),
    UnsafeLeft(val::UnsafeLeft),
    Right(val::Right),
    SafeRight(val::SafeRight),
    UnsafeRight(val::UnsafeRight),
}

impl UpdateStyleValues for JustifySelf {
    fn update_style_values(self, values: StyleValues) -> StyleValues {
        values.add(St::JustifySelf, self)
    }
}

#[derive(Clone, Debug, Copy, PartialEq, Eq, Display, From)]
pub enum AlignSelf {
    Auto(val::Auto),
    Normal(val::Normal),
    Stretch(val::Stretch),
    Baseline(val::Baseline),
    FirstBaseline(val::FirstBaseline),
    LastBaseline(val::LastBaseline),
    Center(val::Center),
    SafeCenter(val::SafeCenter),
    UnsafeCenter(val::UnsafeCenter),
    Start(val::Start),
    SafeStart(val::SafeStart),
    UnsafeStart(val::UnsafeStart),
    End(val::End),
    SafeEnd(val::SafeEnd),
    UnsafeEnd(val::UnsafeEnd),
    SelfStart(val::SelfStart),
    SafeSelfStart(val::SafeSelfStart),
    UnsafeSelfStart(val::UnsafeSelfStart),
    SelfEnd(val::SelfEnd),
    SafeSelfEnd(val::SafeSelfEnd),
    UnsafeSelfEnd(val::UnsafeSelfEnd),
    FlexStart(val::FlexStart),
    SafeFlexStart(val::SafeFlexStart),
    UnsafeFlexStart(val::UnsafeFlexStart),
    FlexEnd(val::FlexEnd),
    SafeFlexEnd(val::SafeFlexEnd),
    UnsafeFlexEnd(val::UnsafeFlexEnd),
}

impl UpdateStyleValues for AlignSelf {
    fn update_style_values(self, values: StyleValues) -> StyleValues {
        values.add(St::AlignSelf, self)
    }
}
