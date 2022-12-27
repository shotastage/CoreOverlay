struct Platform {}

impl Platform {
    pub fn new() -> Self {
        Self {}
    }

    pub fn get_platform() -> String {
        let platform = "macOS";
        platform.to_string()
    }
}
