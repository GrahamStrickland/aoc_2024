mod day1;

pub use crate::day1::list_diff;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_list_diff() {
        let result = list_diff();
        assert_eq!(result, 0)
    }
}
