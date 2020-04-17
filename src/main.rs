use std::{env, fs, io, path};

#[derive(Debug)]
struct Project {
    parent: String,
    pen_name: String
}

impl Project {
    pub fn get_path(&self) -> String {
        let mut path: String = String::from("./pens/");
        path.push_str(&self.parent.as_str());
        path.push_str(&"/");
        path.push_str(&self.pen_name.as_str());
        path
    }
}

fn main() {

    let scaffold_dir: String = String::from("./scaffold/");

    // get the details of the new pen
    let project_details = get_args();

    // create the new directory
    let create = create_pen_directory(project_details.get_path());

    match create {
        Ok(()) => println!("Successfully created directory!"),
        Err(e) => println!("{:?}", e)
    }

    // get the file paths to be copied into the new directory
    let scaffold_files = get_scaffold_files(scaffold_dir);

    copy_files(scaffold_files, project_details.get_path());
}

// Get a list of files from the scaffold directory
fn get_scaffold_files(path: String) -> Vec<String> {

    let mut files: Vec<String> = Vec::new();

    let mut paths: Vec<path::PathBuf> = Vec::new();

    for entry in fs::read_dir(path).unwrap() {
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

// Function to determine the path to scaffold the files to
fn get_args() -> Project {
    // get the args
    let args: Vec<String> = env::args().collect();

    assert_eq!(args.len(), 2);

    // the directory should be the second arg
    let path = &args[1];

    // split on the slash
    let paths: Vec<&str> = path.split('/').collect();

    // only allow 2 levels deep. This means that projects can be flattly grouped under an umbrella directory
    assert_eq!(paths.len(), 2);

    let project: Project = Project { 
        parent: String::from(paths[0]),
        pen_name: String::from(paths[1])
    };

    // can do this with the debug trait
    println!("{:?}", project);

    // return the populated project struct
    project
}

fn create_pen_directory(path: String) -> Result<(), io::Error> {
    let p = path::Path::new(&path);
    println!("creating directory: {:?}", p.to_str().unwrap());
    fs::create_dir_all(&path)?;
    Ok(())
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
        match fs::copy(from, to) {
            Ok(_) => println!("=> Created {}", to.to_str().unwrap()),
            Err(e) => println!("Uh-oh: {:?}", e)
        }
    }
}