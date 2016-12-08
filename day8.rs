use std::collections::HashMap;

fn main() {
    let ladders: HashMap<u32, u32> =
        vec![(3,17), (8,10), (15,44), (22,5), (39,56), (49,75), (62,45),
             (64,19), (65,73), (80,12), (87,79)].into_iter().collect();
    
    let mut players = vec![1;1337];
    let mut n = 0;
    for (i, dice) in include_str!("input8").trim().lines().enumerate() {
        let j = i % 1337;
        let pos = players[j] + dice.parse::<u32>().unwrap();
        if pos == 90 {
            println!("Player {} wins. Result: {}", j + 1, n*(j + 1));
            break
        }
        if pos < 90 {
            match ladders.get(&pos) {
                Some(&b) => { n += 1; players[j] = b; },
                None => { players[j] = pos; },
            }
        }
    }
}
