pub fn nth(n: u32) -> u32 {

    let mut prime: u32 = 0;
    let mut result: u32 = 0;
    
    for x in 3.. { 
        if is_prime(x) {
            prime += 1;
        }
        if prime == n {
            result = x;
            break;
        }
    }

    result
}

fn is_prime(n: u32) -> bool {
    match n {
        0 | 1 => false,
        2 => true,
        _even if n % 2 == 0 => false,
        _ => {
            let sqrt_limit = (n as f32).sqrt() as u32;
            (3..=sqrt_limit).step_by(2).find(|i| n % i == 0).is_none()
        }
    }
}
