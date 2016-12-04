use std::collections::VecDeque;
const N: usize = 1337;

fn main() {
    let mut n: VecDeque<usize> = VecDeque::new();
    for i in 1..N+1 {
        if i % 7 != 0 && !i.to_string().contains('7') {
            n.push_back(i);
        }
        else {
            let m = n.pop_front().unwrap();
            n.push_back(m);
        }
    }
    println!("{}", n.back().unwrap());
}
