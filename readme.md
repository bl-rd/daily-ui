# Stuff made by me

I'm trying to learn [Rust](https://www.rust-lang.org/) so I've had a go at making my own (sort of) static site generator! In the spirit of the [IndieWeb](https://indieweb.org/), I'd like to be able to keep a copy of things I'me made in [Codepen]() locally and publish there when I'm ready.

This is also going to act as a bit of a ~dumping ground~ experimentation area that can live outside of my [personal website](https://bl-rd.dev/).

## Requirements

Needs [Rust](https://www.rust-lang.org/) to run the build process. The easiest way to get everything needed is to install [rustup](https://rustup.rs/). After cloning the repository, make sure to build the program that will build the site.

```bash
cargo build --release
```

Also make sure to run `npm install` (or [Yarn](https://yarnpkg.com/); or [pnpm](https://pnpm.js.org/), which I use).

## Building the site

To run locally, run

```bash
npm run serve
```

which will build all the html files and start a local web server.

## Creating new pages

There is a `config.json` file that stores all the information for building the site. If a new section is needed, make sure to create it first as this is needed (for now) before any scaffolding takes place. e.g.

```json
[
  {
    "name": "daily-ui",
    "text": "Daily UI",
    "pens": []
  }
]
```

Now to scaffold a new project into here, run

```bash
npm run go -- js daily-ui/login-page "A login page"
```
This will create a basic javascript-powered directory under `/pens/daily-ui/login-page`. To create a rust powered hybrid app, change the `js` argument to `rust`. Both of these commands update the `config.json` file automatically.

Make sure to run `npm run build` before commiting.