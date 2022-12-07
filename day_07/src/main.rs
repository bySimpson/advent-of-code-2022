use std::{
    fs::File,
    io::{self, BufRead, BufReader},
    ops::Deref,
};

use std::cell::RefCell;
use std::rc::Rc;

use clap::Parser;

use crate::cli::Folder;
mod cli;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long, default_value = "./challenges/challenge_07.txt")]
    path: String,
}

fn main() -> io::Result<()> {
    challenge_1_2().unwrap();
    Ok(())
}

fn challenge_1_2() -> io::Result<()> {
    let args = Args::parse();
    // File hosts must exist in current path before this produces output
    let file = File::open(args.path)?;
    let reader = BufReader::new(file);
    let root_folder = Rc::new(RefCell::new(Folder::new(String::from("/"))));
    let mut current_folder: Rc<RefCell<Folder>> = Rc::clone(&root_folder);

    for line in reader.lines() {
        let c_line = line?;
        let split_instruction = c_line.split(" ").collect::<Vec<&str>>();
        if c_line.starts_with("$ ") {
            match split_instruction[1] {
                "cd" => {
                    if split_instruction[2] == "/" {
                        current_folder = Rc::clone(&root_folder);
                    } else if split_instruction[2] == ".." {
                        let current_clone = Rc::clone(&current_folder);
                        current_folder = current_clone.deref().borrow().parent.upgrade().unwrap();
                    } else {
                        let new_folder =
                            Rc::new(RefCell::new(Folder::new(split_instruction[2].to_string())));
                        new_folder.deref().borrow_mut().parent =
                            Rc::downgrade(&Rc::clone(&current_folder));

                        current_folder
                            .deref()
                            .borrow_mut()
                            .folders
                            .push(Rc::clone(&new_folder));
                        current_folder = Rc::clone(&new_folder);
                    }
                }
                "ls" => (),
                _ => panic!("Unknown instruction {}", split_instruction[2]),
            }
        } else {
            //println!("{}", split_instruction[0]);
            match split_instruction[0].parse::<i32>() {
                Ok(size) => {
                    // file
                    current_folder
                        .deref()
                        .borrow_mut()
                        .add_file(split_instruction[1].to_string(), size);
                }
                Err(_) => {
                    // folder
                }
            }
        }
    }
    root_folder.deref().borrow_mut().calculate_size();
    let points_1 = root_folder
        .deref()
        .borrow()
        .get_folders_smaller_then(100000)
        .iter()
        .sum::<i32>();

    let points_2 = root_folder
        .deref()
        .borrow()
        .get_smallest_directory(70000000);

    println!("Points 1:\t{}", points_1);
    println!("Points 2:\t{}", points_2);
    Ok(())
}
