fn main() {
    let mut sum: i32 = 0;
    
    // fib[1] will always be the current number in the sequence
    let mut fib: Vec<i32> = vec![1, 1, 0];

    while fib[1] < 4000000 {
        if is_even(&fib[1]) {
            sum += fib[1]
        };
        fib = fib_step(&fib);
    }

    print!("the sum is {sum}");
}

// steps the fib vector up uwu, has to be a better way of passing and working with variables?
fn fib_step(v: &Vec<i32>) -> Vec<i32> {
    let mut i: Vec<i32> = v.to_vec();
    i[2] = i[0] + i[1];
    i[0] = i[1];
    i[1] = i[2];
    return i;
}

// simple even check
fn is_even(i: &i32) -> bool {
    if i % 2 == 0 {
        return true;
    }

    return false;
}
