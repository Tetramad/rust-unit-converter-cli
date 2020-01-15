mod argparser;
mod unit;
mod length;
mod area;
mod volume;

fn main() {
    let args = argparser::arguments();
    let (from, to, value) = unit::identify(&args);

    if unit::is_convertable(&from, &to) {
        println!("{}", unit::convert(&from, &to, value));
    } else {
        panic!();
    }
}
