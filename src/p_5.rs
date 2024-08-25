/*
<p>$2520$ is the smallest number that can be divided by each of the numbers from $1$ to $10$ without any remainder.</p>
<p>What is the smallest positive number that is <strong class="tooltip">evenly divisible<span class="tooltiptext">divisible with no remainder</span></strong> by all of the numbers from $1$ to $20$?</p>
*/

pub fn solution() {
    // Greatest common divisor using Euclid's algorithm
    fn gcd(a: u64, b: u64) -> u64 {
        let mut a = a;
        let mut b = b;
        while b != 0 {
            let temp = b;
            b = a % b;
            a = temp;
        }
        a
    }

    // Least common multiple
    fn lcm(a: u64, b: u64) -> u64 {
        a * b / gcd(a, b)
    }

    let mut result = 1;

    for i in 1..=20 {
        result = lcm(result, i);
    }

    assert_eq!(result, 232792560 as u64);

    println!("[P5] The smallest positive number that is evenly divisible by all of the numbers from 1 to 20 is {}", result);
}