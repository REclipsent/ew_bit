// Erm What The Block Is This (ew_bit)
use jars::{jar, JarOptionBuilder};
use std::{error::Error};
use serde_json::Value;

fn main() {
    let jar_path = "C:\\Data\\ewbit\\Jade-1.20.2-fabric-12.3.0.jar";
        
    match find_fabric_mod_json(jar_path) {
        Ok(version) => {
            println!("Mod is for Minecraft Version: {}", version)
        }
        Err(err) => {
            eprint!("{}", err);
            return;
        }
    }
}

fn find_fabric_mod_json(path: &str) -> Result<String, Box<dyn Error>>  {
    let local_jar = jar(path, JarOptionBuilder::default()).unwrap();

    for (file_path, content) in local_jar.files {
        if file_path == "fabric.mod.json" {
            println!("Found Mod's Config Json File");
            let content_string = String::from_utf8(content)?;
            let json_value: Value = serde_json::from_str(&content_string)?;

            println!("{}", content_string);

            let version = if let Some(version) = json_value["depends"]["minecraft"].as_str() {
                version.to_owned()
            } else {
                return Err("Minecraft field not found".into());
            };
            //print!("{}", content_string);
            return Ok(version);
        }
    }

    Err("fabric.mod.json not found".into())
}