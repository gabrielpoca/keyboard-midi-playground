use super::scale::Scale;

#[derive(Debug)]
pub struct MelodicMinor {
    pub root: u32,
}

unsafe impl Sync for MelodicMinor {}
unsafe impl Send for MelodicMinor {}

impl MelodicMinor {
    pub fn new(root: u32) -> MelodicMinor {
        return MelodicMinor { root };
    }
}

impl Scale for MelodicMinor {
    fn increase_root(&mut self, nr: u32) {
        self.root = self.root + nr;
    }

    fn decrease_root(&mut self, nr: u32) {
        self.root = self.root - nr;
    }

    fn label(&self) -> String {
        return "Melodic Minor".into();
    }

    fn notes(&self) -> Vec<u32> {
        let n1: u32 = self.root as u32;
        let n2 = n1 + 2;
        let n3 = n2 + 1;
        let n4 = n3 + 2;
        let n5 = n4 + 2;
        let n6 = n5 + 2;
        let n7 = n6 + 2;

        return vec![n1, n2, n3, n4, n5, n6, n7];
    }
}
