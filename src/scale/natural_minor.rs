use super::scale::Scale;

#[derive(Debug, Copy, Clone)]
pub struct NaturalMinor {
    pub root: u32,
}

impl Scale for NaturalMinor {
    fn new(root: u32) -> NaturalMinor {
        return NaturalMinor { root };
    }

    fn notes(&self) -> Vec<u32> {
        let n1: u32 = self.root as u32;
        let n2 = n1 + 2;
        let n3 = n2 + 1;
        let n4 = n3 + 2;
        let n5 = n4 + 2;
        let n6 = n5 + 1;
        let n7 = n6 + 2;

        return vec![n1, n2, n3, n4, n5, n6, n7];
    }

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