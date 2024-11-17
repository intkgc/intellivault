use std::{
    i64::MAX,
    sync::{Arc, RwLock},
};

use rand::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, FromFormField)]
pub enum SortingType {
    Downgrade,
    Upgrade,
    Random,
}

#[derive(Deserialize, Serialize, FromForm)]
pub struct KeywordsMatcher {
    pub keywords: Vec<String>,
    pub rules: MatcherRules,
}

#[derive(Deserialize, Serialize, FromForm)]
pub struct MatcherRules {
    take_first: i32,
    sorting_type: SortingType,
    minimum_threshold: i32,
}

pub struct Matcher {
    rules: MatcherRules,
    keywords: Vec<Arc<RwLock<Vec<i64>>>>,
}

impl Matcher {
    pub(crate) fn new(rules: MatcherRules, keywords: Vec<Arc<RwLock<Vec<i64>>>>) -> Self {
        Self { rules, keywords }
    }

    fn select(rules: &MatcherRules, mut files: Vec<(i32, i64)>) -> Vec<i64> {
        let count = rules.take_first as usize;

        
        match rules.sorting_type {
            SortingType::Downgrade => {
                files.sort_by(|x, y| y.0.cmp(&x.0));
                files.iter().map(|i| i.1).take(count).collect()
            }
            SortingType::Upgrade => {
                files.sort_by_key(|it| it.0);
                files.iter().map(|i| i.1).take(count).collect()
            }
            SortingType::Random => {
                let mut rng = rand::thread_rng();
                files
                    .choose_multiple(&mut rng, count)
                    .map(|i| i.1)
                    .collect()
            }
        }
    }
    pub(crate) fn find_matches(self) -> Vec<i64> {
        let mut files: Vec<(i32, i64)> = vec![];

        let keywords: Vec<_> = self.keywords.iter().map(|it| it.read().unwrap()).collect();

        let mut indeces: Vec<i32> = vec![0; keywords.len()];

        let mut previous = -1;
        let mut repeatitions = 1;

        let mut is_last_vec = false;
        if keywords.len() == 1 && self.rules.minimum_threshold <= 1 {
            return match self.rules.sorting_type {
                SortingType::Downgrade | SortingType::Upgrade => keywords[0]
                    .iter()
                    .map(|it| *it)
                    .take(self.rules.take_first as usize)
                    .collect(),
                SortingType::Random => {
                    let mut rng = rand::thread_rng();
                    keywords[0]
                        .choose_multiple(&mut rng, self.rules.take_first as usize)
                        .map(|it| *it)
                        .collect()
                }
            };
        }

        loop {
            if indeces
                .iter()
                .enumerate()
                .map(|index| {
                    (keywords[index.0].len() <= *index.1 as usize)
                        .then(|| 0)
                        .unwrap_or(1)
                })
                .sum::<i32>()
                <= 1
            {
                if is_last_vec {
                    break;
                }

                is_last_vec = true;
            }

            let min = indeces
                .iter_mut()
                .enumerate()
                .map(|it| {
                    let vec = &keywords[it.0];
                    let value = match (*it.1 as usize) < vec.len() {
                        true => vec[*it.1 as usize],
                        false => MAX,
                    };
                    (it.0, value)
                })
                .filter(|it| it.1 != MAX)
                .min_by(|x, y| x.1.cmp(&y.1));

            if let Some(value) = min {
                indeces[value.0] += 1;

                if previous == value.1 {
                    repeatitions += 1;
                }
                if (repeatitions >= self.rules.minimum_threshold)
                    && (previous != value.1 || is_last_vec)
                {
                    if previous != -1 {
                        files.push((repeatitions, previous));
                    }

                    
                }
                if previous != value.1 {
                    repeatitions = 1;
                }
                previous = value.1;
            } else {
                break;
            }
        }

        Self::select(&self.rules, files)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn match_test() {
        let rules = MatcherRules {
            take_first: 2,
            sorting_type: SortingType::Downgrade,
            minimum_threshold: 1,
        };

        let keywords = vec![vec![1, 2, 3, 4, 353, 3434, 3434, 3434, 2335, 533 ,23435 ,532]];

        let keywords: Vec<Arc<RwLock<Vec<i64>>>> = keywords
            .into_iter()
            .map(|it| Arc::new(RwLock::new(it)))
            .collect();

        let mut files = Matcher::new(rules, keywords).find_matches();
        files.sort();
        println!("{:?}", files);
        assert!(files == vec![2, 3], "files = {:?}", files)
    }
}
