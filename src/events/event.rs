use termion::event::Key;

#[derive(Debug, Clone, Copy)]
pub enum NoteMessage {
    On = 0x90,
    Off = 0x80,
}

#[derive(Debug, Clone)]
pub enum Event {
    None,
    Note {
        message: NoteMessage,
        note: u8,
        velocity: u8,
    },
    Quit,
}
