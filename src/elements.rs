use core::cmp::{min,max};
use colored::{Colorize,ColoredString,Color};

#[derive(Copy, Clone, Debug, Eq, Ord, PartialEq, PartialOrd, Hash)]
pub struct Pos {
    // using screen coordinates, y increases downwards
    pub x: i8,
    pub y: i8,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
#[repr(i8)]
pub enum Dir {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug)]
pub struct Goal {
    pub pos: Pos,
    pub color: i8,
}

#[derive(Debug)]
pub struct Turn {
    pub pos: Pos,
    pub dir: Dir,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub struct Square {
    pub pos: Pos,
    pub color: i8,
    pub dir: Dir,
}

#[derive(Debug, Default)]
pub struct GameData {
    pub goals: Vec<Goal>,
    pub turns: Vec<Turn>,
    pub color_map: Vec<String>,
}

#[derive(Debug, Default, Clone, Eq, PartialEq, Hash)]
pub struct State {
    pub squares: Vec<Square>,
}

#[derive(Debug, Default)]
pub struct Game {
    pub data: GameData,
    pub state: State,
}

impl Pos {
    pub fn step(&mut self, d: Dir) {
        match d {
            Dir::Up => self.y -= 1i8,
            Dir::Down => self.y += 1i8,
            Dir::Left => self.x -= 1i8,
            Dir::Right => self.x += 1i8,
        }
    }
    pub fn new(x: i8, y: i8) -> Pos {
        Pos {x, y}
    }
}

impl GameData {
    pub fn action(&self, state: &State, color: i8) -> State {
        let mut ret = state.clone();
        let mut acted = ret.squares.iter_mut().find(|e| e.color == color);
        let mut dir = Dir::Up;
        if let Some(ref initial) = acted {
            dir = initial.dir;
        }
        while let Some(ref mut sq) = acted {
            sq.pos.step(dir);
            let p = sq.pos;
            let c = sq.color;
            if let Some(turn) = self.turns.iter().find(|e| e.pos == p) {
                sq.dir = turn.dir;
            }
            acted = ret.squares.iter_mut().find(|e| e.pos == p && e.color != c);
        }
        ret
    }
    pub fn won(&self, state: &State) -> bool {
        for goal in self.goals.iter() {
            if !state.squares.iter().any(|e| e.pos == goal.pos && e.color == goal.color) {
                return false;
            }
        }
        true
    }
}


pub fn debug_print(state: &State, data: &GameData) {
    let draw_colors = vec!["red", "green", "blue", "magenta", "cyan", "yellow"];
    if state.squares.is_empty() || data.goals.is_empty() {
        println!("Incomplete puzzle: {:#?} {:#?}", &state, &data);
        return;
    }
    let mut tl = state.squares[0].pos;
    let mut br = tl;
    for e in &state.squares {
        tl.x = min(tl.x, e.pos.x);
        tl.y = min(tl.y, e.pos.y);
        br.x = max(br.x, e.pos.x);
        br.y = max(br.y, e.pos.y);
    }
    for e in &data.goals {
        tl.x = min(tl.x, e.pos.x);
        tl.y = min(tl.y, e.pos.y);
        br.x = max(br.x, e.pos.x);
        br.y = max(br.y, e.pos.y);
    }
    for e in &data.turns {
        tl.x = min(tl.x, e.pos.x);
        tl.y = min(tl.y, e.pos.y);
        br.x = max(br.x, e.pos.x);
        br.y = max(br.y, e.pos.y);
    }
    for y in tl.y..=br.y {
        let mut line = String::default();
        for x in tl.x..=br.x {
            let curr_pos = Pos::new(x,y);
            let symbol;
            let mut fg : i8 = -1i8;
            let mut bg : i8 = -1i8;
            let is_turn = data.turns.iter().find(|e| e.pos == curr_pos);
            let is_goal = data.goals.iter().find(|e| e.pos == curr_pos);
            let is_square = state.squares.iter().find(|e| e.pos == curr_pos);
            if let Some(goal) = is_goal {
                match is_square {
                    None => { symbol="\u{25CB}"; fg = goal.color; },
                    Some(sq) => {
                        bg = goal.color;
                        fg = sq.color;
                        match sq.dir {
                            Dir::Up => { symbol="\u{25D3}"; },
                            Dir::Down => { symbol="\u{25D2}"; },
                            Dir::Left => { symbol="\u{25D0}"; },
                            Dir::Right => { symbol="\u{25D1}"; },
                        }
                    }
                }
            } else if let Some(turn) = is_turn {
                if let Some(sq) = is_square {
                    fg = sq.color;
                    match turn.dir {
                        Dir::Up => { symbol="\u{25B2}"; },
                        Dir::Down => { symbol="\u{25BC}"; },
                        Dir::Left => { symbol="\u{25C0}"; },
                        Dir::Right => { symbol="\u{25B6}"; },
                    }
                } else {
                    match is_turn {
                        Some(turn) => {
                            match turn.dir {
                                Dir::Up => { symbol="\u{25B3}"; },
                                Dir::Down => { symbol="\u{25BD}"; },
                                Dir::Left => { symbol="\u{25C1}"; },
                                Dir::Right => { symbol="\u{25B7}"; },
                            }
                        },
                        None => unreachable!(),
                    }
                }
            } else {
                match is_square {
                    None => { symbol=" "; },
                    Some(sq) => {
                        fg = sq.color;
                        match sq.dir {
                            Dir::Up => { symbol="\u{2B12}"; },
                            Dir::Down => { symbol="\u{2B13}"; },
                            Dir::Left => { symbol="\u{25E7}"; },
                            Dir::Right => { symbol="\u{25E8}"; },
                        }
                    }
                }
            }
            let mut formatted = ColoredString::from(symbol);
            if bg >= 0i8 {
                formatted = formatted.on_color(Color::from(draw_colors[bg as usize]));
            }
            if fg >= 0i8 {
                formatted = formatted.color(Color::from(draw_colors[fg as usize]));
            }
            line += &formatted.to_string();
        }
        println!("{}", line);
    }
}
