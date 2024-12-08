pub fn list_diff(list1: &Vec<i32>, list2: &Vec<i32>) -> i32 {
    let mut diff = 0;
    let mut new_list1 = list1.clone();
    let mut new_list2 = list2.clone();

    new_list1.sort();
    new_list2.sort();

    for (item1, item2) in new_list1.iter().zip(new_list2.iter()) {
        diff += (item1 - item2).abs()
    }

    diff
}
