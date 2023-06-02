fn main() {
    let mut primes: Vec<i32> = vec![2, 3];

    // check 6n-1 and 6n+1, start with an offset of -1,
    // so basically stat at 5, step by 6, (5, 11, 17)
    // and check for n and n+2 (5 and 7, 11 and 13, 17 and 19)
    let mut a: i32 = 5;
    while primes.len() < 10001 {
        if is_prime(a) {
            primes.push(a);
        }
        if is_prime(a + 2) {
            primes.push(a + 2);
        }
        a += 6;
    }

    println!("{}", primes[10000]);
}

fn is_prime(n: i32) -> bool {
    // basic conditions
    if n < 2 {
        return false;
    } else if n == 2 || n == 3 {
        return true;
    } else if n % 2 == 0 {
        return false;
    }

    let mut test: i32 = 3;
    // assumption: if not divisible by anything below sqrt()
    // then not divisible at all, hence prime
    while test < ((n as f32).sqrt() + 1.0) as i32 {
        if n % test == 0 {
            return false;
        }
        test += 2;
    }

    // passed all the tests, must be prime
    return true;
}
