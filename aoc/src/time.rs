pub fn time<F, T>(task: &str, f: F) -> T
where
    F: FnOnce() -> T,
{
    let start = std::time::Instant::now();
    let result = f();
    println!("{}: {:?}", task, start.elapsed());
    result
}
