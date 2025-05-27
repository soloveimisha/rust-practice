pub fn gray(n: u8) -> Vec<String> {
    if n == 0 {
        return vec![String::new()];
    }
    if n == 1 {
        return vec!["0".to_string(), "1".to_string()];
    }

    let prev = gray(n - 1);
    let mut result = Vec::with_capacity(prev.len() * 2);

    
    for code in &prev {
        result.push(format!("0{}", code));
    }
    
    for code in prev.iter().rev() {
        result.push(format!("1{}", code));
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let test_data = [
            (0, vec!("")),
           (1, vec!("0", "1")),
           (2, vec!("00", "01", "10", "11")),
           (3, vec!("000", "001", "010", "011", 
                    "100", "101", "110", "111")),
           (4, vec!("0000", "0001", "0010", "0011", 
                    "0100", "0101", "0110", "0111", 
                    "1000", "1001", "1010", "1011",
                    "1100", "1101", "1110", "1111")),
        ];

        for (n, expected) in test_data.iter() {
            assert_eq!(gray(*n), expected.iter().map(|&s| s.to_string()).collect::<Vec<_>>());
        }
    }
}
