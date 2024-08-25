/*
<p>The sum of the squares of the first ten natural numbers is,</p>
$$1^2 + 2^2 + ... + 10^2 = 385.$$
<p>The square of the sum of the first ten natural numbers is,</p>
$$(1 + 2 + ... + 10)^2 = 55^2 = 3025.$$
<p>Hence the difference between the sum of the squares of the first ten natural numbers and the square of the sum is $3025 - 385 = 2640$.</p>
<p>Find the difference between the sum of the squares of the first one hundred natural numbers and the square of the sum.</p>
*/

pub fn solution() {
    let n = 100;

    let sum_of_squares = (n * (n + 1) * (2 * n + 1)) / 6;

    let sum = (n * (n + 1)) / 2;

    let square_of_sum = sum * sum;

    let diff = square_of_sum - sum_of_squares;

    assert_eq!(diff, 25164150 as i32);

    println!("[P6] The difference between the sum of the squares of the first {} natural numbers and the square of the sum is {}", n, diff);
}