
struct Logger {}

impl Logger {
    pub fn new() -> Self {
        Self {}
    }

    pub fn log(msg: String) {
        println!("{}", msg);
    }

    pub fn warning(msg: String) {
        eprintln!("{}", msg);
    }

    pub fn error(msg: String) {
        eprintln!("{}", msg);
    }
}
