use super::scale::Scale;

pub struct PentatonicMinor {
    pub root: u32,
}

unsafe impl Sync for PentatonicMinor {}
unsafe impl Send for PentatonicMinor {}

impl PentatonicMinor {
    pub fn new(root: u32) -> PentatonicMinor {
        return PentatonicMinor { root };
    }
}

impl Scale for PentatonicMinor {
    fn increase_root(&mut self, nr: u32) {
        self.root = self.root + nr;
    }

    fn decrease_root(&mut self, nr: u32) {
        self.root = self.root - nr;
    }

    fn label(&self) -> String {
        return "Pentatonic Minor".into();
    }

    fn notes(&self) -> Vec<u32> {
        let n1: u32 = self.root as u32;
        let n2 = n1 + 3;
        let n3 = n2 + 2;
        let n4 = n3 + 2;
        let n5 = n4 + 3;

        return vec![n1, n2, n3, n4, n5];
    }
}
