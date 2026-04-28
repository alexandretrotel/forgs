pub fn print_progress(label: &str, current: usize, total: usize) {
    if current == 1 || current == total || current.is_multiple_of(10) {
        println!("{label}: {current}/{total}");
    }
}
