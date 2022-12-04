use std::str::FromStr;

#[derive(Debug)]
pub struct Pair {
    min: i32,
    max: i32,
}

impl FromStr for Pair {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let pair: Vec<i32> = s
            .split('-')
            .map(|pair| pair.parse::<i32>().unwrap())
            .collect();

        Ok(Pair {
            min: pair[0],
            max: pair[1],
        })
    }
}

impl Pair {
    pub fn is_subset(&self, other: &Pair) -> bool {
        self.min >= other.min && self.max <= other.max
    }

    pub fn has_overlap(&self, other: &Pair) -> bool {
        let min_overlap = self.min >= other.min && self.min <= other.max;
        let max_overlap = self.max >= other.min && self.max <= other.max;
        let is_subset = other.is_subset(self);
        min_overlap || max_overlap || is_subset
    }
}
