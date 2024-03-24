// ACCEPTED
pub fn length_of_longest_substring(s: String) -> i32 {
    let bytes = s.as_bytes();
    let mut char_ptr = 0;
    let mut max_size = 0;
    let mut curr_size = 0;
    for (i, _) in bytes.iter().enumerate() {
        if let Some(idx) = s[char_ptr..i].find(&s[i..i+1]) {
            char_ptr += idx;
            curr_size = i - char_ptr;
            char_ptr += 1;
        } else {
            curr_size += 1;
        }

        if curr_size > max_size {
            max_size = curr_size;
        }
    }

    max_size as i32
}

