fn main() {
    let mut square_of_sums: i32 = 0;
    let mut sum_of_squares: i32 = 0;

    for i in 1..101 {
        sum_of_squares += i * i;
        square_of_sums += i;
    }

    square_of_sums *= square_of_sums;

    // .abs() gets the absolute, (possitive) value, so we dont need
    // to wory about ordering and accidentally getting a negative difference
    let difference: i32 = (sum_of_squares - square_of_sums).abs();

    println!("{difference}");
}
