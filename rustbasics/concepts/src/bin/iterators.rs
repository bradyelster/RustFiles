use std::f32::consts::PI;

fn vector_mag_1(v1: &[u32]) -> f32 {
    v1.iter()
    .map(|&x| x as f32 * x as f32)
    .sum::<f32>()
    .sqrt()
}

fn vector_mag_2(v1: &[u32]) -> f32 {
    let mut sum: f32 = 0.0;
    let mut v1_f32 = [0.0; 3]; 

    for i in 0..v1.len() {
        v1_f32[i] = (v1[i] as f32) * (v1[i] as f32);
        sum += v1_f32[i];
    }
    return sum.sqrt()
}

fn vector_mag_3(v1: &[u32]) -> f32 {
    let mut sum: f32 = 0.0;
    for &entry in v1 {
        sum += (entry as f32) * (entry as f32);
    }
    sum.sqrt()
}

fn get_angle_rad(v1: &[u32; 2]) -> f32 {
    return (v1[0] as f32 / v1[1] as f32).atan()
}

fn get_angle_deg(v1: &[u32; 2]) -> f32 {
    return (v1[0] as f32 / v1[1] as f32).atan()*(180f32/PI)
}

fn dot_prod(v1: &[u32; 2], v2: &[u32; 2]) -> f32 {
    let angle1: f32 = get_angle_rad(v1);
    let angle2: f32 = get_angle_rad(v2);
    return vector_mag_1(v1)*vector_mag_1(v2)*(angle2-angle1).cos()
}

fn main() {
    let arr1: [u32; 3] = [3, 3, 3];
    println!("(Rustic) vector magnitude is: {:?}", vector_mag_1(&arr1));
    println!("(Pythonic) vector magnitude is: {:?}", vector_mag_3(&arr1));
    println!("(Bad Habit) vector magnitude is: {:?}", vector_mag_2(&arr1));
    println!("\n");

    let arr2: [u32; 2] = [1, 2];
    let arr3: [u32; 2] = [3, 8];
    println!("Angle from x-axis for v1 (radians): {}", get_angle_rad(&arr3));
    println!("Angle from x-axis for v1 (degrees): {}", get_angle_deg(&arr3));
    println!("Dot prod: {}", dot_prod(&arr2, &arr3));
}