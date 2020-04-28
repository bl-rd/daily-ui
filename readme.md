# Pens

TODO: write some information about this project

## What??

- Create a Rust app that includes all the scaffolding files in a directory of choosing (similar to Hugo)
  - Allow some presets? Similar to taxonomies??
- Have some sort of config file that can build an index page of all the files
- Include a dev server?
- Have a command to build all the files so when running the app the projects can be viewed
- Link to source files
- Post to Codepen?? [API](https://blog.codepen.io/documentation/api/prefill/)

- two types
  - standard js scaffolding
  - wasm-pack scaffolding

## What needs to happen...

- cli command to generate new sub-directory
- need to choose if it's a rust or js project
- saved in a config file that is used to build the index pages

### MVP

- cli command that adds to the config file

- parse the config file into structured data
- add the new entry (with unix timestamp)
- serialize into JSON and write back

## Scaffold a new project

`cargo run <type> <path> <friendly name>`

e.g.

```bash
cargo run rust labs/rss-reader "In browser RSS reader"
```

## TODO

- Support scaffolding a wasm-pack scaffold that targets `--web` (i.e. does not need webpack)
- Support a JavaScript scaffold
- Build the HTML files