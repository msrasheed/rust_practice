// use std::collections::VecDeque;

// Initial solution... NOT ACCEPTED
// Didn't realize ends up being a tree and revisits already processed
// start/ends so runtime becomes exponential and intractable for larger strings.
// Could do some kind of "memoizing" of what start/ends I've seen but at this point lost
// hope in this approach
//
// tried to do see if tails of possible palindrome could be used to determine plausible next
// checking start/end points. First palindrome found guaranteed to be longest
//
// pub fn longest_palindrome(s: String) -> String {
//     let bytes = s.as_bytes();
//     let mut queue: VecDeque<(usize, usize)> = VecDeque::with_capacity(s.len());
//     let mut frontptr = 0;
//     let mut backptr = s.len()-1;
//
//     queue.push_back((frontptr, backptr));
//
//     while let Some((mut front, mut back)) = queue.pop_front() {
//         frontptr = front;
//         backptr = back;
//
//         println!("{front} {back}, {}", &s[front..(back+1)]);
//
//         if (back - front) == 0 {break;}
//
//         if bytes[front] != bytes[back] {
//             queue.push_back((front, back-1));
//             queue.push_back((front+1, back));
//             continue;
//         }
//
//         let beginchar = bytes[front];
//         let mut frontchar = None;
//         let mut backchar = None;
//         front += 1;
//         back -= 1;
//
//         while (front < back) && bytes[front] == bytes[back] {
//             if (bytes[front] == beginchar) && frontchar.is_none() {
//                 frontchar = Some(front);
//                 backchar = Some(back);
//             }
//             front += 1;
//             back -= 1;
//         }
//
//         if front >= back {
//             break;
//         }
//
//         queue.push_back((frontptr, backchar.unwrap_or(back)));
//         queue.push_back((frontchar.unwrap_or(front), backptr));
//     }
//     (&s[frontptr..(backptr+1)]).to_string()
// }

// Second solution... ACCPETED
// frustrated from above so did a simple sliding window.
// first palindrome found is guaranteed to be longest
pub fn longest_palindrome(s: String) -> String {
    let bytes = s.as_bytes();
    let mut size = s.len();
    let mut frontres = 0;

    for i in (0..s.len()).rev() {
        size = i + 1;
        let mut found: bool = false;

        for j in 0..(s.len() - size + 1) {
            let mut front = j;
            let mut back = front + size - 1;
            frontres = front;
            
            while (front < back) && bytes[front] == bytes[back] {
                front += 1;
                back -= 1;
            }
            
            if front >= back {
                found = true;
                break;
            }
        }

        if found {break;}
    }
    (&s[frontres..(frontres + size)]).to_string()
}

// Will try again. After reading a bit saw approach about starting from
// possible "centers" from palindrome and the mentioning of a dynamic programming approach
// (didn't read what the DP approach details were tho).
// So I can now imagin a DP approach I'll try out later. 

#[cfg(test)]
mod palindrome_test {
    use super::*;

    #[test]
    fn examples() {
        let s = String::from("babad");
        let res = longest_palindrome(s);
        assert_eq!(res, "bab");


        let s = String::from("cbbd");
        let res = longest_palindrome(s);
        assert_eq!(res, "bb");

        let s = String::from("xaabacxcabaaxcabaax");
        let res = longest_palindrome(s);
        assert_eq!(res, "xaabacxcabaax");

        // let s = String::from("xaabacxcabaaxcabaax");
        //                              |   |

        let s = String::from("a");
        let res = longest_palindrome(s);
        assert_eq!(res, "a");

        let s = String::from("bb");
        let res = longest_palindrome(s);
        assert_eq!(res, "bb");

        let s = String::from("civilwartestingwhetherthatnaptionoranynartionsoconceivedandsodedicatedcanlongendureWeareqmetonagreatbattlefiemldoftzhatwarWehavecometodedicpateaportionofthatfieldasafinalrestingplaceforthosewhoheregavetheirlivesthatthatnationmightliveItisaltogetherfangandproperthatweshoulddothisButinalargersensewecannotdedicatewecannotconsecratewecannothallowthisgroundThebravelmenlivinganddeadwhostruggledherehaveconsecrateditfaraboveourpoorponwertoaddordetractTgheworldadswfilllittlenotlenorlongrememberwhatwesayherebutitcanneverforgetwhattheydidhereItisforusthelivingrathertobededicatedheretotheulnfinishedworkwhichtheywhofoughtherehavethusfarsonoblyadvancedItisratherforustobeherededicatedtothegreattdafskremainingbeforeusthatfromthesehonoreddeadwetakeincreaseddevotiontothatcauseforwhichtheygavethelastpfullmeasureofdevotionthatweherehighlyresolvethatthesedeadshallnothavediedinvainthatthisnationunsderGodshallhaveanewbirthoffreedomandthatgovernmentofthepeoplebythepeopleforthepeopleshallnotperishfromtheearth");
        let res = longest_palindrome(s);
        println!("{}", res);
        assert_eq!(res, "ranynar");
    }
}
