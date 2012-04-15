use euler;

import euler::prime;

fn main() {
    let mut num = 600851475143u64;
    for prime::prime().each {|p|
        while num % p == 0u64 {
            num /= p;
        }
        if num == 1u {
            io::println(#fmt("%u", p));
            break;
        }
    };
}
