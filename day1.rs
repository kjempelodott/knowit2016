fn main() {
    println!("{}6", (0u32..).skip_while(|x|
              6*10u32.pow(1 + ((*x as f32).log10().floor()) as u32) + x !=
              4*(6 + x*10)).next().unwrap());
}
