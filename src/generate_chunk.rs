use noise::{NoiseFn, Perlin};

pub fn generate(x: i32, z: i32) -> Vec<Vec<i32>> {
    let perlin = Perlin::new(0);
    
    let mut final_vec = Vec::with_capacity(16);
    
    for sub_x in 0..16 {
        let mut local_vec = Vec::with_capacity(16);
        
        for sub_z in 0..16 {
            let y = (perlin.get([(x * 16 + sub_x) as f64 / 16.0, (z * 16 + sub_z) as f64 / 16.0]) * 8.0) as i32;
            
            local_vec.push(y);
        }
        
        final_vec.push(local_vec);
    }
    
    final_vec
}