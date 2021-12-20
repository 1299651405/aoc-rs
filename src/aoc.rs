use rand::Rng;


fn gen_snow(n: usize) -> String {
    
    let mut rng = rand::thread_rng();

    (0..n).map(|_| {    
        match rng.gen::<u8>() {
            0..=32 => "* ",
            33..=64 => "\' ",
            65..=96 => "` ",
            97..=128 => ". ",
            129..=160 => ", ",
            161..=192 => "^ ",
            _ => "` ",
        }
    }).collect()
}



#[macro_export]
macro_rules! snow_print {
    ($a:expr,$b:expr,$c:expr) => {
        let snow = gen_snow(100);

        let reset = "\x1B[0m";
        let bold = "\x1B[1m";
        let blink = "\x1B[5m";
        
        println!("{}{}{}",reset,blink,&snow[0..30]);
        println!("{}{}{} Day {:02} {}{}{}",&snow[30..40],reset,bold,a,reset,blink,&snow[50..60]);
        println!("{}{}",&snow[60..90],reset);
        println!("  Part 1: {}",b);
        println!("  Part 2: {}",c);
        
    };
} 
