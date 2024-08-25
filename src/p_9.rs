/*
<p>A Pythagorean triplet is a set of three natural numbers, $a \lt b \lt c$, for which,
$$a^2 + b^2 = c^2.$$</p>
<p>For example, $3^2 + 4^2 = 9 + 16 = 25 = 5^2$.</p>
<p>There exists exactly one Pythagorean triplet for which $a + b + c = 1000$.<br>Find the product $abc$.</p>
*/

pub fn solution() {
    let sum = 1000;
    let mut product = 0;
    let mut a = 0;
    let mut b = 0;
    let mut c = 0;
    
    for i in 1..sum / 3 {
        for j in (i + 1)..(sum - i) / 2 {
            let k = sum - i - j;
            if i * i + j * j == k * k {
                product = i * j * k;
                a = i;
                b = j;
                c = k;
                break;
            }
        }
        if product != 0 {
            break;
        }
    }

    assert_eq!(product, 31875000 as i32);

    println!("[P9] The Pythagorean triple for which a+b+c = 1000 is ({}, {}, {}). The product abc is {}", a, b, c, product);
}


