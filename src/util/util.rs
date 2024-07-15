use colored::Colorize;

#[inline]
#[cold]
fn cold() {}

#[inline]
pub fn likely(b: bool) -> bool {
    if !b {
        cold()
    }
    b
}

#[inline]
pub fn unlikely(b: bool) -> bool {
    if b {
        cold()
    }
    b
}

pub fn print_debug(
    parse_duration: std::time::Duration,
    opt_duration: std::time::Duration,
    exec_duration: std::time::Duration,
) {
    println!();
    println!(
        "{}",
        format!("[ DEBUG ] Parsing      took: {:?}", parse_duration)
            .yellow()
            .bold()
    );
    println!(
        "{}",
        format!("[ DEBUG ] Optimization took: {:?}", opt_duration)
            .green()
            .bold()
    );
    println!(
        "{}",
        format!("[ DEBUG ] Execution    took: {:?}", exec_duration)
            .red()
            .bold()
    );
    println!();
}
