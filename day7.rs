fn main() {
    let (mut x, mut y) = (0, 0);
    for l in include_str!("input7").trim().lines() {
        let mut z = l.split_whitespace();
        let (i, d) = (z.nth(1).unwrap().parse().unwrap(), z.nth(1).unwrap());
        match d {
            "south" => y -= i,
            "north" => y += i,
            "west" => x += i,
            "east" => x -= i,
            _ => ()
        }
    }
    println!("{},{}", y, x);
}
