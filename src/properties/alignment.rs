#[derive(Clone, Default)]
pub struct Alignment {
    horizontal: Option<Align>,
    vertical: Option<Align>,
}

#[derive(Clone)]
enum Align {
    Stretch,
    Start,
    End,
    Center,
    Baseline,
}
