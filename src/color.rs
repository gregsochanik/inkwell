//! ANSI colour codes and helpers for terminal output.
#![allow(dead_code)]

pub const RESET: &str = "\x1b[0m";
pub const BOLD: &str = "\x1b[1m";
pub const DIM: &str = "\x1b[2m";
pub const ITALIC: &str = "\x1b[3m";

pub const RED: &str = "\x1b[31m";
pub const GREEN: &str = "\x1b[32m";
pub const YELLOW: &str = "\x1b[33m";
pub const MAGENTA: &str = "\x1b[35m";
pub const CYAN: &str = "\x1b[36m";

pub const BRIGHT_RED: &str = "\x1b[91m";
pub const BRIGHT_YELLOW: &str = "\x1b[93m";

/// Room title — bold cyan.
pub fn room_title(text: &str) -> String {
    format!("{BOLD}{CYAN}{text}{RESET}")
}

/// Item name — yellow.
pub fn item(text: &str) -> String {
    format!("{YELLOW}{text}{RESET}")
}

/// NPC name — bold green.
pub fn npc(text: &str) -> String {
    format!("{BOLD}{GREEN}{text}{RESET}")
}

/// Exits line — bold.
pub fn exits(text: &str) -> String {
    format!("{BOLD}{text}{RESET}")
}

/// Danger / death text — bright red.
pub fn danger(text: &str) -> String {
    format!("{BRIGHT_RED}{text}{RESET}")
}

/// Success / win text — bold bright yellow.
pub fn success(text: &str) -> String {
    format!("{BOLD}{BRIGHT_YELLOW}{text}{RESET}")
}

/// Riddle text — italic magenta.
pub fn riddle(text: &str) -> String {
    format!("{ITALIC}{MAGENTA}{text}{RESET}")
}

/// NPC dialogue — green.
pub fn dialogue(text: &str) -> String {
    format!("{GREEN}{text}{RESET}")
}

/// Terminal bell character.
pub const BELL: &str = "\x07";
