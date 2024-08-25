/*
<p>If we list all the natural numbers below $10$ that are multiples of $3$ or $5$, we get $3, 5, 6$ and $9$. The sum of these multiples is $23$.</p>
<p>Find the sum of all the multiples of $3$ or $5$ below $1000$.</p>
*/

pub fn solution() {
    let limit = 1000;
    let mut sum = 0;

    for i in 1..limit {
        if i % 3 == 0 || i % 5 == 0 {
            sum += i;
        }
    }

    assert_eq!(sum, 233168 as i32);

    println!("[P1] The sum of all multiples of 3 or 5 below {} is {}", limit, sum);
}