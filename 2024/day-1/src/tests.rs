#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn calculate_difference_short_case(){
        assert_eq!(calculate_total_diff("foo.txt").unwrap(), 11);
    }

    #[test]
    fn calculate_difference_long_case(){
        assert_eq!(calculate_total_diff("input.txt").unwrap(), 2031679);
    }
}