use std::process::Command;
use std::path::Path;
use std::io::Result;

pub fn execute_command_advanced(
    command: &str,
    args: &[&str],
    working_dir: Option<&Path>,
    env_vars: Option<&[(&str, &str)]>,
) -> Result<String> {
    let mut cmd = Command::new(command);

    cmd.args(args);

    if let Some(dir) = working_dir {
        cmd.current_dir(dir);
    }

    if let Some(vars) = env_vars {
        for (key, value) in vars {
            cmd.env(key, value);
        }
    }

    let output = cmd.output()?;

    if output.status.success() {
        Ok(String::from_utf8_lossy(&output.stdout).to_string())
    } else {
        let error = String::from_utf8_lossy(&output.stderr);
        Err(std::io::Error::new(
            std::io::ErrorKind::Other,
            format!("Command failed: {}", error)
        ))
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use std::env;
    use std::fs;
    use tempfile::TempDir;

    #[test]
    fn test_basic_command_execution() -> Result<()> {
        let output = execute_command_advanced(
            "echo",
            &["Hello, World!"],
            None,
            None,
        )?;

        assert_eq!(output.trim(), "Hello, World!");
        Ok(())
    }

    #[test]
    fn test_command_with_working_directory() -> Result<()> {
        let temp_dir = TempDir::new()?;
        let temp_path = temp_dir.path();

        // テスト用のファイルを作成
        fs::write(temp_path.join("test.txt"), "test content")?;

        let output = execute_command_advanced(
            "ls",
            &[],
            Some(temp_path),
            None,
        )?;

        assert!(output.contains("test.txt"));
        Ok(())
    }

    #[test]
    fn test_command_with_env_vars() -> Result<()> {
        let env_vars = &[("TEST_VAR", "test_value")];

        let output = execute_command_advanced(
            "printenv",
            &["TEST_VAR"],
            None,
            Some(env_vars),
        )?;

        assert_eq!(output.trim(), "test_value");
        Ok(())
    }

    #[test]
    fn test_command_failure() {
        let result = execute_command_advanced(
            "non_existent_command",
            &[],
            None,
            None,
        );

        assert!(result.is_err());
        if let Err(e) = result {
            assert!(e.to_string().contains("Command failed"));
        }
    }

    #[test]
    fn test_command_with_multiple_args() -> Result<()> {
        let output = execute_command_advanced(
            "echo",
            &["arg1", "arg2", "arg3"],
            None,
            None,
        )?;

        assert_eq!(output.trim(), "arg1 arg2 arg3");
        Ok(())
    }

    #[test]
    fn test_command_with_multiple_env_vars() -> Result<()> {
        let env_vars = &[
            ("TEST_VAR1", "value1"),
            ("TEST_VAR2", "value2"),
        ];

        let output = execute_command_advanced(
            "sh",
            &["-c", "echo $TEST_VAR1 $TEST_VAR2"],
            None,
            Some(env_vars),
        )?;

        assert_eq!(output.trim(), "value1 value2");
        Ok(())
    }
}
