/*
<p>By listing the first six prime numbers: $2, 3, 5, 7, 11$, and $13$, we can see that the $6$th prime is $13$.</p>
<p>What is the $10\,001$st prime number?</p>
*/

pub fn solution() {
    fn is_prime(num: u64, primes: &[u64]) -> bool {
        if num < 2 {
            return false;
        }
        let limit = (num as f64).sqrt() as u64;
            for &p in primes.iter() {
                if p > limit {
                    break;
                }
                if num % p == 0 {
                    return false;
                }
            }
        true
    }

    let mut primes: Vec<u64> = vec![2];
    let mut count = 1;
    let mut num = 3;
    let target = 10_001;

    while count < target {
        if is_prime(num, &primes) {
                primes.push(num);
            count += 1;
        }
        num += 2;
    }

    assert_eq!(primes[target - 1], 104743 as u64);

    println!("[P7] The {}st prime number is {}", target, primes[target - 1]);
}

