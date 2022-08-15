fn main() {
    let input = std::fs::read_to_string("imput.txt").unwrap();
    let a = input
        .lines()
        .filter(|x| !x.is_empty())
        .map(|x| {
            let mut splitted = x.split_whitespace();
            let word = splitted.next().unwrap();
            let number = str::parse::<i32>(splitted.next().unwrap()).unwrap();
            (word, number)
        })
        .collect::<Vec<_>>();

    // Part 1   
    let mut y = 0;
    let mut x = 0;
    for e in a {
        match e {
            ("forward", n) => { x += n }
            ("back", n)    => { x -= n }
            ("up", n)      => { y -= n }
            ("down", n)    => { y += n }
            _ => unreachable!()
        }
    }
    println!("{}", x * y);

    // Part 2
    let mut y = 0;
    let mut x = 0;
    let mut aim = 0;
    for e in a {
        match e {
            ("forward", n) => { x += n; y += aim * n }
            ("back", n)    => { x -= n; y += aim * n }
            ("up", n)      => { aim -= n }
            ("down", n)    => { aim += n }
            _ => unreachable!()
        }
    }
    println!("{}", x * y);
}