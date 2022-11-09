// This stub file contains items that aren't used yet; feel free to remove this module attribute
// to enable stricter warnings.
#![allow(unused)]

pub fn divmod(dividend: i16, divisor: i16) -> (i16, i16) {
    let result: (i16, i16) = (dividend/divisor, dividend%divisor);
    result
}

pub fn evens<T>(iter: impl Iterator<Item = T>) -> impl Iterator<Item = T> {
    // iter.enumerate().filter(|pair|{pair.0%2==0}).map(|pair|pair.1)
    iter.enumerate().filter_map(|pair|{
        match pair.0%2{
            0 => Some(pair.1),
            _ => None,
        }
    })
}

pub struct Position(pub i16, pub i16);
impl Position {
    pub fn manhattan(&self) -> i16 {
        self.0.abs()+self.1.abs()

    }
}
