extern crate minifier;
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

const CSS_PLACEHOLDER: &str = "/* CSS_INCLUDE */";
const TITLE_PLACEHOLDER: &str = "{{ PAGE_TITLE_TEMPLATE }}";
const LIST_PLACEHOLDER: &str = "{{ LIST_TEMPLATE }}";
const HREF_PLACEHOLDER: &str = "{{ HREF }}";
const LINK_TEXT_PLACEHOLDER: &str = "{{ TEXT }}";

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
}

// Get the deserialised config file
fn get_config_data(path: String) -> Result<Vec::<Project>, serde_json::error::Error> {

  let config = std::fs::read_to_string(path).unwrap();

  let mut projects: Vec<Project> = serde_json::from_str(config.as_str())?;

  // sort the pens, oldest to newest
  for n in 0..projects.len() {
    projects[n].pens.sort_by(|a, b| b.timestamp.cmp(&a.timestamp));
  }

  Ok(projects)
}

// Load a HTML template as a String
// from the file system
fn get_html_template(path: String) -> String {
  let mut markup = std::fs::read_to_string(path).unwrap();

  if markup.contains(&CSS_PLACEHOLDER) {
    markup = markup.replace(CSS_PLACEHOLDER, get_root_css().as_str());
  }

  String::from(markup)
}

// Build the <li> HTML for project pages.
// This might be more reusable if it takes separate href and text arguments
fn build_pen_list_markup(html: &String, href: &String, text: &String) -> String {
  let parts = href.split("/");
  let link = parts.collect::<Vec<_>>().pop().unwrap();
  let mut markup = html.replace(HREF_PLACEHOLDER, link);
  markup = markup.replace(LINK_TEXT_PLACEHOLDER, text.as_str());
  
  markup
}

// Loop through the top level
// projects and build the pens index.html page
fn build_landing_page(data: &Vec<Project>) {

  let mut page_markup_template = get_html_template(String::from("html/templates/list-page.html"));
  page_markup_template = page_markup_template.replace(TITLE_PLACEHOLDER, "Pens");

  let list_markup_template = get_html_template(String::from("html/partials/pen-link.html"));
  let mut list_markup = String::new();

  for project in data {
    list_markup.push_str(build_pen_list_markup(&list_markup_template, &project.name, &project.text).as_str());
  }

  page_markup_template = page_markup_template.replace(LIST_PLACEHOLDER, list_markup.as_str());

  write("pens/index.html", page_markup_template).expect("unable to create index file");
}

fn build_project_page(project: &Project) {
  let mut page_template = get_html_template(String::from("html/templates/list-page.html"));
  page_template = page_template.replace(TITLE_PLACEHOLDER, project.text.as_str());

  let list_markup_template = get_html_template(String::from("html/partials/pen-link.html"));
  let mut list_markup = String::new();

  for pen in project.pens.iter() {
    list_markup.push_str(build_pen_list_markup(&list_markup_template, &pen.path, &pen.text).as_str());
  }

  page_template = page_template.replace(LIST_PLACEHOLDER, list_markup.as_str());
  
  write(format!("pens/{}/index.html", project.name), page_template).expect("unable to create project page");
}

fn build_root_page() {
  let markup_template = get_html_template(String::from("html/templates/root.html"));

  // don't need to actually do any transformation currently...
  write("index.html", markup_template).expect("unable to create root page");
}

fn get_root_css() -> String {
  let css = get_html_template(String::from("assets/css/main.css"));

  minifier::css::minify(css.as_str()).unwrap()
}