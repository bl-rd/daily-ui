use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{console, Element, HtmlElement};
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
    Win,
    Lose,
    Draw
}

enum ButtonState {
    Disabled,
    Enabled
}

// The button selectors
const BUTTON_ROCK_SELECTOR: &str = "#rock";
const BUTTON_PAPER_SELECTOR: &str = "#paper";
const BUTTON_SCISSORS_SELECTOR: &str = "#scissors";
const BUTTON_ACTION_SELECTOR: &str = "#again";
const BUTTON_PLAY_SELECTOR: &str = "#play";

// the game UI selectors
const UI_OUTCOME_SELECTOR: &str = ".ui__outcome";
const UI_PLAY_SELECTOR: &str = ".ui__play";
const UI_MENU_SELECTOR: &str = ".ui__menu";

const SCORE_PLAYER: &str = ".score--player";
const SCORE_AI: &str = ".score--ai";

const IMAGE_SELECTOR: &str = ".game__element img";
const IMAGE_PLAYER_SELECTOR: &str = ".game__element--player img";
const IMAGE_AI_SELECTOR: &str = ".game__element--ai img";
const TEXT_PLAYER_SELECTOR: &str = ".game__element--player .game__element__text";
const TEXT_AI_SELECTOR: &str = ".game__element--ai .game__element__text";
const TEXT_SELECTOR: &str = ".game__element__text";
const RESULT_SELECTOR: &str = ".ui__result";

// animation classes
const ANIM_ROCK_IT: &str = "rock-it";
const ANIM_WIN: &str = "win";
const ANIM_LOSE: &str = "lose";

// other options
const STATE_TRANSITION_TIME: i32 = 1750;

