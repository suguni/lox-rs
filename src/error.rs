use std::cell::Cell;

pub fn error(line: usize, message: &str) {
    report(line, "", message);
}

pub fn report(line: usize, place: &str, message: &str) {
    eprintln!("[line {line}] Error{place}: {message}");
    set_error(true);
}

thread_local! {
    static HAS_ERROR: Cell<bool> = Cell::new(false);
}

pub fn set_error(error: bool) {
    HAS_ERROR.set(error);
}

pub fn get_error() -> bool {
    HAS_ERROR.get()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn has_error() {
        assert!(!get_error());
        set_error(true);
        assert!(get_error());
        set_error(false);
        assert!(!get_error());
    }
}
