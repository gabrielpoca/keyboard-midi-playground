use crate::player::PlayerNote;
use super::scale::Scale;

pub struct Chord<T: Scale> {
    scale: T,
}

impl<S: Scale> Chord<S> {
    pub fn new(scale: S) -> Self {
        return Chord { scale };
    }

    pub fn get_notes(&self, p: PlayerNote) -> Vec<PlayerNote> {
        let position = self
            .scale
            .notes()
            .iter()
            .position(|&n| n == p.note)
            .unwrap_or_else(|| 0);

        let base = self.scale.note(position as i32);
        let second = self.scale.note((position + 2) as i32);
        let third = self.scale.note((position + 5) as i32);

        return vec![
            p,
            PlayerNote {
                note: p.note + (second - base),
                ..p
            },
            PlayerNote {
                note: p.note + (third - base),
                ..p
            },
        ];
    }
}
