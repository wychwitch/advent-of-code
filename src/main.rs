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
    let mut rounds: Vec<(&str, &str)> = Vec::new();
    let score_rounds: Vec<i32>;

    for round in rounds_combined {
        let mut rounds_split = round.split(" ");
        rounds.push((rounds_split.next().unwrap(), rounds_split.next().unwrap()));
    }
    let rounds_typed: Vec<(RPS, RPS)> = rounds
        .into_iter()
        .map(|round| {
            let l = match round.0 {
                "A" => RPS::Rock(1),
                "B" => RPS::Paper(2),
                "C" => RPS::Scissors(3),
                _ => panic!("Should've matched an enum!"),
            };
            let r = match round.1 {
                "X" => RPS::Rock(1),
                "Y" => RPS::Paper(2),
                "Z" => RPS::Scissors(3),
                _ => panic!("Should've matched an enum!"),
            };
            (l, r)
        })
        .collect();

    enum RPS {
        Rock(i32),
        Paper(i32),
        Scissors(i32),
    }

    score_rounds = rounds_typed
        .into_iter()
        .map(|round| {
            let mut my_score = 0;
            match round.0 {
                RPS::Paper(n) => my_score += n,
                RPS::Rock(n) => my_score += n,
                RPS::Scissors(n) => my_score += n,
            }
            match round.1 {
                RPS::Rock(m) => {
                    if my_score > m {
                        my_score + 6
                    } else if my_score == m {
                        my_score + 3
                    } else {
                        my_score
                    }
                }
                RPS::Paper(m) => {
                    if my_score > m {
                        my_score + 6
                    } else if my_score == m {
                        my_score + 3
                    } else {
                        my_score
                    }
                }
                RPS::Scissors(m) => {
                    if my_score > m {
                        my_score + 6
                    } else if my_score == m {
                        my_score + 3
                    } else {
                        my_score
                    }
                }
            }
        })
        .collect();

    score_rounds.into_iter().sum()
}

pub fn advent_day_2() {}

fn main() {
    //advent_day_1();
    println!("RPS score: {}", advent_process_rps_input());
}
