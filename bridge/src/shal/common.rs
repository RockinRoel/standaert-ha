#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub(super) enum Edge {
    Rising,
    Falling,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub(super) enum IsWas {
    Was,
    Is,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub(super) enum Value {
    Low,
    High,
}
