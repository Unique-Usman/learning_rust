fn factorial(num: u64) -> u64 {
    (1..num + 1).fold(1, |acc, x| acc * x)
}
