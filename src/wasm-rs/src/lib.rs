use wasm_bindgen::closure::Closure;
use wasm_bindgen::prelude::wasm_bindgen;
use wasm_bindgen::{JsCast, JsValue};
use xterm_js_rs::addons::fit::FitAddon;
use xterm_js_rs::{OnKeyEvent, Terminal, TerminalOptions, Theme};

#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

// https://notes.burke.libbey.me/ansi-escape-codes/
const KEY_ENTER: u32 = 13;
const KEY_BACKSPACE: u32 = 8;
const KEY_LEFT_ARROW: u32 = 37;
const KEY_UP_ARROW: u32 = 38;
const KEY_RIGHT_ARROW: u32 = 39;
const KEY_DOWN_ARROW: u32 = 40;

const CURSOR_LEFT: &str = "\x1b[D";
const CURSOR_RIGHT: &str = "\x1b[C";

// Heavily Inspired By: https://github.com/segeljakt/xterm-js-rs/blob/master/example/src/lib.rs
#[wasm_bindgen(start)]
pub fn main() -> Result<(), JsValue> {
    let terminal: Terminal = Terminal::new(
        TerminalOptions::new()
            .with_cursor_blink(true)
            .with_cursor_width(8)
            .with_font_size(16)
            .with_draw_bold_text_in_bright_colors(true)
            .with_right_click_selects_word(true)
            .with_theme(
                Theme::new()
                    .with_foreground("#FFFFFF")
                    .with_background("#000000"),
            ),
    );

    let elem = web_sys::window()
        .unwrap()
        .document()
        .unwrap()
        .get_element_by_id("terminal")
        .unwrap();

    let mut line = String::new();
    let mut cursor_col = 0;

    intro(&terminal);
    terminal.open(elem.dyn_into()?);
    prompt(&terminal);

    let term: Terminal = terminal.clone().dyn_into()?;

    let callback = Closure::wrap(Box::new(move |e: OnKeyEvent| {
        let event = e.dom_event();
        match event.key_code() {
            KEY_ENTER => {
                if !line.is_empty() {
                    term.writeln("");
                    match line.as_str() {
                        "about" => about(&term),
                        "experience" => experience(&term),
                        "contact" => contact(&term),
                        "clear" => term.clear(),
                        _ => help(&term),
                    }
                    line.clear();
                    cursor_col = 0;
                }
                prompt(&term);
            }
            KEY_BACKSPACE => {
                if cursor_col > 0 {
                    term.write("\u{0008} \u{0008}");
                    line.pop();
                    cursor_col -= 1;
                }
            }
            KEY_LEFT_ARROW => {
                if cursor_col > 0 {
                    term.write(CURSOR_LEFT);
                    cursor_col -= 1;
                }
            }
            KEY_RIGHT_ARROW => {
                if cursor_col < line.len() {
                    term.write(CURSOR_RIGHT);
                    cursor_col += 1;
                }
            }
            KEY_UP_ARROW | KEY_DOWN_ARROW => (),
            _ => {
                if !event.alt_key() && !event.alt_key() && !event.ctrl_key() && !event.meta_key() {
                    term.write(&event.key());
                    line.push_str(&e.key());
                    cursor_col += 1;
                }
            }
        }
    }) as Box<dyn FnMut(_)>);

    terminal.on_key(callback.as_ref().unchecked_ref());
    callback.forget();

    let addon = FitAddon::new();
    terminal.load_addon(addon.clone().dyn_into::<FitAddon>()?.into());
    addon.fit();
    terminal.focus();

    Ok(())
}

fn intro(term: &Terminal) {
    about(term);
    prompt(term);
    term.writeln("help");
    help(term);
}

fn about(term: &Terminal) {
    const GREETING: &str =
        "Hello, I am \x1b[1;33mDarius Carrier\x1b[0m, a Software Site Reliability Engineer. Please use the terminal below to learn about me!";
    term.writeln(GREETING);
    term.writeln("");
    term.writeln("");
    term.writeln("Powered by: 🦀 Rust + WASM");
}

fn contact(term: &Terminal) {
    term.writeln("");
    term.writeln("https://github.com/dcarrier");
    term.writeln("https://linkedin.com/in/ddcarrier");
}

fn experience(term: &Terminal) {
    term.writeln("");
    term.writeln("💻  Languages: Go, Rust, Python");
    term.writeln("🚀  Built server provisioning Go service for worldwide CDN.");
    term.writeln("🚀  Built pop on a stick automation for distributed datacenter turn up.")
}

fn help(term: &Terminal) {
    term.writeln("commands: about, clear, contact, experience, help");
}

fn prompt(term: &Terminal) {
    const PROMPT: &str = "$ ";
    term.writeln("");
    term.write(PROMPT);
}
