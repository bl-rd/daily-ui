use serde::{Deserialize, Serialize};
use std::env::{args, current_dir, set_current_dir};
use std::fs::{copy, read_to_string, read_dir, create_dir_all, write};
use std::time::{SystemTime, UNIX_EPOCH};
use std::process::Command;
use std::path;
use std::path::Path;

#[derive(Deserialize, Serialize, Debug)]
struct Project {
    name: String,
    pens: Vec<Pen>
}

#[derive(Deserialize, Serialize, Debug)]
struct Pen {
    path: String,
    text: String,
    flavour: String,
    timestamp: u64
}

#[derive(Debug)]
struct ProjectType {
    parent: String,
    directory: String,
    flavour: String,
    path: String,
    name: String
}

fn main() {

    let dir = current_dir();

    let args = get_args();

    let mut config_data = get_config_data("config.json".to_owned()).unwrap();

    let template_directory = args.directory.clone();
    let template_collection = args.parent.clone();
    let template_flavour = args.flavour.clone();

    let pen: Pen = Pen {
        path: args.path,
        text: args.name,
        flavour: args.flavour,
        timestamp: get_timestamp()
    };
    
    let index = get_project_index(&config_data, args.parent);

    config_data[index].pens.push(pen);

    let serialised = serde_json::to_string_pretty(&config_data).unwrap();

    match template_flavour.as_str() {
        "rust" => build_wasm_template(template_collection, template_directory),
        _ => build_js_template(template_collection, template_directory).unwrap()
    };

    // go back to the project root and update the config file
    assert!(set_current_dir(&dir.unwrap().as_path()).is_ok());
    write("config.json", serialised).expect("Unable to write file");
}

// Get the deserialised config file
fn get_config_data(path: String) -> Result<Vec::<Project>, serde_json::error::Error> {

    let config = read_to_string(path).unwrap();

    let projects: Vec<Project> = serde_json::from_str(config.as_str())?;

    Ok(projects)
}

fn get_project_index(config: &Vec<Project>, project_name: String) -> usize {
    let index = config
        .into_iter()
        .position(|p| p.name == project_name.as_str());

    match index {
        Some(i) => i,
        None => panic!(format!("No project with the name \"{}\" exists!", project_name))
    }
}

fn get_timestamp() -> u64 {
    SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs()
}

// Function to determine the path to scaffold the files to
fn get_args() -> ProjectType {

    // get the args
    let args: Vec<String> = args().collect();

    assert_eq!(args.len(), 4);

    let flavour = &args[1];
    let path = &args[2];
    let name = &args[3];

    // only allow 2 levels deep. This means that projects can be flattly grouped under an umbrella directory
    let paths: Vec<&str> = path.split('/').collect();
    assert_eq!(paths.len(), 2);
    let parent = paths[0];
    let directory = paths[1];

    ProjectType { 
        parent: parent.to_owned(),
        directory: directory.to_owned(),
        flavour: flavour.to_owned(),
        name: name.to_owned(),
        path: path.to_owned()
    }
}

fn build_wasm_template(collection: String, project: String) {
    let proj = format!("./pens/{}/", collection);

    // set the current directory so we can run the scaffolding
    let path = Path::new(&proj);
    assert!(set_current_dir(&path).is_ok());
    
    Command::new("npm")
        .arg("init")
        .arg("rust-webpack")
        .arg(project)
        .output()
        .expect("Unable to scaffold wasm-pack template");
}

fn build_js_template(collection: String, project: String) -> Result<(), std::io::Error> {
    let proj_dir = format!("./pens/{}/{}", collection, project);
    let scaffold_dir: String = String::from("./scaffold/");

    let path = Path::new(&proj_dir);
    create_dir_all(&path)?;

    let files = get_scaffold_files(scaffold_dir);

    copy_files(files, proj_dir);

    Ok(())
}

fn get_scaffold_files(path: String) -> Vec<String> {

    let mut files: Vec<String> = Vec::new();

    let mut paths: Vec<path::PathBuf> = Vec::new();

    for entry in read_dir(path).unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();
        // don't include directories for now!
        if !path.is_dir() {
            files.push(String::from(path.to_str().unwrap()));
            paths.push(path);
        }
    }

    files

    // https://doc.rust-lang.org/std/fs/fn.copy.html
}

fn copy_files(files: Vec<String>, directory: String) {
    println!("DIRECTORY {}", directory);
    for file in files {
        let from = path::Path::new(&file);
        let mut x = directory.clone();
        x.push_str(&"/");
        let file_name = from.file_name().unwrap().to_str().unwrap();
        x.push_str(&file_name);
        let to = path::Path::new(&x);
        match copy(from, to) {
            Ok(_) => println!("=> Created {}", to.to_str().unwrap()),
            Err(e) => println!("Uh-oh: {:?}", e)
        }
    }
}