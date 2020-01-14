macro css_values( $( ($name:ident, $cssvalue:literal) $(,)? )* ) {
    $(
        #[derive(Clone, Copy, Debug, PartialEq, Eq, Display)]
        #[display(fmt = $cssvalue)]
        pub struct $name;
    )*
}

css_values! {
    (Initial, "initial"), (Inherit, "inherit"), (Thin, "thin"), (Medium, "medium"),
    (Thick, "thick"), (None, "none"), (Hidden, "hidden"), (Dotted, "dotted"),
    (Dashed, "dashed"), (Solid, "solid"), (Double, "double"), (Groove, "groove"),
    (Ridge, "ridge"), (Inset, "inset"), (Outset, "outset"), (Auto, "auto"),
    (MinContent, "min-content"), (MaxContent, "max-content"), (Content, "content"),
    (Row, "row"), (RowReverse, "row-reverse"), (Column, "column"), (ColumnReverse, "column-reverse"),
    (Wrap,"wrap"), (NoWrap,"nowrap"), (WrapReverse,"wrap-reverse"),
    (Normal,"normal"), (SpaceBetween,"space-between"), (SpaceAround,"space-around"),
    (SpaceEvenly,"space-evenly"), (Stretch,"stretch"), (Center,"center"), (SafeCenter,"safe center"),
    (UnsafeCenter,"unsafe center"), (Start,"start"), (SafeStart,"safe start"), (UnsafeStart,"unsafe start"),
    (End,"end"), (SafeEnd,"safe end"), (UnsafeEnd,"unsafe end"), (FlexStart,"flex-start"),
    (SafeFlexStart,"safe flex-start"), (UnsafeFlexStart,"unsafe flex-start"), (FlexEnd,"flex-end"),
    (SafeFlexEnd,"safe flex-end"), (UnsafeFlexEnd,"unsafe flex-end"), (Left,"left"), (SafeLeft,"safe left"),
    (UnsafeLeft,"unsafe left"), (Right,"right"), (SafeRight,"safe right"), (UnsafeRight,"unsafe right"),
    (Baseline,"baseline"), (FirstBaseline,"first baseline"), (LastBaseline,"last baseline"),
    (SelfStart,"self-start"), (SafeSelfStart,"safe self-start"), (UnsafeSelfStart,"unsafe self-start"),
    (SelfEnd,"self-end"), (SafeSelfEnd,"safe self-end"), (UnsafeSelfEnd,"unsafe self-end"),
    (RepeatX, "repeat-x"), (RepeatY, "repeat-y"), (Repeat, "repeat"), (Space, "space"), (Round, "round"),
    (NoRepeat, "no-repeat"), (Scroll, "scroll"), (Fixed, "fixed"), (Local, "local"), (BorderBox, "border-box"),
    (PaddingBox, "padding-box"), (ContentBox, "content-box"), (Cover, "cover"), (Contain, "contain"),
    (Top, "top"), (Bottom, "bottom"), (Flex, "flex"), (Relative, "relative"), (Absolute, "absolute"),
    (InlineBlock, "inline-block"), (Block, "block"), (Ease, "ease"), (Linear, "linear"), (EaseIn, "ease-in"),
    (EaseOut, "ease-out"), (EaseInOut, "ease-in-out"), (StepStart, "step-start"), (StepEnd, "step-end"),
}
