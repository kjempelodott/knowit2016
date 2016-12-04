const N: usize = 1337;

fn main() {
    let mut n: Vec<usize> = (1..N+1)
        .map(|i| if i % 7 != 0 && !i.to_string().contains('7') { return i } else { 0 } )
        .collect();
    println!("{}", (0..N)
             .scan(0, |j, i| {
                 if n[i] == 0 {
                     n[i] = n[*j];
                     *j += 1;
                 }
                 Some(n[i])
             }).last().unwrap());
}
