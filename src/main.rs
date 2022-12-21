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

pub fn advent_process_rps_input() -> (Vec<i32>, Vec<i32>) {
    let file_path = "input2.txt";
    let contents = fs::read_to_string(file_path).expect("Should've been able to read file");
    let rounds_combined: Vec<&str> = contents.split("\n").collect();
    let rounds: Vec<(Move, Move)>;
    let cheated_rounds: Vec<(Move, Move)>;
    let my_round_scores: Vec<i32>;
    let my_cheated_round_scores: Vec<i32>;
    let rounds_raw: Vec<(&str, &str)> = rounds_combined
        .into_iter()
        .filter_map(|round| match round {
            "\n" => None,
            _ => round.split_once(" "),
        })
        .collect();

    #[derive(Debug)]
    struct Cheat {
        goal: String,
    }

    impl Cheat {
        pub fn new(goal: &str) -> Cheat {
            Cheat {
                goal: match goal {
                    "X" => "Lose".to_string(),
                    "Y" => "Draw".to_string(),
                    "Z" => "Win".to_string(),
                    _ => panic!("Whoops!! Broken match!"),
                },
            }
        }
        pub fn cheated_move(&self, opponent: &Move) -> Move {
            match self.goal.as_str() {
                "Lose" => match opponent.shape.as_str() {
                    "Rock" => Move::new("C"),
                    "Paper" => Move::new("A"),
                    "Scissors" => Move::new("B"),
                    _ => panic!("match failed!"),
                },
                "Draw" => match opponent.shape.as_str() {
                    "Rock" => Move::new("A"),
                    "Paper" => Move::new("B"),
                    "Scissors" => Move::new("C"),
                    _ => panic!("match failed!"),
                },
                "Win" => match opponent.shape.as_str() {
                    "Rock" => Move::new("B"),
                    "Paper" => Move::new("C"),
                    "Scissors" => Move::new("A"),
                    _ => panic!("match failed!"),
                },
                _ => panic!("match failed!"),
            }
        }
    }

    #[derive(Debug)]
    struct Move {
        shape: String,
        score: i32,
        bonus: i32,
    }

    impl Move {
        pub fn new(shape_key: &str) -> Move {
            let shape: &str = match shape_key {
                "A" => "Rock",
                "B" => "Paper",
                "C" => "Scissors",
                //for backwards compatibility with pt 1
                "X" => "Rock",
                "Y" => "Paper",
                "Z" => "Scissors",
                _ => panic!("Huh! somehow the key wasn't right?"),
            };
            Move {
                shape: shape.to_string(),
                score: match shape {
                    "Rock" => 1,
                    "Paper" => 2,
                    "Scissors" => 3,
                    _ => panic!("Huh! somehow the key wasn't right?"),
                },
                bonus: 0,
            }
        }
        pub fn get_total(&self) -> i32 {
            self.score + self.bonus
        }

        pub fn process_round(opponent: &mut Move, me: &mut Move) {
            let my_shape = me.shape.to_string();
            let opp_shape = opponent.shape.to_string();
            if my_shape == opp_shape {
                me.bonus = 3;
                opponent.bonus = 3;
            } else if &my_shape == "Rock" {
                if &opp_shape == "Paper" {
                    me.bonus = 0;
                    opponent.bonus = 6;
                } else {
                    me.bonus = 6;
                    opponent.bonus = 0;
                }
            } else if &my_shape == "Scissors" {
                if &opp_shape == "Rock" {
                    me.bonus = 0;
                    opponent.bonus = 6;
                } else {
                    me.bonus = 6;
                    opponent.bonus = 0;
                }
            } else if &my_shape == "Paper" {
                if &opp_shape == "Scissors" {
                    me.bonus = 0;
                    opponent.bonus = 6;
                } else {
                    me.bonus = 6;
                    opponent.bonus = 0;
                }
            }
        }
    }
    rounds = rounds_raw
        .iter()
        .cloned()
        .map(|round| {
            let (opponent_shape_key, my_shape_key) = round;
            let mut opponent_move = Move::new(opponent_shape_key);
            let mut my_move = Move::new(my_shape_key);
            Move::process_round(&mut opponent_move, &mut my_move);
            (opponent_move, my_move)
        })
        .collect();
    cheated_rounds = rounds_raw
        .into_iter()
        .map(|round| {
            let (opponent_shape_key, my_goal_key) = round.clone();
            let mut opponent_move = Move::new(opponent_shape_key);
            let cheat = Cheat::new(my_goal_key);
            let mut my_move = cheat.cheated_move(&opponent_move);
            Move::process_round(&mut opponent_move, &mut my_move);
            (opponent_move, my_move)
        })
        .collect();

    my_round_scores = rounds
        .iter()
        .map(|round| {
            let (_, my_round) = round;
            my_round.get_total()
        })
        .collect();
    my_cheated_round_scores = cheated_rounds
        .into_iter()
        .map(|round| {
            let (_, my_round) = round;
            my_round.get_total()
        })
        .collect();

    (my_round_scores, my_cheated_round_scores)
}

pub fn advent_day_2() {
    let (day_2_1_vec, day_2_2_vec) = advent_process_rps_input();
    let day_2_1: i32 = day_2_1_vec.into_iter().sum();
    let day_2_2: i32 = day_2_2_vec.into_iter().sum();
    println!("RPS score: {}\n Cheated: {}", day_2_1, day_2_2);
}

fn main() {
    //advent_day_1();
    advent_day_2();
}
