use super::scale::Scale;

pub fn get(scale: &Box<dyn Scale + Sync + Send>, p: u32) -> Vec<u32> {
    let position = scale
        .notes()
        .iter()
        .position(|&n| n == p)
        .unwrap_or_else(|| 0);

    let base = scale.note(position as i32);
    let second = scale.note((position + 2) as i32);
    let third = scale.note((position + 4) as i32);

    return vec![p, p + (second - base), p + (third - base)];
}
