#[derive(Clone, Debug, Copy, PartialEq, Eq, Display)]
pub enum JustifyContent {
    #[display(fmt = "normal")]
    Normal,
    #[display(fmt = "space-between")]
    SpaceBetween,
    #[display(fmt = "space-around")]
    SpaceAround,
    #[display(fmt = "space-evenly")]
    SpaceEvenly,
    #[display(fmt = "stretch")]
    Stretch,
    #[display(fmt = "center")]
    Center,
    #[display(fmt = "safe center")]
    SafeCenter,
    #[display(fmt = "unsafe center")]
    UnsafeCenter,
    #[display(fmt = "start")]
    Start,
    #[display(fmt = "safe start")]
    SafeStart,
    #[display(fmt = "unsafe start")]
    UnsafeStart,
    #[display(fmt = "end")]
    End,
    #[display(fmt = "safe end")]
    SafeEnd,
    #[display(fmt = "unsafe end")]
    UnsafeEnd,
    #[display(fmt = "flex-start")]
    FlexStart,
    #[display(fmt = "safe flex-start")]
    SafeFlexStart,
    #[display(fmt = "unsafe flex-start")]
    UnsafeFlexStart,
    #[display(fmt = "flex-end")]
    FlexEnd,
    #[display(fmt = "safe flex-end")]
    SafeFlexEnd,
    #[display(fmt = "unsafe flex-end")]
    UnsafeFlexEnd,
    #[display(fmt = "left")]
    Left,
    #[display(fmt = "safe left")]
    SafeLeft,
    #[display(fmt = "unsafe left")]
    UnsafeLeft,
    #[display(fmt = "right")]
    Right,
    #[display(fmt = "safe right")]
    SafeRight,
    #[display(fmt = "unsafe right")]
    UnsafeRight,
}

#[derive(Clone, Debug, Copy, PartialEq, Eq, Display)]
pub enum AlignContent {
    #[display(fmt = "normal")]
    Normal,
    #[display(fmt = "baseline")]
    Baseline,
    #[display(fmt = "first baseline")]
    FirstBaseline,
    #[display(fmt = "last baseline")]
    LastBaseline,
    #[display(fmt = "space-between")]
    SpaceBetween,
    #[display(fmt = "space-around")]
    SpaceAround,
    #[display(fmt = "space-evenly")]
    SpaceEvenly,
    #[display(fmt = "stretch")]
    Stretch,
    #[display(fmt = "center")]
    Center,
    #[display(fmt = "safe center")]
    SafeCenter,
    #[display(fmt = "unsafe center")]
    UnsafeCenter,
    #[display(fmt = "start")]
    Start,
    #[display(fmt = "safe start")]
    SafeStart,
    #[display(fmt = "unsafe start")]
    UnsafeStart,
    #[display(fmt = "end")]
    End,
    #[display(fmt = "safe end")]
    SafeEnd,
    #[display(fmt = "unsafe end")]
    UnsafeEnd,
    #[display(fmt = "flex-start")]
    FlexStart,
    #[display(fmt = "safe flex-start")]
    SafeFlexStart,
    #[display(fmt = "unsafe flex-start")]
    UnsafeFlexStart,
    #[display(fmt = "flex-end")]
    FlexEnd,
    #[display(fmt = "safe flex-end")]
    SafeFlexEnd,
    #[display(fmt = "unsafe flex-end")]
    UnsafeFlexEnd,
}

#[derive(Clone, Debug, Copy, PartialEq, Eq, Display)]
pub enum AlignItems {
    #[display(fmt = "normal")]
    Normal,
    #[display(fmt = "stretch")]
    Stretch,
    #[display(fmt = "baseline")]
    Baseline,
    #[display(fmt = "first baseline")]
    FirstBaseline,
    #[display(fmt = "last baseline")]
    LastBaseline,
    #[display(fmt = "center")]
    Center,
    #[display(fmt = "safe center")]
    SafeCenter,
    #[display(fmt = "unsafe center")]
    UnsafeCenter,
    #[display(fmt = "start")]
    Start,
    #[display(fmt = "safe start")]
    SafeStart,
    #[display(fmt = "unsafe start")]
    UnsafeStart,
    #[display(fmt = "end")]
    End,
    #[display(fmt = "safe end")]
    SafeEnd,
    #[display(fmt = "unsafe end")]
    UnsafeEnd,
    #[display(fmt = "self-start")]
    SelfStart,
    #[display(fmt = "safe self-start")]
    SafeSelfStart,
    #[display(fmt = "unsafe self-start")]
    UnsafeSelfStart,
    #[display(fmt = "self-end")]
    SelfEnd,
    #[display(fmt = "safe self-end")]
    SafeSelfEnd,
    #[display(fmt = "unsafe self-end")]
    UnsafeSelfEnd,
    #[display(fmt = "flex-start")]
    FlexStart,
    #[display(fmt = "safe flex-start")]
    SafeFlexStart,
    #[display(fmt = "unsafe flex-start")]
    UnsafeFlexStart,
    #[display(fmt = "flex-end")]
    FlexEnd,
    #[display(fmt = "safe flex-end")]
    SafeFlexEnd,
    #[display(fmt = "unsafe flex-end")]
    UnsafeFlexEnd,
}

