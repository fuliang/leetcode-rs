use std::collections::HashMap;

#[derive(PartialEq, Eq, Hash, Debug, Clone, Copy)]
pub enum State {
    Start,
    Signed,
    InNumber,
    End,
}

pub struct DFA {
    sign: i64,
    ans: i64,
    state: State,
    table: HashMap<State, Vec<State>>,
}

impl DFA {
    fn new() -> DFA {
        let states = vec![State::Start, State::Signed, State::InNumber, State::End];
        let trans_matrix = vec![
            vec![State::Start, State::Signed, State::InNumber, State::End],
            vec![State::End, State::End, State::InNumber, State::End],
            vec![State::End, State::End, State::InNumber, State::End],
            vec![State::End, State::End, State::End, State::End],
        ];
        let table: HashMap<State, Vec<State>> = states.into_iter().zip(trans_matrix.into_iter()).collect();

        DFA { 
            sign: 1, 
            ans: 0,
            state: State::Start,
            table: table,
        }
    }

    fn get(&mut self, c: char) {
        let col = self.get_col(c);
        let row = self.table.get(&self.state).unwrap();
        self.state = *row.get(col).unwrap();
        match self.state {
            State::InNumber => {
                 self.ans = self.ans * 10 + (c as i64 - 48); //48 is '0'
                 self.ans = if self.sign == 1 { 
                    i64::min(self.ans, i32::MAX as i64)
                } else { 
                    i64::min(self.ans, -(i32::MIN as i64)) 
                }
            },
            State::Signed => {
                self.sign = if c == '+' {1} else {-1};
            }
            _ => {}
        }
    }

    fn get_col(&self, c: char) -> usize {
        match c {
            ' ' => 0,
            '+' | '-' => 1,
            '0'..='9' => 2,
            _ => 3, 
        }
    }
}

pub struct Solution;

impl Solution {
    pub fn my_atoi(s: String) -> i32 {
        let mut dfa = DFA::new();
        for c in s.chars() {
            dfa.get(c);
        }
        return (dfa.sign * dfa.ans) as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_c() {
        assert_eq!('0' as i32, 48);
    }

    #[test]
    fn test_case1() {
        let s = String::from("42");
        let i = Solution::my_atoi(s);
        assert_eq!(i, 42);
    }

    #[test]
    fn test_case2() {
        let s = String::from("   -42");
        let i = Solution::my_atoi(s);
        assert_eq!(i, -42);
    }

    #[test]
    fn test_case3() {
        let s = String::from("4193 with words");
        let i = Solution::my_atoi(s);
        assert_eq!(i, 4193);
    }

    #[test]
    fn test_case4() {
        let s = String::from("words and 987");
        let i = Solution::my_atoi(s);
        assert_eq!(i, 0);
    }

    #[test]
    fn test_case5() {
        let s = String::from("-91283472332");
        let i = Solution::my_atoi(s);
        assert_eq!(i, -2147483648);
    }
}