use std::time;

pub mod point;
mod grid;
pub use grid::Grid as Grid;

pub fn load_input(year: usize, day: usize) -> String {
    let path = format!("input/{}-{}.txt", year, day);
    let res = std::fs::read_to_string(&path);
    match res {
        Ok(input) => input,
        Err(e) => {
            eprintln!("{} : {}", e, path);
            std::process::exit(2);
        }
    }
}

pub fn timer(f: fn()) {
    let t0 = time::Instant::now();
    let ret = f();
    println!("time used {:?}", time::Instant::now().duration_since(t0));

    ret
}
