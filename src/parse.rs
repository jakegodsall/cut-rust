pub struct Range {
    pub start: Option<usize>,
    pub end: Option<usize>,
}

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

fn parse_list(list: String) -> Result<Vec<i32>, Box<dyn std::error::Error>> {
    let nums: Result<Vec<i32>, _> = list
        .split(',')
        .map(str::trim)
        .filter(|s| !s.is_empty())
        .map(str::parse::<i32>)
        .collect();

    Ok(nums?)
}