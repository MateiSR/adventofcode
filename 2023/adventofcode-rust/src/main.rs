use std::env;
use std::fs;
use std::thread::current;

#[derive(Clone, Copy)]
struct Colors {
    red: isize,
    green: isize,
    blue: isize,
}
struct Game {
    id: usize,
    color_tables: Vec<Colors>,
}

fn assign_colors_to_table(mut color_table:Colors, number: isize, color: &str) -> Colors {
    match color {
        "red"=>color_table.red = number,
        "green"=>color_table.green = number,
        "blue"=>color_table.blue = number,
        _=>println!("Invalid color"),
    }

    color_table
}

fn process_color_list(color_list: Vec<&str>) -> Colors {
    let mut color_table = Colors {
        red: 0,
        green: 0,
        blue: 0,
    };

    for mut color in color_list {
        color = color.trim();
        //println!("{}", String::from(color));
        let space_index: usize = color.find(' ').unwrap_or(0);
        let ascii_number: &str = &color[..space_index];
        let color: &str = &color[space_index+1..];
        match ascii_number.parse::<isize>() {
            Ok(number) => {
                println!("Number: {}, Color: {}", number, String::from(color));
                color_table = assign_colors_to_table(color_table, number, color);
                //print_color_table(color_table);
            }
            Err(e) => {
                println!("Error parsing number, check input file!");
            }
        }
    }

    color_table
}
fn process_input_file(contents: String) -> Vec<Game> {
    let mut i: usize = 0;
    let mut games:Vec<Game>  = Vec::new();

    for line in contents.lines() {

        i = i + 1;

        let mut current_game = Game {
            id: i,
            color_tables: Vec::new(),
        };

        println!("{}", String::from(line));
        match line.find(':') {

            Some(index) => {
                println!("Index: {index}");
                let game_list: String = line.chars().skip(index + 2).collect();
                //println!("{}", String::from(game_list));
                let game_list_parts: Vec<&str> = game_list.split(';').collect();
                for mut game in game_list_parts {
                    game = game.trim();
                    //println!("{}", String::from(game));
                    let color_list: Vec<&str> = game.split(',').collect();
                    let current_color_table: Colors = process_color_list(color_list);
                    current_game.color_tables.push(current_color_table);
                    println!("---");
                }
            }
            None => println!("Invalid line found, check input file!"),
        }
        games.push(current_game);
    }
    games
}

fn is_game_possible(game: Game, bag: Colors) -> bool {

    for color_table in game.color_tables {
        //print_color_table(color_table);
        if (color_table.red > bag.red || color_table.green > bag.green || color_table.blue > bag.blue) {
            return false
        }
    }

    return true
}

fn process_games(games: Vec<Game>, bag: Colors) -> usize {
    let mut sum:usize = 0;
    for game in games {
        let game_id = game.id;
        if (is_game_possible(game, bag)) {
            println!("Adding game with ID: {}", game_id);
            sum += game_id;
        }
    }

    sum
}

fn get_bag_minimum(game: Game) -> Colors {
    let mut min_bag = Colors {
        red: 1,
        green: 1,
        blue : 1,
    };

    let mut initialized: bool = false;

    for color_table in game.color_tables {
        if (!initialized) {
            min_bag = color_table;
            initialized = true;
        }
        if (color_table.red > min_bag.red) {min_bag.red = color_table.red;}
        if (color_table.green > min_bag.green) {min_bag.green = color_table.green;}
        if (color_table.blue > min_bag.blue) {min_bag.blue = color_table.blue;}
    }

    min_bag
}

fn get_power(games: Vec<Game>) -> isize {
    let mut final_power:isize = 1;

    for game in games {
        let game_min_bag: Colors = get_bag_minimum(game);
        final_power += game_min_bag.red * game_min_bag.green * game_min_bag.blue
    }

    final_power
}

fn main() {
    const FILE: &str = "./in.txt";
    let content: String = fs::read_to_string(FILE).
        expect("Input file couldn't be opened");
    // println!("{contents}");

    // Predefined contents of bag in Part 1 of the problem
    let bag = Colors {
        red: 12,
        green: 13,
        blue: 14,
    };

    let games: Vec<Game> = process_input_file(content);
    let final_sum: usize = process_games(games, bag);
    println!("Final sum is: {}", final_sum);

    let content: String = fs::read_to_string(FILE).
        expect("Input file couldn't be opened");
    let games: Vec<Game> = process_input_file(content);
    let final_power: isize = get_power(games);
    println!("Final power is: {}", final_power - 1);

}

fn print_color_table(color_table: Colors) -> () {
    println!("-START OUT-\nRed: {}, Green: {}, Blue: {}\n-STOP OUT-", color_table.red, color_table.green, color_table.blue);
}