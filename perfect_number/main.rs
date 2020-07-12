fn divisors(n: usize) -> usize {
    let mut sum: usize = 0;
    for i in 1..=(f64::sqrt(n as f64) + 1e-9) as usize {
        if n % i == 0 {
            sum += i;
            if i != n / i {
                sum += n / i;
            }
        }
    }
    return sum;
}

fn main() {
    for i in 1.. 10000 {
        let x = divisors(i);
        if (x - i) == i {
            println!("It's a perfect number: {}", i);
        }
    }
}
