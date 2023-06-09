use crate::controller::{chat, settings};
use crate::ui;
use crate::utils::clear;

pub fn main_controller() {
    loop {
        let options: Vec<&str> = vec!["Chat", "Settings", "Exit"];
        let choice = ui::main::display_main_menu(options);

        clear();
        match choice {
            "Chat" => chat::main_chat_controller(),
            "Settings" => settings::settings_controller(),
            "Exit" => break,
            _ => break,
        };
    }
}
