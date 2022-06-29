use crate::elements::*;
use std::fs;

use crate::pest::{Parser,iterators::Pair,RuleType};

#[derive(Parser)]
#[grammar = "squares.pest"]
pub struct SquaresParser;

fn to_color(color : &str) -> i8 {
    (color.chars().next().unwrap() as i8) - ('A' as i8)
}

fn to_dir(dir : &str) -> Dir {
    let c = dir.chars().next().unwrap();
    match c {
        '^' => Dir::Up,
        'v' => Dir::Down,
        '<' => Dir::Left,
        '>' => Dir::Right,
        _ => unreachable!(),
    }
}

fn to_pos<'a, T : RuleType>(coord : Pair<'a, T>) -> Pos
{
    let mut coords = coord.into_inner();
    let x = coords.next().unwrap().as_str();
    let x : i8 = x.parse::<i8>().unwrap();
    let y = coords.next().unwrap().as_str();
    let y : i8 = y.parse::<i8>().unwrap();
    Pos::new(x, y)
}

impl SquaresParser {
    pub fn parse_file(file: &str) -> Game {
        let unparsed_file = fs::read_to_string(file)
            .expect("cannot read file");
        let parsed_file = SquaresParser::parse(Rule::game, &unparsed_file)
            .expect("unsuccessful parse")
            .next().unwrap();
        let mut game = Game::default();
        for element in parsed_file.into_inner() {
            match element.as_rule() {
                Rule::element => {
                    for field in element.into_inner() {
                        //println!("{:?} -> {:?}", field.as_rule(), field.as_str());
                        match field.as_rule() {
                            Rule::square => {
                                let mut inner = field.into_inner();
                                let color : i8 = to_color(inner.next().unwrap().as_str());
                                let pos = to_pos(inner.next().unwrap());
                                let dir = to_dir(inner.next().unwrap().as_str());
                                game.squares.push(Square {color: color, pos: pos, dir: dir});
                            }
                            Rule::goal => {
                                let mut inner = field.into_inner();
                                let color : i8 = to_color(inner.next().unwrap().as_str());
                                let pos = to_pos(inner.next().unwrap());
                                game.goals.push(Goal {color: color, pos: pos});
                            }
                            Rule::turn => {
                                let mut inner = field.into_inner();
                                let dir = to_dir(inner.next().unwrap().as_str());
                                let pos = to_pos(inner.next().unwrap());
                                game.turns.push(Turn {pos: pos, dir: dir});
                            }
                            _ => unreachable!(),
                        }
                    }
                }
                Rule::EOI => (),
                _ => unreachable!(),
            }
        }
        game
    }
}
