use std::{
    collections::{HashMap, HashSet},
    fs, vec,
};

fn advent_load_string(file_name: &str) -> Vec<String> {
    let contents = fs::read_to_string(file_name).expect("Should've been able to read file");
    let return_value: Vec<&str> = contents.split("\n").collect();
    return_value.into_iter().map(String::from).collect()
}

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

pub fn advent_rucksort_sum() {
    let lines = advent_load_string("input3.txt");
    let mut big_items: Vec<char> = ('A'..='Z').collect();
    let mut all_items: Vec<char> = ('a'..='z').collect();
    all_items.append(&mut big_items);
    let mut counter = 1;

    let mut item_value_map = HashMap::<char, i32>::new();

    for item in &all_items {
        item_value_map.insert(item.to_owned(), counter);
        //println!("{item}:{counter:?} ");
        counter += 1;
    }

    fn split_and_separate(items: &str) -> (Vec<char>, Vec<char>) {
        let list_len = items.len() / 2;
        let (half1, half2) = items.split_at(list_len);
        (half1.chars().collect(), half2.chars().collect())
    }

    fn group_elves(elves: Vec<String>) -> Vec<(Vec<char>, Vec<char>, Vec<char>)> {
        let mut all_elves: Vec<(Vec<char>, Vec<char>, Vec<char>)> = Vec::new();
        for elf_group in elves.chunks(3) {
            all_elves.push((
                elf_group[0].chars().collect(),
                elf_group[1].chars().collect(),
                elf_group[2].chars().collect(),
            ))
        }
        all_elves
    }

    fn find_badge(elf_group: (Vec<char>, Vec<char>, Vec<char>)) -> char {
        fn return_common_val(
            largest_bag: Vec<char>,
            other_bags: (Vec<char>, Vec<char>),
        ) -> Option<char> {
            for item in largest_bag {
                if other_bags.0.contains(&item) && other_bags.1.contains(&item) {
                    return Some(item);
                }
            }
            return None;
        }

        let (elf1, elf2, elf3) = elf_group;
        let badge: char;

        if elf1.iter().len() > elf2.iter().len() && elf1.iter().len() > elf3.iter().len() {
            badge = return_common_val(elf1, (elf2, elf3)).expect("oops");
        } else if elf2.iter().len() > elf1.iter().len() && elf2.iter().len() > elf3.iter().len() {
            badge = return_common_val(elf2, (elf1, elf3)).expect("oops");
        } else if elf3.iter().len() > elf1.iter().len() && elf3.iter().len() > elf2.iter().len() {
            badge = return_common_val(elf3, (elf1, elf2)).expect("oops");
        } else {
            badge = return_common_val(elf1, (elf2, elf3)).expect("oops");
        }
        badge
    }

    fn hashset(data: &[char]) -> HashSet<char> {
        HashSet::from_iter(data.iter().cloned())
    }

    let backpack: Vec<(HashSet<char>, HashSet<char>)> = lines
        .iter()
        .map(|line| {
            let (half1, half2) = split_and_separate(line);
            (hashset(&half1), hashset(&half2))
        })
        .collect();
    let elf_groups = group_elves(lines);
    let elf_badges: Vec<(char, i32)> = elf_groups
        .iter()
        .map(|group| {
            let badge = find_badge(group.to_owned());
            (badge, item_value_map[&badge])
        })
        .collect();

    let mut duplicate_items: Vec<(char, i32)> = vec![];

    for compartment in backpack {
        for item in compartment.0 {
            if compartment.1.contains(&item) {
                duplicate_items.push((item, item_value_map[&item]));
            }
        }
    }
    for duplicate_item in &duplicate_items {
        assert!(duplicate_item.1.to_owned() == item_value_map[&duplicate_item.0]);
    }

    let sum: Vec<i32> = duplicate_items
        .iter()
        .map(|item_value| {
            let (_, value) = item_value;
            value.to_owned()
        })
        .collect();

    let badge_sum: Vec<i32> = elf_badges
        .iter()
        .map(|item_value| {
            let (_, value) = item_value;
            value.to_owned()
        })
        .collect();

    println!("ITEMS");
    for item in duplicate_items {
        println!("{:?}", item);
    }
    println!("total: {}", sum.into_iter().sum::<i32>());

    println!("BADGES");
    for badge in elf_badges {
        println!("{:?}", badge);
    }
    println!("total: {}", badge_sum.into_iter().sum::<i32>());
}

fn advent_overlap() {
    let lines = advent_load_string("test.txt");
    let bound_pairs: Vec<(String, String)> = lines
        .into_iter()
        .map(|line| {
            let line_pair = line.split_once(',').unwrap();
            (line_pair.0.to_owned(), line_pair.1.to_owned())
        })
        .collect();
    let split_pairs: Vec<((i32, i32), (i32, i32))> = bound_pairs
        .into_iter()
        .map(|pair| {
            let set1 = pair.0.split_once('-').unwrap();
            let set2 = pair.1.split_once('-').unwrap();

            let set1 = (set1.0.parse().unwrap(), set1.1.parse().unwrap());
            let set2 = (set2.0.parse().unwrap(), set2.1.parse().unwrap());
            (set1, set2)
        })
        .collect();
}

fn main() {
    //advent_day_1();
    //advent_day_2();
    //advent_rucksort_sum();
}
