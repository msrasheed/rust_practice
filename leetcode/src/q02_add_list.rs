#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
  pub val: i32,
  pub next: Option<Box<ListNode>>
}

// commented out to quiet compile warnings
// impl ListNode {
//   #[inline]
//   fn new(val: i32) -> Self {
//     ListNode {
//       next: None,
//       val
//     }
//   }
// }

// ACCEPTED
pub fn add_two_numbers(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let mut l1 = l1;
    let mut l2 = l2;
    let mut remainder = 0;
    let mut ret_list: Option<Box<ListNode>> = None;
    let mut res_nums: Vec<i32> = Vec::new();
    loop {
        let val1;
        let val2;
        if l1.is_none() && l2.is_none() && remainder == 0 {break;}
        if let Some(node) = l1 {
            val1 = node.val;
            l1 = node.next;
        } else {
            val1 = 0;
        }
        if let Some(node) = l2 {
            val2 = node.val;
            l2 = node.next;
        } else {
            val2 = 0;
        }
        let mut add = val1 + val2 + remainder;
        if add > 9 {
            remainder = 1;
            add -= 10;
        } else {
            remainder = 0;
        }
        // println!("{val1} + {val2} = {add}, {remainder}");
        res_nums.push(add);
    }

    for val in res_nums.iter().rev() {
        ret_list = Some(Box::new(ListNode {
            next: ret_list,
            val: *val
        }))
    }
    ret_list
}