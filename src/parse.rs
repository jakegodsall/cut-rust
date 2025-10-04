pub enum Selection {
    Range { start: Option<usize>, end: Option<usize> },
    List(Vec<usize>),
}

/// Extract range statement into a Selection::Range struct
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
pub fn parse_range(input: &str) -> Result<Selection, &'static str> {
    let s = input.trim();
    if s.is_empty() {
        return Err("empty string");
    }

    if s.matches("-").count() > 1 {
        return Err("too many '-' characters");
    }

    if let Some((left, right)) = s.split_once("-") {
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
        
        Ok(Selection::Range { start, end })
    } else {
        let n = s.parse::<usize>().map_err(|_| "invalid numbrer")?;
            Ok(Selection::Range {
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
        match res {
            Selection::Range { start, end } => {
                assert_eq!(start, Some(1));
                assert_eq!(end, Some(2));
            }
            _ => panic!("expected Selection::Range")
        }
    }

    #[test]
    fn parses_range_with_start_exclusive() {
        let range_str = "1-";
        let res = parse_range(range_str).unwrap();
        match res {
            Selection::Range { start, end } => {
                assert_eq!(start, Some(1));
                assert_eq!(end, None);
            },
            _ => panic!("expected Selection::Range"),
        }
    }

    #[test]
    fn parses_range_with_end_exclusive() {
        let range_str = "-2";
        let res = parse_range(range_str).unwrap();
        match res {
            Selection::Range { start, end } => {
                assert_eq!(start, None);
                assert_eq!(end, Some(2));
            },
            _ => panic!("unexpected Selection::Range"),
        }
    }

    #[test]
    fn parses_range_empty() {
        let range_str = "-";
        let res = parse_range(range_str).unwrap();
        match res {
            Selection::Range { start, end } => {
                assert_eq!(start, None);
                assert_eq!(end, None)
            }
            _ => panic!("expected Selection::Range"),
        }
    }
}

/// Extract list into a Selection::List struct
/// 
/// # Example
/// 
/// ```
/// use cut_rust::parse::{parse_list, Selection};
///
/// let list_str = "1,2,3";
/// let res = parse_list(list_str).unwrap();
///
/// match res {
///     Selection::List(v) => {
///         assert_eq!(v, vec![1, 2, 3]);
///     }
///     _ => panic!("expected Selection::List"),
/// }
/// ```
pub fn parse_list(list: &str) -> Result<Selection, Box<dyn std::error::Error>> {
    let nums: Vec<usize> = list
        .split(',')
        .map(str::trim)
        .filter(|s| !s.is_empty())
        .map(|s| s.parse::<usize>().unwrap())
        .collect();

    Ok(Selection::List(nums))
}

pub fn parse_delimiter(delimiter: &str) -> Result<char, Box<dyn std::error::Error>> {
    Ok(delimiter.parse::<char>().map_err(|_| format!("delimiter must be exactly one character, got {:?}", delimiter))?)
}