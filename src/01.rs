

fn get_count(values: &[u32], delta: usize) -> usize {
    values
        .iter()
        .enumerate()
        .filter(|(i,v)| *v < values.get(i + delta).unwrap_or(v))
        .count()
}

fn main() {

    
    let input: Vec<u32> = std::fs::read_to_string("input/01.txt")
        .unwrap()
        .lines()
        .map(|s| s.parse().unwrap())
        .collect();

    println!("{} {} {}",1, get_count(&input,1), get_count(&input,3))
}