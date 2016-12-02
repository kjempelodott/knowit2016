const MAX: u64 = 4000000000;
fn main() {
    let mut a: u64 = 1;
    let mut b: u64 = 1;
    let mut sum: u64 = 0;
    loop {
        if b % 2 == 0 { sum += b }
        let n = a + b;
        if n >= MAX { break; }       
        a = b;
        b = n;
    }
    println!("{}", sum);
}
