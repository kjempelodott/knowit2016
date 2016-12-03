use std::collections::HashMap;

fn main() {
    let mut p: HashMap<&str, HashMap<&str, i32>> = HashMap::new();
    let (f, h): (Vec<(&str, &str, &str)>, Vec<(&str, &str, &str)>) =
        include_str!("input3").split("\n")
        .map(|l| {
            let mut a = l.split_whitespace();
            (a.next().unwrap(), a.next().unwrap(), a.next().unwrap())
        }).partition(|l| l.0 == "friends");
    for l in f {
        p.entry(l.1).or_insert(HashMap::new()).insert(l.2, 0);
        p.entry(l.2).or_insert(HashMap::new()).insert(l.1, 0);
    }
    for l in h {
        if p.entry(l.0).or_insert(HashMap::new()).contains_key(l.2) {
            p.get_mut(l.0).unwrap().insert(l.2, 1);
        }
        if p.entry(l.2).or_insert(HashMap::new()).get(l.0) == Some(&1) {
            p.get_mut(l.2).unwrap().insert(l.0, 0);
            p.get_mut(l.0).unwrap().insert(l.2, 0);
        }
    }
    println!("{}", p.iter().max_by_key(|x| x.1.values().fold(0, |z,y| z+y)).unwrap().0);
}
