use std::env;
use std::fs;

fn main() {
    if env::args().len() < 3 {
        eprintln!("Usage: {} <file_name> <name>", env::args().nth(0).unwrap());
        return;
    }

    let name = env::args().nth(2).unwrap();
    println!("Hello, {}!", name);
    
    let file_name = env::args().nth(1).unwrap_or_else(|| "names.txt".to_string());

    let list_names = fs::read_to_string(&file_name)
        .expect("Failed to read names.txt");

    for line in list_names.lines() {
        if line == name {
            println!("{} is in the list!", name);
            return;
        }
    }
    println!("{} is not in the list.", name);

    // Append it to the file
    let mut file = fs::OpenOptions::new()
        .append(true)
        .create(true)
        .open(&file_name)
        .expect("Failed to open names.txt");
    use std::io::Write;
    writeln!(file, "{}", name)
        .expect("Failed to write to names.txt");
    println!("{} has been added to the list.", name);
    
}
