// Erm What The Block Is This (ew_bit)
use jars::{jar, JarOptionBuilder};
use std::{error::Error};
use serde::Deserialize;
use serde_json;
use std::fs::File;
use std::io::Read;

#[derive(Debug, Deserialize)]
struct ModInfo {
    depends: Depends,
}

#[derive(Debug, Deserialize)]
struct Depends {
    minecraft: String,
}

fn main() {
    let jar_path = "temp";
    let json_path: String; 
    
    match find_fabric_mod_json(jar_path) {
        Ok(file_path) => {
            json_path = file_path;
        }
        Err(err) => {
            eprint!("{}", err);
            return;
        }
    }

    match read_json_file(&json_path) {
        Ok(content) => {

        }
        Err(err) => {
            eprint!("{}", err);
            return;
        }
    }
}

fn find_fabric_mod_json(path: &str) -> Result<String, Box<dyn Error>>  {
    let local_jar = jar(path, JarOptionBuilder::default()).unwrap();
    
    for (file_path, _) in local_jar.files {
        //println!("{}",file_path);
        if file_path == "fabric.mod.json" {
            println!("Found Mod's Config Json File");
            //print!("{}", content);
            return Ok(file_path);
        }
    }

    Err("fabric.mod.json not found".into())
}

fn read_json_file(path: &str) -> Result<ModInfo, Box<dyn Error>> {
    let mut file = File::open(path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let mod_info: ModInfo = serde_json::from_str(&contents)?;

    Ok(mod_info)

    //Err("could not find minecraft file version".into())
}