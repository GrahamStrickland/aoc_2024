pub mod day1;

pub use crate::day1::list_diff;
pub use crate::day1::read_file;
pub use crate::day1::read_lists;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_read_file() {
        let result = read_file("data/test_day1.tsv");
        assert_eq!(result, "3   4\n4   3\n2   5\n1   3\n3   9\n3   3\n");
    }

    #[test]
    fn test_read_lists() {
        let content = "3   4\n4   3\n2   5\n1   3\n3   9\n3   3\n";
        let res = read_lists(content.as_bytes());
        assert!(res.is_ok(), "Failed to read contents");
        assert_eq!(
            res.ok(),
            Some((vec![3, 4, 2, 1, 3, 3], vec![4, 3, 5, 3, 9, 3]))
        );
    }

    #[test]
    fn test_list_diff() {
        let mut list1 = vec![3, 4, 2, 1, 3, 3];
        let mut list2 = vec![4, 3, 5, 3, 9, 3];
        let result = list_diff(&mut list1, &mut list2);

        assert_eq!(result, 11)
    }
}
