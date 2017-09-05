use std::fs::File;
use std::collections::HashMap;
use std::io::*;

use toml;


#[derive(Debug, Serialize, Deserialize, Clone)]
struct Config {
    pub trivials_names: HashMap<String, String>,
}


#[allow(dead_code)]
/// Write an example TOML file to the data_trivials.rs file
fn write(mut trivials_rs_file: &File) {
    let mut trivials_names: HashMap<String, String> = HashMap::new();

    // Add example trivial_name
    trivials_names.insert(
        "water".to_owned(),
        "H2O".to_owned()
    );

    // Generate config
    let config = Config { trivials_names: trivials_names };

    // Convert to TOML
    let config_string = toml::to_string(&config).unwrap();

    // Write TOML to file
    trivials_rs_file.write_all(config_string.as_bytes()).ok();
}


#[allow(dead_code)]
/// Reads the data_trivials.toml file, converts it to the appropriate format
/// and writes it to the trivials.rs file
fn read_and_write(mut trivials_toml_file: &File, mut trivials_rs_file: &File) {
    // Read TOML file
    let mut trivials_toml = String::new();
    trivials_toml_file.read_to_string(&mut trivials_toml).ok();

    // Convert to config struct
    let config: Config;
    match toml::from_str(&trivials_toml) {
        Ok(x) => config = x,
        Err(e) => panic!("{:?}", e),
    }

    // Write header to file
    trivials_rs_file.write_all(b"use molecule::Molecule;\n\n").ok();

    trivials_rs_file.write_all(b"lazy_static! {\n").ok();

    // Convert items from TOML file to RS syntax
    for (capsname, trivial) in config.trivials_names.clone().into_iter() {
        let rust_trivial_name = format!(
            "\n    pub static ref {capsname}: Molecule = Molecule::from_string(\"{symbol}\".to_owned()).unwrap();",
            capsname = capsname,
            symbol = trivial,
        );

        // Append to file
        trivials_rs_file.write_all(rust_trivial_name.as_bytes()).ok();
    }

    trivials_rs_file
        .write_all(b"\n\n    pub static ref ALL_TRIVIALS: Vec<(&'static str, &'static Molecule)> = vec! {\n")
        .ok();

    for (i, (capsname, _)) in config.trivials_names.iter().enumerate() {
        if i > 0 {
            trivials_rs_file.write_all(b",\n").ok();
        }
        trivials_rs_file.write_all(format!("        (\"{0}\", &{0})", capsname).as_bytes()).ok();
    }

    trivials_rs_file.write_all(b"\n    };\n").ok();

    trivials_rs_file.write_all(b"}\n").ok();
}


pub fn trivial_main() {
    let mut trivials_toml_file = File::open("src/data_trivials.toml").unwrap();
    let mut trivials_rs_file = File::create("src/data_trivials.rs").unwrap();

    // NOTE: For debugging only:
    // write(&mut trivials_rs_file);

    read_and_write(&mut trivials_toml_file, &mut trivials_rs_file);
}
