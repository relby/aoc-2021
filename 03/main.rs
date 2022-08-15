
fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    let a = input
        .lines()
        .filter(|x| !x.is_empty())
        .collect::<Vec<_>>();
    
    let rows = a.len();
    let cols = a[0].len();
    // Part 1
    let mut g = 0;
    let mut e = 0;

    for x in 0..cols {
        let mut c0 = 0;
        let mut c1 = 0;
        for y in 0..rows {
            let s = a[y].chars().nth(x).unwrap();
            match s {
                '0' => { c0 += 1 }
                '1' => { c1 += 1 }
                _ => unreachable!()
            }
        }
        g += ((c0 < c1) as i32) << (cols - x - 1);
        e += ((c0 > c1) as i32) << (cols - x - 1);
    }
    println!("{}", g * e);

    // Part 2
    let mut oxy_rem = a.to_vec();
    let mut co2_rem = a.to_vec();

    let mut c0;
    let mut c1;

    for x in 0..cols {
        // count oxy
        c0 = 0; c1 = 0;
        if oxy_rem.len() > 1 {
            for y in 0..oxy_rem.len() {
                let s = oxy_rem[y].chars().nth(x).unwrap();
                match s {
                    '0' => { c0 += 1 }
                    '1' => { c1 += 1 }
                    _ => unreachable!()
                }
            }
            let oxy_cond = if c1 >= c0 { '1' } else { '0' };
            oxy_rem = oxy_rem.into_iter().filter(|e| e.chars().nth(x).unwrap() == oxy_cond).collect();
        }
        // count co2
        c0 = 0; c1 = 0;
        if co2_rem.len() > 1 {
            for y in 0..co2_rem.len() {
                let s = co2_rem[y].chars().nth(x).unwrap();
                match s {
                    '0' => { c0 += 1 }
                    '1' => { c1 += 1 }
                    _ => unreachable!()
                }
            }
            let co2_cond = if c0 <= c1 { '0' } else { '1' };
            co2_rem = co2_rem.into_iter().filter(|e| e.chars().nth(x).unwrap() == co2_cond).collect();
        }
    }
    let oxy_rem_num = i32::from_str_radix(oxy_rem[0], 2).unwrap();
    let co2_rem_num = i32::from_str_radix(co2_rem[0], 2).unwrap();
    println!("{:?}", oxy_rem_num * co2_rem_num);
}