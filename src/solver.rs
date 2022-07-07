use crate::elements::*;
use std::collections::{VecDeque,HashSet};
use std::convert::TryInto;

pub struct Solver;

impl Solver {
    pub fn solve(puzzle: Game) -> Option<Vec<String>> {
        let data = &puzzle.data;
        let initial_state = &puzzle.state;
        let area = get_area(initial_state, data);
        let mut tl = area.tl();
        let mut br = area.br();
        let num_sq = initial_state.squares.len() - 1;
        tl.x-=num_sq as i8;
        tl.y-=num_sq as i8;
        br.x+=num_sq as i8;
        br.y+=num_sq as i8;
        let area = Area::new(&tl, &br);
        let mut states = HashSet::new();
        let actors_num: i8 = initial_state.squares.len().try_into().unwrap();
        let mut parents = Vec::new();
        let mut queue = VecDeque::new();
        for action in 0..actors_num {
            let next_state = data.action(initial_state, action);
            if data.won(&next_state) {
                return Some(vec![data.color_map[action as usize].clone()])
            }
            parents.push((0, action));
            queue.push_back(next_state);
        }

        let mut index = 0;
        while let Some(parent) = queue.pop_front() {
            index += 1;
            if !states.contains(&parent) {
                for action in 0..actors_num {
                    let next_state = data.action(&parent, action);
                    if data.won(&next_state) {
                        println!("Explored {} states", parents.len());
                        let mut result = vec![data.color_map[action as usize].clone()];
                        while index != 0 {
                            let (next_index, action) = parents.swap_remove(index - 1);
                            result.push(data.color_map[action as usize].clone());
                            index = next_index;
                        }
                        result.reverse();
                        return Some(result);
                    } else {
                        if next_state.squares.iter().any(|sq| !area.inside(&sq.pos)) {
                            continue;
                        }
                        parents.push((index, action));
                        queue.push_back(next_state);
                    }
                }
                states.insert(parent);
            }
        }
        None
    }
}
