// extern crate rand;
// extern crate js_sys;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{console, Element};
use js_sys::{Math};
// use rand::prelude::*;


// When the `wee_alloc` feature is enabled, this uses `wee_alloc` as the global
// allocator.
//
// If you don't want to use `wee_alloc`, you can safely delete this.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

enum Choice {
    Rock,
    Paper,
    Scissors
}


// This is like the `main` function, except for JavaScript.
#[wasm_bindgen]
pub fn main_js() -> Result<(), JsValue> {
    // This provides better error messages in debug mode.
    // It's disabled in release mode so it doesn't bloat up the file size.
    #[cfg(debug_assertions)]
    console_error_panic_hook::set_once();


    // Your code goes here!
    let random_choice = get_random_choice().unwrap();
    console::log_1(&JsValue::from_str(get_choice(random_choice).as_str()));

    let button = query_selector("button").unwrap();

    let closure = Closure::wrap(Box::new(move |_event: web_sys::MouseEvent| {
        console::log_1(&JsValue::from_str("clicked button!"));
    }) as Box<dyn FnMut(_)>);
    button.add_event_listener_with_callback("click", closure.as_ref().unchecked_ref())?;
    closure.forget();

    Ok(())
}

/// Convert the game choice into it's corresponding String value
fn get_choice(choice: Choice) -> String {
    match choice {
        Choice::Rock => String::from("Rock"),
        Choice::Paper => String::from("Paper"),
        Choice::Scissors => String::from("Scissors")
    }
}

/// Get a random game option
fn get_random_choice() -> Option<Choice> {
    let num = get_random_int(3);
    match num {
        0 => Some(Choice::Rock),
        1 => Some(Choice::Paper),
        2 => Some(Choice::Scissors),
        _ => None
    }
}

/// Get a random u64 up to, but not including, the max
fn get_random_int(max: u64) -> u64 {
    use Math::{floor, random};
    floor(random() * floor(max as f64)) as u64
}

/// Wrapper for the query_element API
fn query_selector(selector: &str) -> Option<Element> {
    let body: Element = web_sys::window()
        ?.document()
        ?.body()
        ?.into();

    body.query_selector(selector).ok()?
}