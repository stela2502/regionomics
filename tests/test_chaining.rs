#[cfg(test)]
mod tests {
    #[test]
    fn test_array_merge() {
        let a = vec!["A".to_string(), "B".to_string()];
        let b = vec!["C".to_string(), "D".to_string()];
        assert_eq!(
                a.iter().chain( b.iter()).map(|s| s.clone()).collect::<Vec<String>>().join("\t"),
                "A\tB\tC\tD",
                "Arrays do not integrate as expected?!"
            );
    }
}