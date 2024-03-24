// ACCEPTED
pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
    let totlen = nums1.len() + nums2.len();
    let mut ptr1 = 0;
    let mut ptr2 = 0;
    let mut vals = vec![0, 0];

    for _ in 0..totlen {
        let val;
        if ptr1 == nums1.len() {
            val = nums2[ptr2];
            ptr2 += 1;
        } else if ptr2 == nums2.len() {
            val = nums1[ptr1];
            ptr1 += 1;
        } else if nums1[ptr1] > nums2[ptr2] {
            val = nums2[ptr2];
            ptr2 += 1;
        } else {
            val = nums1[ptr1];
            ptr1 += 1;
        }
        let totptr = ptr1 + ptr2 - 1;
        vals[totptr % 2] = val;
        if totptr == totlen / 2 {
            break;
        }
    }

    if totlen % 2 == 0 {
        ((vals[0] as f64) + (vals[1] as f64)) / 2.0
    } else {
        vals[(totlen / 2) % 2] as f64
    }
}

