use serde::{Deserialize, Serialize};

use std::cmp::Ordering;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(untagged)]
pub enum IntList {
    List(Vec<IntList>),
    Integer(i32),
}

impl IntList {
    pub fn parse(input_line: String) -> Self {
        let serde_val = serde_json::from_str::<IntList>(&input_line);
        match serde_val {
            Ok(val) => val,
            Err(_) => unreachable!(),
        }
    }

    pub fn compare(&self, right: &Self) -> Ordering {
        match (self, right) {
            (&Self::Integer(left), &Self::Integer(right)) => Self::compare_number(left, right),
            (IntList::List(left), IntList::List(right)) => Self::compare_list(left, right),
            (IntList::List(left), &IntList::Integer(right)) => {
                Self::compare_list(left, &[Self::Integer(right)])
            }
            (&IntList::Integer(left), IntList::List(right)) => {
                Self::compare_list(&[Self::Integer(left)], right)
            }
        }
    }

    fn compare_number(left: i32, right: i32) -> Ordering {
        left.cmp(&right)
    }

    fn compare_list(left: &[Self], right: &[Self]) -> Ordering {
        for i in 0..left.len().max(right.len()) {
            //left is longer
            let Some(left) = left.get(i) else {
                return Ordering::Less
            };
            //left is shorter
            let Some(right) = right.get(i) else {
                return Ordering::Greater
            };
            if !left.compare(right).is_eq() {
                return left.compare(right);
            }
        }
        Ordering::Equal
    }
}
