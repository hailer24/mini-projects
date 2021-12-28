fn main() {
    let mut ret: i32 = 0;
    for i in 0..100 {
        
        if i%2 == 0 {
            ret += (i)*(i);
        }
    }
    println!("{}", (ret as f64).powf(1f64/5f64));
}
