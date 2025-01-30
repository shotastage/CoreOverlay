use std::{env, path::PathBuf, io};

pub fn get_work_dir() -> io::Result<PathBuf> {
    if cfg!(unix) {
        env::var("HOME")
            .map(|home| PathBuf::from(home).join(".compute-dht"))
            .map_err(|e| io::Error::new(io::ErrorKind::NotFound, e))
    } else {
        env::var("APPDATA")
            .map(|app_data| PathBuf::from(app_data).join("compute-dht"))
            .map_err(|e| io::Error::new(io::ErrorKind::NotFound, e))
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use std::env;

    #[test]
    fn test_get_work_dir_with_valid_env() {
        // Set up environment variables for testing
        if cfg!(unix) {
            env::set_var("HOME", "/home/testuser");
            let result = get_work_dir().unwrap();
            assert_eq!(result, PathBuf::from("/home/testuser/.compute-dht"));
        } else {
            env::set_var("APPDATA", r"C:\Users\testuser\AppData\Roaming");
            let result = get_work_dir().unwrap();
            assert_eq!(result, PathBuf::from(r"C:\Users\testuser\AppData\Roaming\compute-dht"));
        }
    }

    #[test]
    fn test_get_work_dir_with_missing_env() {
        // Temporarily remove the relevant environment variable
        if cfg!(unix) {
            env::remove_var("HOME");
            assert!(get_work_dir().is_err());
        } else {
            env::remove_var("APPDATA");
            assert!(get_work_dir().is_err());
        }
    }

    #[test]
    fn test_get_work_dir_path_structure() {
        let path = get_work_dir().unwrap();

        // Ensure the last component of the path is "compute-dht"
        assert_eq!(path.file_name().unwrap(), "compute-dht");

        // Ensure the path is absolute
        assert!(path.is_absolute());
    }
}
