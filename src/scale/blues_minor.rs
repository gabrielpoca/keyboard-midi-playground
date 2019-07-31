use super::scale::Scale;

pub struct BluesMinor {
    pub root: u32,
}

unsafe impl Sync for BluesMinor {}
unsafe impl Send for BluesMinor {}

impl BluesMinor {
    pub fn new(root: u32) -> BluesMinor {
        return BluesMinor { root };
    }
}

impl Scale for BluesMinor {
    fn increase_root(&mut self, nr: u32) {
        self.root = self.root + nr;
    }

    fn decrease_root(&mut self, nr: u32) {
        self.root = self.root - nr;
    }

    fn label(&self) -> String {
        return "Blues Minor".into();
    }

    fn notes(&self) -> Vec<u32> {
        let n1: u32 = self.root as u32;
        let n2 = n1 + 3;
        let n3 = n2 + 2;
        let n4 = n3 + 3;
        let n5 = n4 + 2;

        return vec![n1, n2, n3, n4, n5];
    }
}
