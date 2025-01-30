/// Displays a message to standard output
///
/// This function prints a decorated message indicating that something
/// is under construction or in development. The message consists of
/// three lines with "======" borders.
///
/// # Example
///
/// ```
/// message();
/// // Output:
/// // ==============================
/// // ## Under construction >>>
/// // ==============================
/// ```
///
/// # Notes
///
/// This function simply prints to standard output and
/// does not return any value.
pub fn message() {
    println!("==============================");
    println!("## Under construction >>>");
    println!("==============================");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_message() {
        // Capture the output of the message function
        let output = std::panic::catch_unwind(|| {
            message();
        });

        // Check if the function executed without panicking
        assert!(output.is_ok());

        // Note: To actually verify the printed output, you would need to capture stdout,
        // which is more complex and typically not done in simple unit tests.
    }
}
