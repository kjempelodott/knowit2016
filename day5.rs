fn main() {
    let v: Vec<u8> = include_str!("input5").trim()
        .replace("[","").replace("]","").replace(" ", "").split(",")
        .map(|x| match x {
            "I"    => 1,
            "II"   => 2,
            "III"  => 3,
            "IV"   => 4,
            "V"    => 5,
            "VI"   => 6,
            "VII"  => 7,
            "VIII" => 8,
            "IX"   => 9,
            "X"    => 10,
            "XI"   => 11,
            "XII"  => 12,
            "XIII" => 13,
            _      => 0
        }).collect();
    println!("{}", (0..v.len()/2)
             .map(|i| (v[i] + 96 + v[v.len()-i-1]) as char)
             .collect::<String>());
}
