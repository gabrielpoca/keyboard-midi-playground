pub trait Scale: Copy {
    fn new(root: u32) -> Self;
    fn notes(&self) -> Vec<u32>;
    fn note(&self, position: i32) -> u32;
}
