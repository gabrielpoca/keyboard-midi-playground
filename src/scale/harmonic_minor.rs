use super::scale::Scale;

#[derive(Debug)]
pub struct HarmonicMinor {
    pub root: u32,
}

impl HarmonicMinor {
    pub fn new(root: u32) -> HarmonicMinor {
        return HarmonicMinor { root };
    }
}

impl Scale for HarmonicMinor {
    fn label(&self) -> String {
        return "Harmonic Minor".into();
    }

    fn notes(&self) -> Vec<u32> {
        let n1: u32 = self.root as u32;
        let n2 = n1 + 2;
        let n3 = n2 + 1;
        let n4 = n3 + 2;
        let n5 = n4 + 2;
        let n6 = n5 + 1;
        let n7 = n6 + 3;

        return vec![n1, n2, n3, n4, n5, n6, n7];
    }
}
