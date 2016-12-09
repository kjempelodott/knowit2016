use std::collections::HashMap;

fn main() {
    let mut x: HashMap<&str, i32> = HashMap::new();
    for l in include_str!("input9").trim().lines() {
        let tr: Vec<&str> = l.split(',').collect();
        let z: i32 = tr[2].parse().unwrap();
        *x.entry(tr[0]).or_insert(0) -= z;
        *x.entry(tr[1]).or_insert(0) += z;
    }
    println!("{}", x.iter().filter(|z| *z.1 > 10).count());
}