// This is like the `main` function, except for JavaScript.
#[wasm_bindgen(start)]
pub fn main_js() -> Result<(), JsValue> {
    // This provides better error messages in debug mode.
    // It's disabled in release mode so it doesn't bloat up the file size.
    #[cfg(debug_assertions)]
    console_error_panic_hook::set_once();


    // get references to the UI elements
    let rock_button = query_selector(BUTTON_ROCK_SELECTOR).unwrap();
    let paper_button = query_selector(BUTTON_PAPER_SELECTOR).unwrap();
    let scissors_button = query_selector(BUTTON_SCISSORS_SELECTOR).unwrap();
    let again_button = query_selector(BUTTON_ACTION_SELECTOR).unwrap();
    let player_image = query_selector(IMAGE_PLAYER_SELECTOR).unwrap();
    let ai_image = query_selector(IMAGE_AI_SELECTOR).unwrap();

    // all the button callbacks
    let rock_closure = Closure::wrap(Box::new(move |_event: web_sys::MouseEvent| {
        play(Choice::Rock);
    }) as Box<dyn FnMut(_)>);

    let paper_closure = Closure::wrap(Box::new(move |_event: web_sys::MouseEvent| {
        play(Choice::Paper);
    }) as Box<dyn FnMut(_)>);

    let scissors_closure = Closure::wrap(Box::new(move |_event: web_sys::MouseEvent| {
        play(Choice::Scissors);
    }) as Box<dyn FnMut(_)>);

    let again_button_closure = Closure::wrap(Box::new(move |_event: web_sys::MouseEvent| {
        reset_text(TEXT_SELECTOR);
        modify_win_class(IMAGE_PLAYER_SELECTOR, Outcome::Draw);
        modify_win_class(IMAGE_AI_SELECTOR, Outcome::Draw);
        hide_elements(UI_OUTCOME_SELECTOR);
        hide_elements(RESULT_SELECTOR);
        update_image(&player_image, &Choice::Rock, false);
        update_image(&ai_image, &Choice::Rock, true);
        change_state(State::Play);
    }) as Box<dyn FnMut(_)>);

    // Add the event listeners to all the game buttons
    rock_button.add_event_listener_with_callback("click", rock_closure.as_ref().unchecked_ref()).unwrap();
    rock_closure.forget();
    paper_button.add_event_listener_with_callback("click", paper_closure.as_ref().unchecked_ref()).unwrap();
    paper_closure.forget();
    scissors_button.add_event_listener_with_callback("click", scissors_closure.as_ref().unchecked_ref()).unwrap();
    scissors_closure.forget();

    again_button.add_event_listener_with_callback("click", again_button_closure.as_ref().unchecked_ref()).unwrap();
    again_button_closure.forget();

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
fn get_outcome(outcome: &Outcome) -> String {
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
fn battle(player_choice: &Choice, ai_choice: &Choice) -> Outcome {
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

/// Helper function to enable/disable the play buttons
fn update_button_disabled_state(state: ButtonState) {
    let rock_button = query_selector(BUTTON_ROCK_SELECTOR).unwrap();
    let paper_button = query_selector(BUTTON_PAPER_SELECTOR).unwrap();
    let scissors_button = query_selector(BUTTON_SCISSORS_SELECTOR).unwrap();

    match state {
        ButtonState::Disabled => {
            rock_button.set_attribute("disabled", "true").expect("Unable to update element's disabled state");
            paper_button.set_attribute("disabled", "true").expect("Unable to update element's disabled state");
            scissors_button.set_attribute("disabled", "true").expect("Unable to update element's disabled state");
        },
        ButtonState::Enabled => {
            rock_button.remove_attribute("disabled").expect("Can't remove disabled attribute from button element");
            paper_button.remove_attribute("disabled").expect("Can't remove disabled attribute from button element");
            scissors_button.remove_attribute("disabled").expect("Can't remove disabled attribute from button element");
        }
    }    
}

/// Function to have a simple game, based on player choice
fn play(player_choice: Choice) {
    update_button_disabled_state(ButtonState::Disabled);

    // rock it
    make_it_rock(IMAGE_SELECTOR, false);

    let window = web_sys::window().expect("Should have a window...");
    let closure = Closure::wrap(Box::new(move || {
        let ai_choice = get_random_choice().unwrap();

        make_it_rock(IMAGE_SELECTOR, true);

        let player_image = query_selector(IMAGE_PLAYER_SELECTOR).unwrap();
        let player_text = query_selector(TEXT_PLAYER_SELECTOR).unwrap();
        let ai_image = query_selector(IMAGE_AI_SELECTOR).unwrap();
        let ai_text = query_selector(TEXT_AI_SELECTOR).unwrap();

        // update the images
        update_image(&player_image, &player_choice, false);
        update_image(&ai_image, &ai_choice, true);

        // update the text
        player_text.set_inner_text(get_choice(&player_choice).as_str());
        ai_text.set_inner_text(get_choice(&ai_choice).as_str());
        show_elements(TEXT_SELECTOR);

        let outcome = battle(&player_choice, &ai_choice);
        match outcome {
            Outcome::Draw => change_state(State::Draw),
            Outcome::Win => change_state(State::Win),
            Outcome::Lose => change_state(State::Lose)
        }
    }) as Box<dyn FnMut()>);
    window.set_timeout_with_callback_and_timeout_and_arguments_0(closure.as_ref().unchecked_ref(), STATE_TRANSITION_TIME)
        .expect("Unable to do timeout");
    closure.forget();
}

/// Get a random u64 up to, but not including, the max
fn get_random_int(max: u64) -> u64 {
    use Math::{floor, random};
    floor(random() * floor(max as f64)) as u64
}

/// Wrapper for the query_element API
fn query_selector(selector: &str) -> Option<HtmlElement> {
    let body: Element = web_sys::window()
        ?.document()
        ?.body()
        ?.into();

    let element = body.query_selector(selector)
        .ok()?
        .unwrap();
    
    // convert into HtmlElement as this has more useful methods
    element.dyn_into::<HtmlElement>().ok()
}

fn query_selector_all(selector: &str) -> Option<Vec::<HtmlElement>> {
    let body: web_sys::Document = web_sys::window()
        ?.document()
        ?.into();
    
    let nodes = body.query_selector_all(selector)
        .ok()?;
    
    let mut elements: Vec::<HtmlElement> = Vec::<HtmlElement>::new();

    for n in 0..nodes.length() {
        let e: HtmlElement = nodes
            .item(n)
            .unwrap()
            .dyn_into::<HtmlElement>()
            .ok()
            .unwrap();
        elements.push(e);
    };

    Some(elements)
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
        State::Win => init_outcome_state(Outcome::Win),
        State::Lose => init_outcome_state(Outcome::Lose),
        State::Draw => init_outcome_state(Outcome::Draw)
    }
}

/// logic for the "menu" state of the game
fn init_menu_state() {

    show_elements(UI_MENU_SELECTOR);

    // get the button element
    let play_button = query_selector(BUTTON_PLAY_SELECTOR).unwrap();

    // if the button is clicked, update the state
    let play_closure = Closure::wrap(Box::new(move |_event: web_sys::MouseEvent| {
        hide_elements(UI_MENU_SELECTOR);
        change_state(State::Play);
    }) as Box<dyn FnMut(_)>);
    play_button.add_event_listener_with_callback("click", play_closure.as_ref().unchecked_ref()).unwrap();
    play_closure.forget();
}

fn init_play_state() {

    show_elements(UI_PLAY_SELECTOR);

    update_button_disabled_state(ButtonState::Enabled);
}

fn init_outcome_state(outcome: Outcome) {

    hide_elements(UI_PLAY_SELECTOR);

    let result_text = query_selector(format!("{} h2", RESULT_SELECTOR).as_str()).unwrap();
    result_text.set_inner_text(get_outcome(&outcome).as_str());

    // closure for the set_timeout.
    let closure = Closure::wrap(Box::new(move || {
        match &outcome {
            Outcome::Draw => console_log(get_outcome(&outcome).as_str()),
            Outcome::Win => {
                modify_win_class(IMAGE_PLAYER_SELECTOR, Outcome::Win);
                modify_win_class(IMAGE_AI_SELECTOR, Outcome::Lose);
                increase_score(SCORE_PLAYER);
            },
            Outcome::Lose => {
                modify_win_class(IMAGE_PLAYER_SELECTOR, Outcome::Lose);
                modify_win_class(IMAGE_AI_SELECTOR, Outcome::Win);
                increase_score(SCORE_AI);
            }
        };
        show_elements(UI_OUTCOME_SELECTOR);
        show_elements(RESULT_SELECTOR);
    }) as Box<dyn FnMut()>);

    let window = web_sys::window().expect("Should have a window...");
    window
        .set_timeout_with_callback_and_timeout_and_arguments_0(
            closure.as_ref().unchecked_ref(),
            STATE_TRANSITION_TIME
        )
        .unwrap();
    closure.forget();
}

/// hide elements
fn hide_elements(selector: &str) {
    let elements = query_selector_all(selector).unwrap();

    for elem in elements.iter() {
        elem
            .set_attribute("hidden", "true")
            .expect(format!("cannot hide element '{}'", selector).as_str());        
    }
}

/// show elements
fn show_elements(selector: &str) {
    let elements = query_selector_all(selector).unwrap();

    for elem in elements.iter() {
        elem
            .remove_attribute("hidden")
            .expect(format!("Unable to remove hidden attribute from {}", selector).as_str());
    }
}

/// Update an image based on user/ai choice
fn update_image(img: &HtmlElement, selection: &Choice, alt: bool) {
    let img_src: &str;
    let img_alt: &str;

    match selection {
        Choice::Rock => {
            img_src = if alt { "assets/rock-500-alt.png" } else { "assets/rock-500.png" };
            img_alt =  "A hand doing a rock sign"; 
        },
        Choice::Paper => {
            img_src = if alt { "assets/paper-500-alt.png" } else { "assets/paper-500.png" };
            img_alt =  "A hand doing a paper sign"; 
        },
        Choice::Scissors => {
            img_src = if alt { "assets/scissors-500-alt.png" } else { "assets/scissors-500.png" };
            img_alt =  "A hand doing a scissors sign"; 
        }
    };

    img.set_attribute("src", img_src).expect("unable to set img src");
    img.set_attribute("alt", img_alt).expect("unable to set img alt");
}

/// reset the text to an empty string to preserve layout
fn reset_text(selector: &str) {
    let elements = query_selector_all(selector).unwrap();

    for elem in elements.iter() {
        elem.set_inner_html("&nbsp;");
    }
}

/// add the rock classes!
fn make_it_rock(selector: &str, remove: bool) {
    let elements = query_selector_all(selector).unwrap();

    for elem in elements.iter() {
        if remove {
            elem.set_class_name("");
        } else {
            elem.set_class_name(ANIM_ROCK_IT);
        }
    }
}

fn increase_score(selector: &str) {
    let element = query_selector(selector).unwrap();

    let curr_str = element.get_attribute("data-score").unwrap();
    let curr = curr_str.parse::<u32>().unwrap();
    let score = curr + 1;
    element.set_attribute("data-score", score.to_string().as_str())
        .expect("Cannot set the score for the element");
}

fn modify_win_class(selector: &str, outcome: Outcome) {
    let element = query_selector(selector).unwrap();

    match outcome {
        Outcome::Win => element.set_class_name(ANIM_WIN),
        Outcome::Lose => element.set_class_name(ANIM_LOSE),
        Outcome::Draw => element.set_class_name("")
    };
}