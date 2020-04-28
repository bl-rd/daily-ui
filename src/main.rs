use serde::{Deserialize, Serialize};
use std::env::{args, set_current_dir};
use std::fs::{read_to_string, write};
use std::time::{SystemTime, UNIX_EPOCH};
use std::process::Command;
use std::path::Path;

// use std::error::Error;
// use std::fs::File;
// use std::io::BufReader;
// use std::path::Path;

#[derive(Deserialize, Serialize, Debug)]
struct Project {
    name: String,
    pens: Vec<Pen>
}

#[derive(Deserialize, Serialize, Debug)]
struct Pen {
    path: String,
    text: String,
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

    let args = get_args();

    let mut config_data = get_config_data("config.json".to_owned()).unwrap();

    let template_directory = args.directory.clone();
    let template_collection = args.parent.clone();

    let pen: Pen = Pen {
        path: args.path,
        text: args.name,
        timestamp: get_timestamp()
    };
    
    let index = get_project_index(&config_data, args.parent);

    config_data[index].pens.push(pen);

    let serialised = serde_json::to_string_pretty(&config_data).unwrap();

    build_wasm_template(template_collection, template_directory);

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

    // let flavour: String = args
    //     .into_iter()
    //     .find(|x| x.contains("--flavour"))
    //     .unwrap()
    //     .split('=')
    //     .collect();
    // println!("FLAVOUR {:?}", flavour);

    // let name = String::from("A friendly name");
    // let path = String::from("/blah/blah");

    // ProjectType { flavour, name, path }

    // // the directory should be the second arg
    // let path = &args[1];

    // // split on the slash
    // let paths: Vec<&str> = path.split('/').collect();

    // // only allow 2 levels deep. This means that projects can be flattly grouped under an umbrella directory
    // assert_eq!(paths.len(), 2);

    // let project: Project = Project { 
    //     parent: String::from(paths[0]),
    //     pen_name: String::from(paths[1])
    // };

    // // can do this with the debug trait
    // println!("{:?}", project);

    // // return the populated project struct
    // project
}

fn build_wasm_template(collection: String, project: String) {
    let proj = format!("./pens/{}/", collection);
    let path = Path::new(&proj);
    assert!(set_current_dir(&path).is_ok());
    // // Command::new("mkdir")
    // //     .arg("-p")
    // //     .arg(proj)
    // //     .output()
    // //     .expect("Failed to write directory");
    
    // println!("{}", proj);

    // Command::new("cd")
    //     .arg(proj)
    //     .output()
    //     .expect("Unable to cd to parent directory");
    
    Command::new("wasm-pack")
        .arg("new")
        .arg(project)
        .output()
        .expect("Unable to scaffold wasm-pack template");
    
    // // Command::new("cd")
    // //     .arg("../")
    // //     .output()
    // //     .expect("Unable to cd back to root level");
}

// ---------------------------------------------------------

// use std::{env, fs, io, path};
// use serde::{Deserialize, Serialize};
// use serde_json::Result;

// #[derive(Serialize, Deserialize)]
// struct Parent {
//     name: String,
//     pens: Vec<()>
// }

// fn create_entry() -> Result<()> {
//     let p = vec![(), ()];

//     let entry = Parent {
//         name: "test".to_owned(),
//         pens: p
//     };

//     // serialze as a JSON string
//     let j = serde_json::to_string(&entry)?;

//     println!("{}", j);

//     Ok(())
// }

// fn main() -> Result<()> {
//     create_entry()
// }

// --------------------------------------------------------------

// #[derive(Debug)]
// struct Project {
//     parent: String,
//     pen_name: String
// }

// impl Project {
//     pub fn get_path(&self) -> String {
//         let mut path: String = String::from("./pens/");
//         path.push_str(&self.parent.as_str());
//         path.push_str(&"/");
//         path.push_str(&self.pen_name.as_str());
//         path
//     }
// }

// fn main() {

//     let scaffold_dir: String = String::from("./scaffold/");

//     // get the details of the new pen
//     let project_details = get_args();

//     // create the new directory
//     let create = create_pen_directory(project_details.get_path());

//     match create {
//         Ok(()) => println!("Successfully created directory!"),
//         Err(e) => println!("{:?}", e)
//     }

//     // get the file paths to be copied into the new directory
//     let scaffold_files = get_scaffold_files(scaffold_dir);

//     copy_files(scaffold_files, project_details.get_path());
// }

// // Get a list of files from the scaffold directory
// fn get_scaffold_files(path: String) -> Vec<String> {

//     let mut files: Vec<String> = Vec::new();

//     let mut paths: Vec<path::PathBuf> = Vec::new();

//     for entry in fs::read_dir(path).unwrap() {
//         let entry = entry.unwrap();
//         let path = entry.path();
//         // don't include directories for now!
//         if !path.is_dir() {
//             files.push(String::from(path.to_str().unwrap()));
//             paths.push(path);
//         }
//     }

//     files

//     // https://doc.rust-lang.org/std/fs/fn.copy.html
// }

// // Function to determine the path to scaffold the files to
// fn get_args() -> Project {
//     // get the args
//     let args: Vec<String> = env::args().collect();

//     assert_eq!(args.len(), 2);

//     // the directory should be the second arg
//     let path = &args[1];

//     // split on the slash
//     let paths: Vec<&str> = path.split('/').collect();

//     // only allow 2 levels deep. This means that projects can be flattly grouped under an umbrella directory
//     assert_eq!(paths.len(), 2);

//     let project: Project = Project { 
//         parent: String::from(paths[0]),
//         pen_name: String::from(paths[1])
//     };

//     // can do this with the debug trait
//     println!("{:?}", project);

//     // return the populated project struct
//     project
// }

// fn create_pen_directory(path: String) -> Result<(), io::Error> {
//     let p = path::Path::new(&path);
//     println!("creating directory: {:?}", p.to_str().unwrap());
//     fs::create_dir_all(&path)?;
//     Ok(())
// }

// fn copy_files(files: Vec<String>, directory: String) {
//     println!("DIRECTORY {}", directory);
//     for file in files {
//         let from = path::Path::new(&file);
//         let mut x = directory.clone();
//         x.push_str(&"/");
//         let file_name = from.file_name().unwrap().to_str().unwrap();
//         x.push_str(&file_name);
//         let to = path::Path::new(&x);
//         match fs::copy(from, to) {
//             Ok(_) => println!("=> Created {}", to.to_str().unwrap()),
//             Err(e) => println!("Uh-oh: {:?}", e)
//         }
//     }
// }