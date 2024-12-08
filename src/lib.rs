mod day1;

pub use crate::day1::list_diff;
pub use crate::day1::read;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_read() {
        let content = "3\t4\n4\t3\n2\t5\n1\t3\n3\t9\n3\t3\n";
        let res = read(content.as_bytes());
        assert!(res.is_ok(), "Failed to read contents");
        assert_eq!(res.ok(), Some((vec![3, 4, 2, 1, 3, 3], vec![4, 3, 5, 3, 9, 3])));
    }

    #[test]
    fn test_list_diff() {
        let list1 = vec![3, 4, 2, 1, 3, 3];
        let list2 = vec![4, 3, 5, 3, 9, 3];
        let result = list_diff(&list1, &list2);

        assert_eq!(result, 11)
    }
}
