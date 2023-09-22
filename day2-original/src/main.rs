use std::collections::HashMap;

#[derive(PartialEq, Clone, Copy, Debug)]
enum Shape {
    Rock,
    Paper,
    Scissors,
}

impl Shape {
    fn beats(&self) -> Shape {
        match &self {
            Shape::Rock => Shape::Scissors,
            Shape::Paper => Shape::Rock,
            Shape::Scissors => Shape::Paper,
        }
    }

    fn loses(&self) -> Shape {
        match &self {
            Shape::Rock => Shape::Paper,
            Shape::Paper => Shape::Scissors,
            Shape::Scissors => Shape::Rock,
        }
    }
}

#[derive(Debug)]
struct Game {
    my_hand: Shape,
    op_hand: Shape,
}

impl Game {
    fn base_score(&self) -> u32 {
        match &self.my_hand {
            Shape::Rock => 1,
            Shape::Paper => 2,
            Shape::Scissors => 3,
        }
    }

    fn comp_score(&self) -> u32 {
        let my_hand_beats = &self.my_hand.beats();
        let op_hand_beats = &self.op_hand.beats();
        if my_hand_beats == &self.op_hand {
            return 6;
        } else if op_hand_beats == &self.my_hand {
            return 0;
        } else {
            return 3;
        }
    }
}

enum Result {
    Win,
    Lose,
    Draw,
}

fn get_required_hand(op_hand: &Shape, required_result: &Result) -> Shape {
    match required_result {
        Result::Win => op_hand.loses(),
        Result::Lose => op_hand.beats(),
        Result::Draw => *op_hand,
    }
}

fn main() {
    let input = std::fs::read_to_string("input.txt").expect("unable to read input.txt");

    let mut chars_to_hands = HashMap::new();
    chars_to_hands.insert("A", Shape::Rock);
    chars_to_hands.insert("B", Shape::Paper);
    chars_to_hands.insert("C", Shape::Scissors);
    chars_to_hands.insert("X", Shape::Rock);
    chars_to_hands.insert("Y", Shape::Paper);
    chars_to_hands.insert("Z", Shape::Scissors);

    let mut chars_to_results = HashMap::new();
    chars_to_results.insert("X", Result::Lose);
    chars_to_results.insert("Y", Result::Draw);
    chars_to_results.insert("Z", Result::Win);

    let mut part1_score = 0;
    let mut part2_score = 0;
    for line in input.lines() {
        let op_hand = chars_to_hands.get(&line[..1]).unwrap();
        let my_part1_hand = chars_to_hands.get(&line[2..]).unwrap();
        let part1_game = Game {
            my_hand: *my_part1_hand,
            op_hand: *op_hand,
        };
        part1_score += part1_game.base_score();
        part1_score += part1_game.comp_score();

        let required_result = chars_to_results.get(&line[2..]).unwrap();
        let my_part2_hand = get_required_hand(op_hand, required_result);
        let part2_game = Game {
            my_hand: my_part2_hand,
            op_hand: *op_hand,
        };
        part2_score += part2_game.base_score();
        part2_score += part2_game.comp_score();
    }

    println!("Part 1 score: {part1_score}");
    println!("Part 2 score: {part2_score}");
}
