use core::cmp::{min,max};
use colored::{Colorize,ColoredString,Color};

#[derive(Copy, Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct Pos {
    // using screen coordinates, y increases downwards
    pub x: i8,
    pub y: i8,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
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

#[derive(Debug)]
pub struct Square {
    pub pos: Pos,
    pub color: i8,
    pub dir: Dir,
}

#[derive(Debug, Default)]
pub struct Game {
    pub squares: Vec<Square>,
    pub goals: Vec<Goal>,
    pub turns: Vec<Turn>,
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
        Pos {x: x, y: y}
    }
}

impl Game {
    pub fn debug_print(&self) {
        let draw_colors = vec!["red", "green", "blue"];
        if self.squares.is_empty() || self.goals.is_empty() {
            println!("Incomplete puzzle: {:#?}", &self);
            return;
        }
        let mut tl = self.squares[0].pos;
        let mut br = tl;
        for e in &self.squares {
            tl.x = min(tl.x, e.pos.x);
            tl.y = min(tl.y, e.pos.y);
            br.x = max(br.x, e.pos.x);
            br.y = max(br.y, e.pos.y);
        }
        for e in &self.goals {
            tl.x = min(tl.x, e.pos.x);
            tl.y = min(tl.y, e.pos.y);
            br.x = max(br.x, e.pos.x);
            br.y = max(br.y, e.pos.y);
        }
        for e in &self.turns {
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
                let is_turn = self.turns.iter().find(|e| e.pos == curr_pos);
                let is_goal = self.goals.iter().find(|e| e.pos == curr_pos);
                let is_square = self.squares.iter().find(|e| e.pos == curr_pos);
                if !is_goal.is_none() {
                    match is_square {
                        None => { symbol="\u{25CB}"; fg = is_goal.unwrap().color; },
                        Some(sq) => {
                            bg = is_goal.unwrap().color;
                            fg = sq.color;
                            match sq.dir {
                                Dir::Up => { symbol="\u{25D3}"; },
                                Dir::Down => { symbol="\u{25D2}"; },
                                Dir::Left => { symbol="\u{25D0}"; },
                                Dir::Right => { symbol="\u{25D1}"; },
                            }
                        }
                    }
                } else if !is_turn.is_none() {
                    if is_square.is_none() {
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
                    } else {
                        fg = is_square.unwrap().color;
                        match is_turn {
                            Some(turn) => {
                                match turn.dir {
                                    Dir::Up => { symbol="\u{25B2}"; },
                                    Dir::Down => { symbol="\u{25BC}"; },
                                    Dir::Left => { symbol="\u{25C0}"; },
                                    Dir::Right => { symbol="\u{25B6}"; },
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
}
