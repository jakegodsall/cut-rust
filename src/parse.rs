pub struct Range {
    pub start: Option<usize>,
    pub end: Option<usize>,
}

/// Extract range statement into a Range struct
/// 
/// # Example
/// 
/// ```
/// let range_str = "3-5";
/// let res = cut_rust::parse::parse_range(range_str).unwrap();
/// 
/// assert_eq!(res.start, Some(3));
/// assert_eq!(res.end, Some(5));
/// ```
/// 
pub fn parse_range(input: &str) -> Result<Range, &'static str> {
    let s = input.trim();
    if s.is_empty() {
        return Err("empty string");
    }

    if s.matches('-').count() > 1 {
        return Err("too many '-' characters");
    }

    if let Some((left, right)) = s.split_once('-') {
        let start = if left.trim().is_empty() {
            None
        } else {
            Some(left.trim().parse::<usize>()
                .map_err(|_| "invalid start")?)
        };

        let end = if right.trim().is_empty() {
            None
        } else {
            Some(right.trim().parse::<usize>()
                .map_err(|_| "invalid end")?)
        };

        Ok(Range { start, end })
    } else {
        let n = s.parse::<usize>().map_err(|_| "invalid numbrer")?;
        Ok(Range {
            start: Some(n),
            end: Some(n)
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_range_with_start_and_end() {
        let range_str = "1-2";
        let res = parse_range(range_str).unwrap();
        assert_eq!(res.start, Some(1));
        assert_eq!(res.end, Some(2));
    }

    #[test]
    fn parses_range_with_start_exclusive() {
        let range_str = "1-";
        let res = parse_range(range_str).unwrap();
        assert_eq!(res.start, Some(1));
        assert_eq!(res.end, None);
    }

    #[test]
    fn parses_range_with_end_exclusive() {
        let range_str = "-2";
        let res = parse_range(range_str).unwrap();
        assert_eq!(res.start, None);
        assert_eq!(res.end, Some(2));
    }

    #[test]
    fn parses_range_empty() {
        let range_str = "-";
        let res = parse_range(range_str).unwrap();
        assert_eq!(res.start, None);
        assert_eq!(res.end, None);
    }
}

pub fn parse_list(list: String) -> Result<Vec<i32>, Box<dyn std::error::Error>> {
    let nums: Result<Vec<i32>, _> = list
        .split(',')
        .map(str::trim)
        .filter(|s| !s.is_empty())
        .map(str::parse::<i32>)
        .collect();

    Ok(nums?)
}