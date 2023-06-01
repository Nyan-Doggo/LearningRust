fn main() {
    let mut factor_collection: Vec<i32> = Vec::new();

    for i in 2..21 {
        let integer_factors: Vec<i32> = get_factor_vec(i);

        for factor in &integer_factors {
            // ensure that the minimum count of each factor is in the list
            while &factor_collection.iter().filter(|&x| x == factor).count()
                < &integer_factors.iter().filter(|&x| x == factor).count()
            {
                factor_collection.push(*factor);
            }
        }
    }

    // then just multiply the whole thing together and bam, works like a charm
    let mut sum = 1;
    for i in factor_collection {
        sum *= i;
    }

    println! {"{sum}"};
}

fn get_factor_vec(i: i32) -> Vec<i32> {
    let mut factors: Vec<i32> = Vec::new();
    let mut target: i32 = i;
    let mut test: i32 = 2;

    // cycle every test: i32 up to the relevant target
    while test < target {
        // if evenly divisble test is a prime factor and we send it to the array, readjust our target and move on
        if target % test == 0 {
            factors.push(test);
            target = target / test;
        } else {
            test += 1;
        }
    }

    // if the remainder is above 1, it is also a factor
    if target > 1 {
        factors.push(target);
    }

    // return our factors ^~^
    return factors;
}
