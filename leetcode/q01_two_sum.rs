use std::collections::HashMap;

pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut ret_vec: Vec<i32> = Vec::new();
    let mut lookup = HashMap::new();
    for (i, elem) in nums.iter().enumerate() {
        let diff = target - elem;
        if lookup.contains_key(elem) {
            ret_vec.push(lookup.remove(elem).unwrap());
            ret_vec.push(i as i32);
            break;
        }
        lookup.insert(diff, i as i32);
    }
    return ret_vec;
}