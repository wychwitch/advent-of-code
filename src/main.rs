use std::collections::HashMap;
use std::fs;

pub fn advent_get_elf_calories() -> Vec<Vec<i32>> {
    let file_path = "input1.txt";
    let contents = fs::read_to_string(file_path).expect("Should've been able to read file");
    let string_vecs: Vec<&str> = contents.split("\n\n").collect();
    let mut elves: Vec<Vec<i32>> = Vec::new();

    for block in &string_vecs {
        let mut elf: Vec<i32> = Vec::new();
        for calorie in block.split("\n") {
            match calorie.parse() {
                Ok(n) => elf.push(n),
                Err(_) => println!(
                    "Huh! An error? Elf looks like this {:?} last elves looks like this {:?}",
                    elf,
                    elves.last()
                ),
            }
        }
        elves.push(elf.into());
    }
    elves
}

pub fn advent_get_elf_calorie_totals() -> Vec<i32> {
    let elf_calories = advent_get_elf_calories();

    let mut elf_calories_totals: Vec<i32> = Vec::new();

    for elf in elf_calories {
        let total: i32 = elf.iter().sum::<i32>();
        elf_calories_totals.push(total)
    }

    elf_calories_totals.sort();
    elf_calories_totals.reverse();

    elf_calories_totals
}

pub fn advent_day_1() {
    let elf_calorie_totals = advent_get_elf_calorie_totals();
    println!(
        "Top Elf:{}, Top 3 elves total: {}",
        elf_calorie_totals[0],
        elf_calorie_totals[0] + elf_calorie_totals[1] + elf_calorie_totals[2]
    );
}

pub fn advent_process_rps_input() -> i32 {
    let file_path = "input2.txt";
    let contents = fs::read_to_string(file_path).expect("Should've been able to read file");
    let rounds_combined: Vec<&str> = contents.split("\n").collect();

    let my_round_scores: Vec<i32>;
    let opponent_shape_map = HashMap::from([("A", 1), ("B", 2), ("C", 3)]);
    let my_shape_map = HashMap::from([("X", 1), ("Y", 2), ("Z", 3)]);
    let rounds_raw: Vec<(&str, &str)> = rounds_combined
        .into_iter()
        .filter_map(|round| match round {
            "\n" => None,
            _ => round.split_once(" "),
        })
        .collect();

    #[derive(Debug)]
    struct Move {
        shape: i32,
        bonus: i32,
    }

    impl Move {
        pub fn get_total(&self) -> i32 {
            self.shape + self.bonus
        }
    }
    let rounds: Vec<(Move, Move)> = rounds_raw
        .into_iter()
        .map(|round| {
            let (opponent_shape, my_shape) = round;
            let my_bonus: i32;
            let opponent_bonus: i32;
            if my_shape > opponent_shape {
                my_bonus = 6;
                opponent_bonus = 0;
            } else if my_shape == opponent_shape {
                my_bonus = 3;
                opponent_bonus = 3;
            } else {
                my_bonus = 0;
                opponent_bonus = 0;
            };

            let opponent = Move {
                shape: opponent_shape_map[opponent_shape],
                bonus: opponent_bonus,
            };
            let me = Move {
                shape: my_shape_map[my_shape],
                bonus: my_bonus,
            };
            (opponent, me)
        })
        .collect();

    my_round_scores = rounds
        .into_iter()
        .map(|round| {
            let (_, my_round) = round;
            my_round.get_total()
        })
        .collect();

    my_round_scores.into_iter().sum()
}

pub fn advent_day_2() {}

fn main() {
    //advent_day_1();
    println!("RPS score: {}", advent_process_rps_input());
}
