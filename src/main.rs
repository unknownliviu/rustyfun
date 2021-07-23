fn main() {
    let n = 1000;
    let mut accumulator = 0;
    for i in 3..n {
        if (i % 3 == 0) || (i % 5 == 0) {
            accumulator = accumulator + i;
        }
    }

    println!("{}", accumulator);
}