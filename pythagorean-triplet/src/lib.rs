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
        .flat_map(move|a: u32| {
            (1..=1000)
            .map(move|b: u32| {
                // Downcasting c to u32 will result in multiple incorrect solutions
                let c = ((a.pow(2) + b.pow(2)) as f32).sqrt(); 
                (a as f32, b as f32, c)
            })
        })
        .filter(|(a, b, c)| a + b + c == 1000.0)
        .map(|(a, b, c)| (a * b * c) as u32)
        .collect();
        
    result.first().cloned()
}

