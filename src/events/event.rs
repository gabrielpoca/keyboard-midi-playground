use std::fmt;

#[derive(Debug, Clone, Copy)]
pub enum NoteMessage {
    On = 0x90,
    Off = 0x80,
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub enum Key {
    Space,
    Num0,
    Num1,
    Num2,
    Num3,
    Num4,
    Num5,
    Num6,
    Num7,
    Num8,
    Num9,
    None,
    Q,
    W,
    E,
    R,
    T,
    Y,
    U,
    I,
    O,
    P,
    A,
    S,
    D,
    F,
    G,
    H,
    J,
    K,
    L,
    Z,
    X,
    C,
    V,
    B,
    N,
    M,
}

impl fmt::Display for Key {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Debug, Clone)]
pub enum Event {
    None,
    Note {
        message: NoteMessage,
        note: u8,
        velocity: u8,
    },
    KeyDown(Key),
    KeyUp(Key),
    Quit,
}
