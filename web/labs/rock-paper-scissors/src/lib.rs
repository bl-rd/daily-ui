use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{console, Element};
use js_sys::{Math};


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

enum Outcome {
    Win,
    Lose,
    Draw
}

enum State {
    Menu,
    Play,
    Outcome
}

// This is like the `main` function, except for JavaScript.
#[wasm_bindgen(start)]
pub fn main_js() -> Result<(), JsValue> {
    // This provides better error messages in debug mode.
    // It's disabled in release mode so it doesn't bloat up the file size.
    #[cfg(debug_assertions)]
    console_error_panic_hook::set_once();

    // start the game!
    change_state(State::Menu);

    Ok(())
}

/// Convert the game choice into it's corresponding String value
fn get_choice(choice: &Choice) -> String {
    match choice {
        Choice::Rock => String::from("Rock"),
        Choice::Paper => String::from("Paper"),
        Choice::Scissors => String::from("Scissors")
    }
}

/// Convert the outcome to it's corresponding String value
fn get_outcome(outcome: Outcome) -> String {
    match outcome {
        Outcome::Win => String::from("You win!"),
        Outcome::Lose => String::from("You lose!"),
        Outcome::Draw => String::from("It's a draw!")
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

/// Determine the outcome based on the two choices
fn battle(player_choice: Choice, ai_choice: Choice) -> Outcome {
    match player_choice {
        Choice::Rock => {
            match ai_choice {
                Choice::Rock => Outcome::Draw,
                Choice::Paper => Outcome::Lose,
                Choice::Scissors => Outcome::Win
            }
        },
        Choice::Paper => {
            match ai_choice {
                Choice::Rock => Outcome::Win,
                Choice::Paper => Outcome::Draw,
                Choice::Scissors => Outcome::Lose
            }
        },
        Choice::Scissors => {
            match ai_choice {
                Choice::Rock => Outcome::Lose,
                Choice::Paper => Outcome::Win,
                Choice::Scissors => Outcome::Draw
            }
        }
    }
}

/// Function to have a simple game, based on player choice
fn play(player_choice: Choice) {
    console_log(format!("You chose {}", get_choice(&player_choice)).as_str());
    let ai_choice = get_random_choice().unwrap();
    console_log(format!("Opponent chose {}", get_choice(&ai_choice)).as_str());
    let outcome = battle(player_choice, ai_choice);
    console_log(get_outcome(outcome).as_str());
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

/// wrapper for the web_sys console::log_1 method
fn console_log(message: &str) {
    console::log_1(&JsValue::from_str(message));
}

/// function for handling the changes in the game state
fn change_state(state: State) {
    match state {
        State::Menu => init_menu_state(),
        State::Play => init_play_state(),
        State::Outcome => {
            console_log("Outcome state");
        }
    }
}

/// logic for the "menu" state of the game
fn init_menu_state() {
    let menu_element = query_selector(".game__menu").unwrap();

    menu_element
        .remove_attribute("hidden")
        .expect("Can't remove hidden attribute from menu element");

    // get the button element
    let play_button = query_selector("button#play").unwrap();
    // if the button is clicked, update the state
    let play_closure = Closure::wrap(Box::new(move |_event: web_sys::MouseEvent| {
        match menu_element.set_attribute("hidden", "true") {
            Ok(()) => change_state(State::Play),
            Err(_) => console_log("Unable to hide menu element")
        }
    }) as Box<dyn FnMut(_)>);
    play_button.add_event_listener_with_callback("click", play_closure.as_ref().unchecked_ref()).unwrap();
    play_closure.forget();
}

fn init_play_state() {

    let play_element = query_selector(".game__play").unwrap();

    play_element
        .remove_attribute("hidden")
        .expect("Can't remove hidden attribute from play element");

    let rock_button = query_selector("#rock-button").unwrap();
    let paper_button = query_selector("#paper-button").unwrap();
    let scissors_button = query_selector("#scissors-button").unwrap();

    let rock_closure = Closure::wrap(Box::new(move |_event: web_sys::MouseEvent| {
        play(Choice::Rock);
    }) as Box<dyn FnMut(_)>);

    let paper_closure = Closure::wrap(Box::new(move |_event: web_sys::MouseEvent| {
        play(Choice::Paper);
    }) as Box<dyn FnMut(_)>);

    let scissors_closure = Closure::wrap(Box::new(move |_event: web_sys::MouseEvent| {
        play(Choice::Scissors);
    }) as Box<dyn FnMut(_)>);

    rock_button.add_event_listener_with_callback("click", rock_closure.as_ref().unchecked_ref()).unwrap();
    rock_closure.forget();
    paper_button.add_event_listener_with_callback("click", paper_closure.as_ref().unchecked_ref()).unwrap();
    paper_closure.forget();
    scissors_button.add_event_listener_with_callback("click", scissors_closure.as_ref().unchecked_ref()).unwrap();
    scissors_closure.forget();
}

fn init_outcome_state() {
    console_log("nope");
}