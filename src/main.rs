use std::{env, io};
use std::fs::{File, OpenOptions};
use std::io::{Read, Write};

fn help() {
    println!("Usage: game_backlog <command>");
    println!("Commands: help");
    println!("          add \"game name\"");
    println!("          remove \"game name\"");
    println!("          show");
}

fn add() -> io::Result<()> {
    let game: String = env::args().nth(2).expect("game to remove was not specified, no game was removed").to_lowercase();
    let mut f = OpenOptions::new()
                    .write(true)
                    .read(true)
                    .open("backlog.txt").expect("Error: could not open file backlog.txt");
    let mut game_list = String::new();
    
    f.read_to_string(&mut game_list).expect("Error reading file to string");

    if game_is_in_list(&game, &game_list) {
        println!("game is already in backlog");
        return Ok(());
    } 
    
    if let Err(e) = writeln!(f, "{}", game) {
        eprintln!("Couldn't write to file: {}", e);
    }

    Ok(())
}

fn show() {
    let mut game_list = String::new();
    let file_open = File::options()
        .read(true)
        .open("backlog.txt");
    match file_open {
        Ok(mut f) => {
            f.read_to_string(&mut game_list).expect("Error reading from backlog.txt");
            println!("Current Backlog:");
            print!("{}",game_list);
        },
        Err(e) => println!("Error opening backlog.txt: {}",e.to_string()),
    }
}

fn game_is_in_list(game: &String, game_list: &String) -> bool {
    for line in game_list.lines() {
        if game.as_str().eq(line) {
            return true;
        }
    }
    false
}

fn remove() -> io::Result<()> {
    let game: String = env::args().nth(2).expect("game to remove was not specified, no game was removed").to_lowercase();

    let mut f = File::open("backlog.txt").expect("Error: could not open file backlog.txt");
    let mut game_list = String::new();
    
    f.read_to_string(&mut game_list).expect("Error reading file to string");
    
    if !game_is_in_list(&game, &game_list) {
        println!("game is not in backlog");
        Ok(())
    } else {
        //remove game
        let new_buffer = game_list_removed(&game, &game_list);

        let mut f = File::create("backlog.txt").expect("Error: could not create file backlog.txt");
        f.write_all(new_buffer.as_bytes()).expect("Could not rewrite to backlog.txt");
        println!("game successfully removed from backlog");
        Ok(())
    }
}

fn game_list_removed(game_to_remove: &String, game_list: &String) -> String {
    let mut edited_list = String::new();
    
    for line in game_list.lines() {
        if game_to_remove != line {
            edited_list.push_str(line);
            edited_list.push('\n');
        } 
    }
    edited_list
}

fn main() {
    let command: String = env::args().nth(1).expect("command required, type game_backlog help for command list");
    
    match command.as_str() {
        "help" => help(),
        "add" => {
            let result = add();
            match result {
                Ok(()) => {},
                Err(e) => println!("Error, could not add game to the backlog: {}", e.to_string()),
            }
            show();
        },
        "remove" => {
            let result = remove();
            match result {
                Ok(()) => {},
                Err(e) => println!("Error, could not remove game from backlog: {}", e.to_string()),
            }
            show();
        },
        "show" => show(),
        _ => println!("command {} is not a supported command",command),
    }
}