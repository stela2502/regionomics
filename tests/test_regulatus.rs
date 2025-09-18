#[cfg(test)]
mod tests {
    use std::process::Command;
    use std::fs;
    use std::path::Path;

    #[test]
    fn test_regulatos() {
        // Paths to input files and expected output
        let bed_file = "tests/data/test.bed";
        let gtf_file = "tests/data/test.gtf.gz";
        let output_file = "tests/data/results/result";
        let expected_file = "tests/data/result";

        // Ensure the output directory exists
        let output_dir = "tests/data/results";
        std::fs::create_dir_all(output_dir).expect("Failed to create results directory");

        // Detect binary path based on build mode
        let binary = if cfg!(debug_assertions) {
            // Debug mode binary path
            "target/debug/regulatos"
        } else {
            // Release mode binary path
            "target/release/regulatos"
        };

        let binary_path = Path::new(binary);

        // Ensure the binary exists
        assert!(
            binary_path.exists(),
            "Could not find regulatus binary at {}",
            binary_path.display()
        );

        // Run the command
        let status = Command::new(binary_path)
            .args([
                "-b", bed_file,
                "-g", gtf_file,
                "-d", "50000",
                "-o", output_file,
            ])
            .status()
            .expect("Failed to execute regulatus");

        // Ensure the command was successful
        assert!(status.success(), "Command failed with status {:?}", status);

        // Read and compare the output to the expected result
        let generated_output = fs::read_to_string(output_file).expect("Failed to read generated output");
        let expected_output = fs::read_to_string(expected_file).expect("Failed to read expected result");

        fn assert_str_eq_verbose(left: &str, right: &str) {
            let mut left_lines = left.lines();
            let mut right_lines = right.lines();

            for (i, (l, r)) in left_lines.by_ref().zip(right_lines.by_ref()).enumerate() {
                if l != r {
                    // Find first character difference
                    let col = l
                        .chars()
                        .zip(r.chars())
                        .position(|(lc, rc)| lc != rc)
                        .map(|p| p + 1) // 1-based column
                        .unwrap_or_else(|| l.len() + 1);
                    panic!(
                        "Strings differ at line {}, column {}:\n  left:  {:?}\n  right: {:?}",
                        i + 1,
                        col,
                        l,
                        r
                    );
                }
            }

            // If one has extra lines
            if left_lines.next().is_some() || right_lines.next().is_some() {
                panic!(
                    "Strings differ in length: left has {} lines, right has {} lines",
                    left.lines().count(),
                    right.lines().count()
                );
            }
        }
        assert_str_eq_verbose(&generated_output, &expected_output);

    }
}
