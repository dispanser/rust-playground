pub fn primes(n: i32) -> Vec<i32> {
    (2 ..= n)
        .filter(|i| is_prime2(i))
        .collect::<Vec<i32>>()
}

pub fn primes2(n: i32) -> impl Iterator<Item = i32> {
    (2..=n)
        .filter(|i| is_prime2(i))
}

fn is_prime(n: i32) -> bool {
    for i in 2 .. n {
        if n % i == 0 {
            return false;
        }
    }
    true
}

fn is_prime2(n: &i32) -> bool {
    let max: i32 = (*n as f32).sqrt() as i32;
    !(2 ..= max).any(|i| n % i == 0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_prime() {
        assert_eq!(is_prime(2), true);
        assert_eq!(is_prime(3), true);
        assert_eq!(is_prime(4), false);
    }

    #[test]
    fn test_is_prime2() {
        assert_eq!(is_prime2(&2), true);
        assert_eq!(is_prime2(&3), true);
        assert_eq!(is_prime2(&4), false);
    }

}