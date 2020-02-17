use crate::css::{values as val, St, StyleMap, ToStyleMap};

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

impl ToStyleMap for JustifyContent {
    fn style_map(&self) -> StyleMap {
        let mut map = StyleMap::default();
        map.add(St::JustifyContent, self);
        map
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

impl ToStyleMap for AlignContent {
    fn style_map(&self) -> StyleMap {
        let mut map = StyleMap::default();
        map.add(St::AlignContent, self);
        map
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

impl ToStyleMap for AlignItems {
    fn style_map(&self) -> StyleMap {
        let mut map = StyleMap::default();
        map.add(St::AlignItems, self);
        map
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

impl ToStyleMap for JustifySelf {
    fn style_map(&self) -> StyleMap {
        let mut map = StyleMap::default();
        map.add(St::JustifySelf, self);
        map
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

impl ToStyleMap for AlignSelf {
    fn style_map(&self) -> StyleMap {
        let mut map = StyleMap::default();
        map.add(St::AlignSelf, self);
        map
    }
}
