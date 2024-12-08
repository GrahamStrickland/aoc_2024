mod day1;

pub use crate::day1::list_diff;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_list_diff() {
        let list1 = vec![3, 4, 2, 1, 3, 3];
        let list2 = vec![4, 3, 5, 3, 9, 3];
        let result = list_diff(&list1, &list2);

        assert_eq!(result, 11)
    }
}
