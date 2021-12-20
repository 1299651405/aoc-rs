
fn myparse(s: &str) -> (&str, u32) {
    let mut a = s.split(' ');
    (a.next().unwrap(), a.next().unwrap().parse().unwrap())
}

fn main() {
    let input = std::fs::read_to_string("input/02.txt").unwrap();
    
    let mut a = 0;
    let mut b = 0;
    let mut d = 0;

    for i in input.lines() {
        let (c,n) = myparse(i);
        match c {
            "forward" => {a += n; d += b*n},
            "up"      => b -= n,
            "down"    => b += n,
            _ => ()
        }
    }

    
    println!("{} {}",a*b, a*d)
}
