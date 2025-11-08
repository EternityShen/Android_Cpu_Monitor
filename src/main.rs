use cpu_monitor::Load;
fn main() {
    let mut load = Load::new();
    loop {
        println!("{}", load.update());
    }
}
