fn main() {
    let mut highest_value: i32 = 0;

    // finding largest palindrome. nested for-loops work, ish.. but man, is there nothing better?
    for a in 100..1000 {
        for b in 100..1000 {
            let test: i32 = a * b;

            if is_palindrome(test) && test > highest_value {
                highest_value = test;
            }
        }
    }

    println!("highest palindrome is {highest_value}");
}


fn is_palindrome(n: i32) -> bool {
    let s1: String = n.to_string();
    let s2: String = s1.chars().rev().collect();
    let res: bool = s1 == s2;
    return res;
}
