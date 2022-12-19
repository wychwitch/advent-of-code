use std::fs;

pub fn advent_day_1_1() {
    let file_path = "input1.txt";
    let contents = fs::read_to_string(file_path).expect("Should've been able to read file");
    let string_vecs: Vec<&str> = contents.split("\n\n").collect();
    let mut elves: Vec<Vec<i32>> = Vec::new();
    for block in &string_vecs {
        let mut elf: Vec<i32> = Vec::new();
        for calorie in block.split("\n") {
            match calorie.parse::<i32>() {
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

    let mut elves_total: Vec<i32> = Vec::new();

    for elf in elves {
        let total: i32 = elf.iter().sum::<i32>();
        elves_total.push(total)
    }

    let max_elf_calories = *elves_total.iter().max().unwrap();

    println!("Top elf total: {}", max_elf_calories);
}

pub fn advent_day_1_2() {
    let file_path = "input1.txt";
    let contents = fs::read_to_string(file_path).expect("Should've been able to read file");
    let string_vecs: Vec<&str> = contents.split("\n\n").collect();
    let mut elves: Vec<Vec<i32>> = Vec::new();
    for block in &string_vecs {
        let mut elf: Vec<i32> = Vec::new();
        for calorie in block.split("\n") {
            match calorie.parse::<i32>() {
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

    let mut elves_total: Vec<i32> = Vec::new();

    for elf in elves {
        let total: i32 = elf.iter().sum::<i32>();
        elves_total.push(total)
    }

    elves_total.sort();
    elves_total.reverse();

    println!(
        "Top 3 elves total: {}",
        elves_total[0] + elves_total[1] + elves_total[2]
    );
}

fn main() {
    advent_day_1_1();
    advent_day_1_2();
}
