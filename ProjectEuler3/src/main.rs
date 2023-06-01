fn main() {
    // i64 beacuse the num is too large
    let mut target: i64 = 600851475143;
    
    let mut test: i64 = 2;
    let mut factors: Vec<i64> = Vec::new();

    // work from the bottom up, 
    // ASIOM: lowest divisor has to be a prime
    while test < target{
        if target % test == 0{
            factors.push(test);
            target = target / test;
        }
        else {
            // could implement a faster way of generating possible vallues
            // by following the 6n+-1 pattern, but oh well
            test += 1;
        }
    }
    
    // add any remaining value if above 1
    if target > 1 {
        factors.push(target);
    }
    
    println!("factors are {:?}", factors);
}


