mod config {
    #[cfg(windows)]
    pub const LINE_ENDING: &'static str = "\r\n";
    #[cfg(not(windows))]
    pub const LINE_ENDING: &'static str = "\n";
}

mod day1;
mod day2;

#[macro_export]
macro_rules! run {
    ( $x:ident ) => {
        {
            use $x::{solution1, solution2};
            println!("Day {}", &stringify!($x)[3..]);
            print!("\tSolution 1: ");
            println!("{}", solution1(format!("src/{}/input.txt", stringify!($x)).as_str()).to_string());
            print!("\tSolution 2: ");
            println!("{}", solution2(format!("src/{}/input.txt", stringify!($x)).as_str()).to_string());
        }
    };
}

fn main() {
    run!(day1);
    run!(day2);
}
