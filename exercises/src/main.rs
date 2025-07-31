use std::fs;
use std::io::{self, Write};
use std::process::Command;



fn main() {
    let bin_dir = "src/bin";
    let entries = fs::read_dir(bin_dir).expect("Konnte src/bin nicht lesen");

    let mut bin_names = Vec::new();

    for entry in entries {
        let entry = entry.expect("Fehler beim Lesen des Eintrags");
        let path = entry.path();

        if path.is_file() {
            if let Some(file_name) = path.file_stem() {
                if let Some(name) = file_name.to_str() {
                    bin_names.push(name.to_string());
                }
            }
        }
    }

    if bin_names.is_empty() {
        println!("Keine Binaries in src/bin gefunden.");
        return;
    }

    println!("Welche Binary möchtest du starten?");
    for (i, name) in bin_names.iter().enumerate() {
        println!("{}: {}", i + 1, name);
    }

    print!("> ");
    io::stdout().flush().unwrap();

    let mut choice = String::new();
    io::stdin().read_line(&mut choice).unwrap();
    let choice = choice.trim().parse::<usize>();

    match choice {
        Ok(index) if index > 0 && index <= bin_names.len() => {
            let bin_name = &bin_names[index - 1];
            println!("Starte `{}`...", bin_name);

            let status = Command::new("cargo")
                .args(["run", "--bin", bin_name])
                .status()
                .expect("Fehler beim Ausführen");

            if !status.success() {
                println!("Binary `{}` konnte nicht ausgeführt werden.", bin_name);
            }
        }
        _ => println!("Ungültige Auswahl."),
    }
}
