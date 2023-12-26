use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::cmp::max;

#[derive(Debug)]
struct Game {
    game_num: u8,
    samples: Vec<Sample>,
}

impl Game {
    fn min_sample(&self) -> Sample {
        Sample {
            red: self.samples.iter().map(|s| s.red).fold(0, |acc, val| max(acc, val)),
            green: self.samples.iter().map(|s| s.green).fold(0, |acc, val| max(acc, val)),
            blue: self.samples.iter().map(|s| s.blue).fold(0, |acc, val| max(acc, val)),
        }
    }
}

#[derive(Debug)]
struct Sample {
    red: u8,
    green: u8,
    blue: u8,
}

impl Sample {
    fn is_subset_of(&self, super_sample: &Sample) -> bool {
        self.red <= super_sample.red
            && self.green <= super_sample.green
            && self.blue <= super_sample.blue
    }

    fn power(&self) -> u32 {
        self.red as u32 * self.green as u32 * self.blue as u32
    }
}

fn read_input_lines() -> Vec<String> {
    let file = File::open("input.txt").expect("Failed to open file");
    BufReader::new(file).lines().map(|e| e.unwrap()).collect()
}

fn parse_line(line: &str) -> Game {
    let game_num: u8;
    let raw_samples: Vec<&str>;
    {
        let mut first_split = line[5..].split(": ");
        game_num = first_split.next().unwrap().parse().unwrap();
        raw_samples = first_split.next().unwrap().split("; ").collect();
    };

    let mut samples: Vec<Sample> = vec![];
    for raw_sample in raw_samples.iter() {
        let mut sample = Sample { red: 0, green: 0, blue: 0 };
        for sample_value in raw_sample.split(", ") {
            let mut sample_val_split = sample_value.split(" ");
            let sample_val = sample_val_split.next().unwrap().parse().unwrap();
            match sample_val_split.next().unwrap().to_lowercase().as_str() {
                "red" => sample = Sample { red: sample_val, ..sample},
                "green" => sample = Sample { green: sample_val, ..sample},
                "blue" => sample = Sample { blue: sample_val, ..sample},
                _ => panic!("unknown sample color")
            };
        }
        samples.insert(0, sample);
    };
    Game { game_num, samples }
}

fn part_one() -> std::io::Result<()> {
    let contents = read_input_lines();

    let games: Vec<Game> = contents
        .iter()
        .map(|line| parse_line(&line))
        .collect();

    let super_sample = Sample {
        red: 12,
        green: 13,
        blue: 14,
    };

    // println!("{:#?}", games);
    let mut game_num_sum: u16 = 0;
    for game in games.iter() {
        let mut is_subset: bool = true;
        for sample in game.samples.iter() {
            if !sample.is_subset_of(&super_sample) {
                is_subset = false;
                break;
            }
        }
        // println!("{} {}", game.game_num, is_subset);
        if is_subset {
            game_num_sum += game.game_num as u16;
        }
    }
    println!("{}", game_num_sum);
    Ok(())
}

fn part_two() -> std::io::Result<()> {
    let contents = read_input_lines();

    let games: Vec<Game> = contents
        .iter()
        .map(|line| parse_line(&line))
        .collect();

    let mut game_power_sum: u32 = 0;
    for game in games.iter() {
        // println!("{} {:?} {:?}", game.game_num, game.min_sample(), game.min_sample().power());
        game_power_sum += game.min_sample().power();
    }
    println!("{}", game_power_sum);
    Ok(())
}

fn main() -> std::io::Result<()> {
    part_one().unwrap();
    part_two().unwrap();
    Ok(())
}
