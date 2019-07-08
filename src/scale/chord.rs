use super::scale::Scale;

pub struct Chord<T: Scale> {
    scale: T,
}

impl<S: Scale> Chord<S> {
    pub fn new(scale: S) -> Self {
        return Chord { scale };
    }

    pub fn get(&self, p: u32) -> Vec<u32> {
        let position = self
            .scale
            .notes()
            .iter()
            .position(|&n| n == p)
            .unwrap_or_else(|| 0);

        let base = self.scale.note(position as i32);
        let second = self.scale.note((position + 2) as i32);
        let third = self.scale.note((position + 4) as i32);

        return vec![p, p + (second - base), p + (third - base)];
    }
}
