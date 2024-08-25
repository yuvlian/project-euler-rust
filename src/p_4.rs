/*
<p>A palindromic number reads the same both ways. The largest palindrome made from the product of two $2$-digit numbers is $9009 = 91 \times 99$.</p>
<p>Find the largest palindrome made from the product of two $3$-digit numbers.</p>
*/

pub fn solution() {
    fn is_palindrome(num: u32) -> bool {
        let s = num.to_string();
        s == s.chars().rev().collect::<String>()
    }

    let mut largest_palindrome = 0;

    for i in (100..1000).rev() {
        if i * 999 <= largest_palindrome {
            break;
        }
        for j in (i..1000).rev() {
            let product = i * j;
            if product <= largest_palindrome {
                break; 
            }
            if is_palindrome(product) {
                largest_palindrome = product;
            }
        }
    }

    assert_eq!(largest_palindrome, 906609);

    println!("[P4] The largest palindrome made from the product of two 3-digit numbers is {}", largest_palindrome);
}
