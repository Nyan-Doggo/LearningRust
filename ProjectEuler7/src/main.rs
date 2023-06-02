fn main() {
    let mut primes: Vec<i32> = vec![2, 3];

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
    if n < 2 {
        return false;
    } else if n == 2 || n == 3 {
        return true;
    } else if n % 2 == 0 {
        return false;
    }

    let mut test: i32 = 3;

    while test < ((n as f32).sqrt() + 1.0) as i32 {
        if n % test == 0 {
            return false;
        }
        test += 2;
    }

    return true;
}
