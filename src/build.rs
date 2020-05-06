use serde::{Deserialize, Serialize};
use std::fs::{write};

#[derive(Deserialize, Serialize, Debug)]
struct Project {
    name: String,
    text: String,
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

pub fn run() {
  let data = get_config_data(String::from("config.json"));

  let projects = match data {
    Ok(d) => d,
    Err(_) => panic!("Unable to get config data")
  };

  build_root_page();

  build_landing_page(&projects);

  for project in projects.iter() {
    build_project_page(project);
  }

  // for p in projects.iter() {
  //   for pen in p.pens.iter() {
  //     list_markup.push_str(build_pen_list_markup(&list_markup_template, &pen.path, &pen.text).as_str());
  //   }
  // }

  // println!("{}", list_markup);
  // write("index.html", list_markup).expect("unable to create index file");
}

// Get the deserialised config file
fn get_config_data(path: String) -> Result<Vec::<Project>, serde_json::error::Error> {

  let config = std::fs::read_to_string(path).unwrap();

  let projects: Vec<Project> = serde_json::from_str(config.as_str())?;

  Ok(projects)
}

// Load a HTML template as a String
// from the file system
fn get_html_template(path: String) -> String {
  let markup = std::fs::read_to_string(path).unwrap();

  String::from(markup)
}

// Build the <li> HTML for project pages.
// This might be more reusable if it takes separate href and text arguments
fn build_pen_list_markup(html: &String, href: &String, text: &String) -> String {
  let link = format!("/pens/{}", href);
  let mut markup = html.replace("{{ HREF }}", link.as_str());
  markup = markup.replace("{{ TEXT }}", text.as_str());
  
  markup
}

// Loop through the top level
// projects and build the pens index.html page
fn build_landing_page(data: &Vec<Project>) {

  let mut page_markup_template = get_html_template(String::from("html/templates/list-page.html"));
  page_markup_template = page_markup_template.replace("{{ PAGE_TITLE_TEMPLATE }}", "Pens page");

  let list_markup_template = get_html_template(String::from("html/partials/pen-link.html"));
  let mut list_markup = String::new();

  for project in data {
    list_markup.push_str(build_pen_list_markup(&list_markup_template, &project.name, &project.text).as_str());
  }

  page_markup_template = page_markup_template.replace("{{ LIST_TEMPLATE }}", list_markup.as_str());

  // println!("{}", page_markup_template);
  write("pens/index.html", page_markup_template).expect("unable to create index file");
}

fn build_project_page(project: &Project) {
  let mut page_template = get_html_template(String::from("html/templates/list-page.html"));
  page_template = page_template.replace("{{ PAGE_TITLE_TEMPLATE }}", project.text.as_str());

  let list_markup_template = get_html_template(String::from("html/partials/pen-link.html"));
  let mut list_markup = String::new();

  for pen in project.pens.iter() {
    // list_markup.push_str(build_pen_list_markup(&list_markup_template, &pen.path, &pen.text).as_str());
    list_markup.push_str(build_pen_list_markup(&list_markup_template, &pen.path, &pen.text).as_str());
  }

  page_template = page_template.replace("{{ LIST_TEMPLATE }}", list_markup.as_str());
  
  write(format!("pens/{}/index.html", project.name), page_template).expect("unable to create project page");
}

fn build_root_page() {
  let markup_template = get_html_template(String::from("html/templates/root.html"));

  // don't need to actually do any transformation currently...
  write("index.html", markup_template).expect("unable to create root page");
}