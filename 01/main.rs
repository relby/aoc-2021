fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    let a = input
        .split("\r\n")
        .filter(|x| !x.is_empty())
        .map(|x| str::parse::<i32>(x).unwrap())
        .collect::<Vec<_>>();

    let n = a.len();

    // Part 1 
    let mut out = 0;
    for i in 1..n {
        out += (a[i] > a[i - 1]) as usize;
    }
    println!("{:?}", out);

    // Part 2
    let mut out = 0;
    for i in 3..n {
        let x = a[i] + a[i - 1] + a[i - 2];
        let y = a[i - 1] + a[i - 2] + a[i - 3];
        out += (x > y) as usize;
    }
    println!("{}", out);
}
