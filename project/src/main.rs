use std::collections::HashMap;
use std::io::{self, Write};

fn main() {
    let mut notes = HashMap::new();

    loop {
        print_menu();

        let mut command = String::new();
        io::stdin().read_line(&mut command).expect("Failed to read line");
        let command = command.trim();

        match command {
            "1" => add_note(&mut notes),
            "2" => delete_note(&mut notes),
            "3" => list_notes(&notes),
            "4" => mark_done(&mut notes),
            "q" => break,
            _ => println!("Invalid command. Please try again."),
        }
    }
}

fn print_menu() {
    println!("**** NOTES APP MENU ****");
    println!("1. Add Note");
    println!("2. Delete Note");
    println!("3. List Notes");
    println!("4. Mark as Done");
    println!("q. Quit");
    print!("Enter your choice: ");
    io::stdout().flush().expect("Failed to flush stdout");
}

fn add_note(notes: &mut HashMap<String, bool>) {
    println!("Enter your note:");
    let mut note = String::new();
    io::stdin().read_line(&mut note).expect("Failed to read line");
    notes.insert(note.trim().to_string(), false);
    println!("Note added.");
}

fn delete_note(notes: &mut HashMap<String, bool>) {
    println!("Enter the note to delete:");
    let mut note = String::new();
    io::stdin().read_line(&mut note).expect("Failed to read line");
    if notes.remove(&note.trim().to_string()).is_some() {
        println!("Note deleted.");
    } else {
        println!("Note not found.");
    }
}

fn list_notes(notes: &HashMap<String, bool>) {
    println!("**** YOUR NOTES ****");
    for (note, done) in notes.iter() {
        if *done {
            println!("[X] {}", note);
        } else {
            println!("[ ] {}", note);
        }
    }
}

fn mark_done(notes: &mut HashMap<String, bool>) {
    println!("Enter the note to mark as done:");
    let mut note = String::new();
    io::stdin().read_line(&mut note).expect("Failed to read line");
    if let Some(done) = notes.get_mut(&note.trim().to_string()) {
        *done = true;
        println!("Note marked as done.");
    } else {
        println!("Note not found.");
    }
}