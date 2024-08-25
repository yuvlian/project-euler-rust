/*
<p>The sum of the primes below $10$ is $2 + 3 + 5 + 7 = 17$.</p>
<p>Find the sum of all the primes below two million.</p>
*/

pub fn solution() {
    let limit: u64 = 2_000_000;
    let mut is_prime = vec![true; limit as usize];
    is_prime[0] = false;
    is_prime[1] = false;

    let mut sum: u64 = 0;

    // Using unsafe because otherwise it takes too long.
    unsafe {
        let is_prime_ptr = is_prime.as_mut_ptr();

        for num in 2..limit {
            if *is_prime_ptr.add(num as usize) {
                sum += num;
                let mut multiple = num * num;
                while multiple < limit {
                    *is_prime_ptr.add(multiple as usize) = false;
                    multiple += num;
                }
            }
        }
    }

    assert_eq!(sum, 142913828922 as u64);

    println!("[P10] The sum of all primes below {} is {}", limit, sum);
}


