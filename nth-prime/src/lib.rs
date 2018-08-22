pub fn nth(n: u32) -> Option<u32> {
    match n {
        0 => None,
        _ => {
            let mut i = 1; // Number to test if it's prime; will be incremented
            let mut c = 0; // Hold the prime count
            'outer: loop {
                i += 1;
                let max = (i as f32).sqrt() as u32;
                for m in 2..=max {
                    // Is not prime number, continue the loop
                    if i % m == 0 {
                        continue 'outer;
                    }
                }

                // Increment the count by 1 for every prime number found
                c += 1;

                // We have found the desired count of prime number, exit!
                if c == n {
                    break;
                }
            }
            Some(i)
        }
    }
}
