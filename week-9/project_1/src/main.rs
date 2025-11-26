use std::fs::File;
use std::io::Write;

fn main() {
    let lager = [
        "33 Export",
        "Desperados",
        "Goldberg",
        "Gulder",
        "Heineken",
        "Star",
    ];

    let stout = [
        "Legend",
        "Turbo King",
        "Williams"
    ];

    let non_alcoholic = [
    "Maltina",
    "Amstel Malta",
    "Malta Gold",
    "Fayrouz"
    ];

    let mut file = File::create("nigerian_breweries.txt").unwrap();

    file.write_all(b"NIGERIAN BREWERIES PRODUCT CATEGORIES\n\n").unwrap();

    file.write_all(b"Lager Drinks:\n").unwrap();
    for item in lager {
        file.write_all(format!("- {}\n", item).as_bytes()).unwrap();
    }

    file.write_all(b"\nStout Drinks:\n").unwrap();
    for item in stout {
        file.write_all(format!("- {}\n", item).as_bytes()).unwrap();
    }

    file.write_all(b"\nNon-Alcoholic Drinks:\n").unwrap();
    for item in non_alcoholic {
        file.write_all(format!("- {}\n", item).as_bytes()).unwrap();
    }

    println!("File created successfully!");
}
