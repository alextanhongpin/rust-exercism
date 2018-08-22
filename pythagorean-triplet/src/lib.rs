pub fn find() -> Option<u32> {
    // let mut a_i = 0;
    // let mut b_i = 0;
    // let range = 1000;
    // for a in 1..range {
    //    for b in 1..range {
    //         if (a as f32) + (b as f32) + ((a as f32).powf(2.0) + (b as f32).powf(2.0)).sqrt() == 1000.0 {
    //             a_i = a;
    //             b_i = b;
    //         }
    //    }
    // }
    // let c_i = 1000 - a_i - b_i;
    // Some(a_i * b_i * c_i)
    
   let result: Vec<u32> = (1..=1000)
        .flat_map(|a: u32| {
            (1..=1000)
            .map(move|b: u32| {
                let c = if a + b < 1000 { 1000 - a - b } else { 0 };
                (a, b, c)
            })
        })
        .filter(|(a, b, c)| a * a + b * b == c * c)
        .map(|(a, b, c)| a * b * c)
        .collect();
        
    result.first().cloned()
}

