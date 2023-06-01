fn main() {
    
    // "let mut" means we're declaring a variable that will change
    let mut sum: i32 = 0;

    // the x..y notation for iterators is weird, but i kinda like it ig
    for n in 1..1000 {
        
        // passing &n instead of n should mean we pass a reffernce and not the actual value?
        // im not entirely sure how this works
        if is_fizzbuzz(&n) {
            sum += n;
        }
    }

    // println! is a macro, the "!"" is the clue here. it expands to more complex code when compiled
    println!("total sum is {sum}");
}


fn is_fizzbuzz(i: &i32) -> bool {
    // no need for () in the if-clause? wut? i mean neat, kinda pythonic ig?
    if i % 3 == 0 || i % 5 == 0 {
        return true;
    }
    return false;
}
