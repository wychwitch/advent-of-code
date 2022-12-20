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
        "Top 3 elves total: {}",
        elf_calorie_totals[0] + elf_calorie_totals[1] + elf_calorie_totals[2]
    );
}

fn main() {
    advent_day_1();
}
