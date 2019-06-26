pub enum NoteMessage {
    On = 0x90,
    Off = 0x80,
}

pub struct Note {
    pub message: NoteMessage,
    pub note: u8,
    pub velocity: u8,
}