#[derive(Clone, Debug, Copy, PartialEq, Eq, Display)]
pub enum JustifySelf {
    #[display(fmt = "auto")]
    Auto,
    #[display(fmt = "normal")]
    Normal,
    #[display(fmt = "stretch")]
    Stretch,
    #[display(fmt = "baseline")]
    Baseline,
    #[display(fmt = "first baseline")]
    FirstBaseline,
    #[display(fmt = "last baseline")]
    LastBaseline,
    #[display(fmt = "center")]
    Center,
    #[display(fmt = "safe center")]
    SafeCenter,
    #[display(fmt = "unsafe center")]
    UnsafeCenter,
    #[display(fmt = "start")]
    Start,
    #[display(fmt = "safee start")]
    SafeeStart,
    #[display(fmt = "unsafee start")]
    UnsafeeStart,
    #[display(fmt = "end")]
    End,
    #[display(fmt = "safee end")]
    SafeeEnd,
    #[display(fmt = "unsafee end")]
    UnsafeeEnd,
    #[display(fmt = "self-start")]
    SelfStart,
    #[display(fmt = "safee self-start")]
    SafeeSelfStart,
    #[display(fmt = "unsafee self-start")]
    UnsafeeSelfStart,
    #[display(fmt = "self-end")]
    SelfEnd,
    #[display(fmt = "safee self-end")]
    SafeeSelfEnd,
    #[display(fmt = "unsafee self-end")]
    UnsafeeSelfEnd,
    #[display(fmt = "flex-start")]
    FlexStart,
    #[display(fmt = "safee flex-start")]
    SafeeFlexStart,
    #[display(fmt = "unsafee flex-start")]
    UnsafeeFlexStart,
    #[display(fmt = "flex-end")]
    FlexEnd,
    #[display(fmt = "safee flex-end")]
    SafeeFlexEnd,
    #[display(fmt = "unsafee flex-end")]
    UnsafeeFlexEnd,
    #[display(fmt = "left")]
    Left,
    #[display(fmt = "safee left")]
    SafeeLeft,
    #[display(fmt = "unsafee left")]
    UnsafeeLeft,
    #[display(fmt = "right")]
    Right,
    #[display(fmt = "safee right")]
    SafeeRight,
    #[display(fmt = "unsafee right")]
    UnsafeeRight,
}

#[derive(Clone, Debug, Copy, PartialEq, Eq, Display)]
pub enum AlignSelf {
    #[display(fmt = "auto")]
    Auto,
    #[display(fmt = "normal")]
    Normal,
    #[display(fmt = "stretch")]
    Stretch,
    #[display(fmt = "baseline")]
    Baseline,
    #[display(fmt = "first baseline")]
    FirstBaseline,
    #[display(fmt = "last baseline")]
    LastBaseline,
    #[display(fmt = "center")]
    Center,
    #[display(fmt = "safee center")]
    SafeeCenter,
    #[display(fmt = "unsafee center")]
    UnsafeeCenter,
    #[display(fmt = "start")]
    Start,
    #[display(fmt = "safee start")]
    SafeeStart,
    #[display(fmt = "unsafee start")]
    UnsafeeStart,
    #[display(fmt = "end")]
    End,
    #[display(fmt = "safee end")]
    SafeeEnd,
    #[display(fmt = "unsafee end")]
    UnsafeeEnd,
    #[display(fmt = "self-start")]
    SelfStart,
    #[display(fmt = "safee self-start")]
    SafeeSelfStart,
    #[display(fmt = "unsafee self-start")]
    UnsafeeSelfStart,
    #[display(fmt = "self-end")]
    SelfEnd,
    #[display(fmt = "safee self-end")]
    SafeeSelfEnd,
    #[display(fmt = "unsafee self-end")]
    UnsafeeSelfEnd,
    #[display(fmt = "flex-start")]
    FlexStart,
    #[display(fmt = "safee flex-start")]
    SafeeFlexStart,
    #[display(fmt = "unsafee flex-start")]
    UnsafeeFlexStart,
    #[display(fmt = "flex-end")]
    FlexEnd,
    #[display(fmt = "safee flex-end")]
    SafeeFlexEnd,
    #[display(fmt = "unsafee flex-end")]
    UnsafeeFlexEnd,
}
