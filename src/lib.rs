pub mod operations;
pub fn add(left: usize, right: usize) -> usize {
    left + right
}
mod entities;
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
