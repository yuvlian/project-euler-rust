/*
<p>The prime factors of $13195$ are $5, 7, 13$ and $29$.</p>
<p>What is the largest prime factor of the number $600851475143$?</p>
*/

pub fn solution() {
    let mut n: u64 = 600_851_475_143;
    let mut factor: u64 = 2;
    let mut largest_prime: u64 = 0;
    
    while factor * factor <= n {
        if n % factor == 0 {
            n /= factor;
            largest_prime = factor;
        } else {
            factor += 1;
        }
    }
    
    if n > 1 {
        largest_prime = n;
    }
    
    assert_eq!(largest_prime, 6857 as u64);

    println!("[P3] The largest prime factor of {} is {}", 600_851_475_143 as u64, largest_prime);
}

