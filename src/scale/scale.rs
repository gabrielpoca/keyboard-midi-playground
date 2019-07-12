pub trait Scale {
    fn increase_root(&mut self, nr: u32);
    fn decrease_root(&mut self, nr: u32);

    fn label(&self) -> String;

    fn notes(&self) -> Vec<u32>;

    fn note(&self, position: i32) -> u32 {
        let notes = self.notes();
        let mut position = position;
        let mut base: i32 = 0;

        while position >= notes.len() as i32 {
            position -= notes.len() as i32;
            base += 12;
        }

        while position < 0 {
            position += notes.len() as i32;
            base -= 12;
        }

        let note = notes[position as usize] as i32;

        return (note + base) as u32;
    }
}
