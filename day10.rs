const KRIGER: i16 = 1;
const TROLLMANN: i16 = 2;
const TYV: i16 = 4;
const PREST: i16 = 8;
const ALLE: i16 = 15;
const DAU_KRIGER: i16 = 16;
const DAU_TROLLMANN: i16 = 32;

fn alive(adv: i16) -> i16 {
    1 + (adv & KRIGER) + (adv & TROLLMANN)/TROLLMANN + (adv & PREST)/PREST
}

fn main() {
    let mut adv = ALLE;
    let mut escaped_goblins = 0;

    for room in 0..100i16 {
        let mut goblins = room + 1;
        let mut can_resurrect = adv & PREST == PREST;
        while goblins > 0 {
            goblins -= (adv & KRIGER) + 10 * (adv & TROLLMANN)/TROLLMANN + (adv & TYV)/TYV;
            if can_resurrect {
                if (adv & (KRIGER + DAU_KRIGER)) == 0 {
                    adv |= KRIGER;
                    can_resurrect = false;
                }
                else if (adv & (TROLLMANN + DAU_TROLLMANN)) == 0 {
                    adv |= TROLLMANN;
                    can_resurrect = false;
                }
            }
            if (adv & ALLE) == TYV {
                escaped_goblins += goblins;
                break;
            }
            if goblins >= 10 * alive(adv) {
                if (adv & KRIGER) == KRIGER {
                    adv ^= KRIGER;
                }
                else if (adv & TROLLMANN) == TROLLMANN {
                    adv ^= TROLLMANN;
                }
                else {
                    adv ^= PREST;
                }
            }
        }
        if (adv & (KRIGER + DAU_KRIGER)) == 0 {
            adv |= DAU_KRIGER;
        }
        if (adv & (TROLLMANN + DAU_TROLLMANN)) == 0 {
            adv |= DAU_TROLLMANN;
        }
    }
    println!("{}", alive(adv) + escaped_goblins + 17);
}
