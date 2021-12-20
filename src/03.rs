

fn main() {
    let input: Vec<u8> = std::fs::read_to_string("input/03.txt").unwrap()
        .lines()
        .map(|str| u32::from_str_radix(str,2).unwrap())
        .collect();


    let oxy = (1..12).rev()



    

}


fn oxy(input: Vec<u32>, p: u32) -> u32 {

    for i in (1..=p).rev() {
        let input = input.into_iter().filter(|&x| x & (1 << i)  )
    }
}