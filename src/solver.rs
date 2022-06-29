use crate::elements::*;
use std::collections::VecDeque;
use std::convert::TryInto;

pub struct Solver;

impl Solver {
    pub fn solve(puzzle: Game) -> Option<Vec<i8>> {
        let data = &puzzle.data;
        let initial_state = &puzzle.state;
        let actors_num: i8 = initial_state.squares.len().try_into().unwrap();
        let mut parents = Vec::new();
        let mut queue = VecDeque::new();
        for action in 0..actors_num {
            let next_state = data.action(initial_state, action);
            if data.won(&next_state) {
                return Some(vec![action])
            }
            parents.push((0, action));
            queue.push_back(next_state);
        }

        let mut index = 0;
        while let Some(parent) = queue.pop_front() {
            index += 1;
            for action in 0..actors_num {
                let next_state = data.action(&parent, action);
                if data.won(&next_state) {
                    let mut result = vec![action];
                    while index != 0 {
                        let (next_index, action) = parents.swap_remove(index - 1);
                        result.push(action);
                        index = next_index;
                    }
                    result.reverse();
                    return Some(result);
                } else {
                    parents.push((index, action));
                    queue.push_back(next_state);
                }
            }
        }
        None
    }
}
