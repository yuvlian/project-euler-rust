/*
<p>By listing the first six prime numbers: $2, 3, 5, 7, 11$, and $13$, we can see that the $6$th prime is $13$.</p>
<p>What is the $10\,001$st prime number?</p>
*/

pub fn solution() {
    fn is_prime(num: u64) -> bool {
        if num < 2 {
            return false;
        }
        if num == 2 {
            return true;
        }
        if num % 2 == 0 {
            return false;
        }
        let limit = (num as f64).sqrt() as u64;
        for i in (3..=limit).step_by(2) {
            if num % i == 0 {
                return false;
            }
        }
        true
    }

    let mut count = 0;
    let mut num = 1;
    let target = 10_001;

    while count < target {
        num += 1;
        if is_prime(num) {
            count += 1;
        }
    }

    assert_eq!(num, 104743 as u64);

    println!("[P7] The {}st prime number is {}", target, num);
}

