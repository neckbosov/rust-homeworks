use std::collections::HashMap;
use std::ops::AddAssign;

pub fn transform_iter<'a, I>(old: I) -> HashMap<char, u32>
where
    I: Iterator<Item = (&'a u32, &'a Vec<char>)>,
{
    old.fold(HashMap::new(), |mut new_table, (score, chars)| {
        for c in chars {
            new_table
                .entry(c.to_ascii_lowercase())
                .or_insert(0)
                .add_assign(score);
        }
        new_table
    })
}

pub fn transform(old: &HashMap<u32, Vec<char>>) -> HashMap<char, u32> {
    transform_iter(old.iter())
}

pub fn score(input: &str, score_table: &HashMap<char, u32>) -> u32 {
    input.chars().fold(0, |acc, c| {
        acc + score_table.get(&c.to_ascii_lowercase()).unwrap_or(&0)
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_data() -> HashMap<u32, Vec<char>> {
        let mut data = HashMap::new();
        data.insert(1, vec!['A', 'E', 'I', 'O', 'U', 'L', 'N', 'R', 'S', 'T']);
        data.insert(2, vec!['D', 'G']);
        data.insert(3, vec!['B', 'C', 'M', 'P']);
        data.insert(4, vec!['F', 'H', 'V', 'W', 'Y']);
        data.insert(5, vec!['K']);
        data.insert(8, vec!['J', 'X']);
        data.insert(10, vec!['Q', 'Z']);
        data
    }

    #[test]
    fn test_transform() {
        let data = test_data();

        let new_data = transform(&data);
        assert_eq!(new_data[&'a'], 1);
        assert_eq!(new_data[&'b'], 3);
        assert_eq!(new_data[&'c'], 3);
        assert_eq!(new_data[&'q'], 10);
        assert_eq!(new_data[&'k'], 5);
        assert_eq!(new_data[&'t'], 1);
        assert_eq!(new_data.len(), 26);
    }

    #[test]
    fn test_score() {
        let input = "The quick brown fox jumps over the lazy dog";
        assert_eq!(score(&input, &transform(&test_data())), 99);
    }
}
