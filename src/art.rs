//! ANSI half-block pixel art for room illustrations.
//!
//! Uses Unicode half-block characters (▀▄█) with ANSI foreground/background
//! colours to create pseudo-pixel art at 2× vertical resolution.

#![allow(dead_code, unused_imports)]

use crate::color;

/// Return room art for a given room id, or None if no art exists.
pub fn get_room_art(room_id: &str) -> Option<String> {
    match room_id {
        "bag_end" => Some(bag_end()),
        "the_hill" => Some(the_hill()),
        "green_dragon" => Some(green_dragon()),
        _ => None,
    }
}

// -- Colour palette constants ------------------------------------------------

// Foreground
const FG_GREEN: &str = "\x1b[32m";
const FG_BRIGHT_GREEN: &str = "\x1b[92m";
const FG_YELLOW: &str = "\x1b[33m";
const FG_BRIGHT_YELLOW: &str = "\x1b[93m";
const FG_RED: &str = "\x1b[31m";
const FG_BROWN: &str = "\x1b[33m";
const FG_CYAN: &str = "\x1b[36m";
const FG_BRIGHT_CYAN: &str = "\x1b[96m";
const FG_WHITE: &str = "\x1b[37m";
const FG_BRIGHT_WHITE: &str = "\x1b[97m";
const FG_GRAY: &str = "\x1b[90m";
const FG_BLACK: &str = "\x1b[30m";

// Background
const BG_GREEN: &str = "\x1b[42m";
const BG_BRIGHT_GREEN: &str = "\x1b[102m";
const BG_YELLOW: &str = "\x1b[43m";
const BG_BROWN: &str = "\x1b[43m";
const BG_CYAN: &str = "\x1b[46m";
const BG_BLUE: &str = "\x1b[44m";
const BG_BLACK: &str = "\x1b[40m";
const BG_WHITE: &str = "\x1b[47m";
const BG_RED: &str = "\x1b[41m";
const BG_BRIGHT_YELLOW: &str = "\x1b[103m";
const BG_GRAY: &str = "\x1b[100m";

const Z: &str = "\x1b[0m"; // reset

// -- Art functions ------------------------------------------------------------

fn bag_end() -> String {
    // A hobbit-hole: round green door in a grassy hillside, round windows
    // with warm glow, chimney smoke, flowers, fence, and winding path.
    //
    // Every row: 2-space indent + 48 art chars = 50 visible chars.

    // Shorthand aliases for this scene
    let s  = BG_CYAN;          // sky bg
    let sf = FG_CYAN;          // sky fg (invisible filler)
    let cl = FG_BRIGHT_WHITE;  // cloud fg
    let h  = BG_GREEN;         // hill bg
    let hf = FG_GREEN;         // hill fg
    let w  = BG_YELLOW;        // window glow bg
    let wf = FG_BRIGHT_YELLOW; // window / bright fg
    let d  = BG_BROWN;         // door bg
    let df = FG_BROWN;         // door frame fg
    let yf = FG_YELLOW;        // door panel fg
    let kf = FG_BRIGHT_WHITE;  // doorknob fg
    let sm = FG_WHITE;         // chimney smoke fg
    let rf = FG_RED;           // flower fg
    let pf = FG_BROWN;         // path fg
    let z  = Z;

    let mut a = String::with_capacity(2048);

    // Row 1: Sky with clouds  (8 + 32 + 8 = 48)
    a += &format!(
        "  {s}{cl}  ░░    {z}\
         {s}{sf}                                {z}\
         {s}{cl}    ░░  {z}\n");

    // Row 2: Hill top emerging  (19 + 10 + 19 = 48)
    a += &format!(
        "  {s}{sf}                   {z}\
         {s}{hf}▄▄▄▄▄▄▄▄▄▄{z}\
         {s}{sf}                   {z}\n");

    // Row 3: Hill wider  (12 + 5 + 14 + 5 + 12 = 48)
    a += &format!(
        "  {s}{sf}            {z}\
         {s}{hf}▄▄▄▄▄{z}\
         {h}{hf}██████████████{z}\
         {s}{hf}▄▄▄▄▄{z}\
         {s}{sf}            {z}\n");

    // Row 4: Hill + chimney smoke  (8 + 5 + 4 + 18 + 5 + 8 = 48)
    a += &format!(
        "  {s}{sf}        {z}\
         {s}{hf}▄▄▄▄▄{z}\
         {h}{sm} ~~ {z}\
         {h}{hf}██████████████████{z}\
         {s}{hf}▄▄▄▄▄{z}\
         {s}{sf}        {z}\n");

    // Row 5: Door frame top + window tops  (11 + 6 + 3 + 8 + 3 + 6 + 11 = 48)
    a += &format!(
        "  {s}{hf}▄▄▄▄▄▄▄▄▄▄▄{z}\
         {h}{wf} ▄▄▄▄ {z}\
         {h}{hf}   {z}\
         {h}{df}╭──────╮{z}\
         {h}{hf}   {z}\
         {h}{wf} ▄▄▄▄ {z}\
         {s}{hf}▄▄▄▄▄▄▄▄▄▄▄{z}\n");

    // Row 6: Windows + door  (11 + [1+4+1] + 2 + [1+6+1] + 2 + [1+4+1] + 13 = 48)
    a += &format!(
        "  {h}{hf}███████████{z}\
         {h}{wf}▐{z}{w}{wf}▓▓▓▓{z}{h}{wf}▌{z}\
         {h}{hf}  {z}\
         {h}{df}│{z}{d}{yf}░░▒▒░░{z}{h}{df}│{z}\
         {h}{hf}  {z}\
         {h}{wf}▐{z}{w}{wf}▓▓▓▓{z}{h}{wf}▌{z}\
         {h}{hf}█████████████{z}\n");

    // Row 7: Windows + door with knob  (11 + [1+4+1] + 2 + [1+3+1+2+1] + 2 + [1+4+1] + 13 = 48)
    a += &format!(
        "  {h}{hf}███████████{z}\
         {h}{wf}▐{z}{w}{wf}▓▓▓▓{z}{h}{wf}▌{z}\
         {h}{hf}  {z}\
         {h}{df}│{z}{d}{yf}░░▒{z}{d}{kf}•{z}{d}{yf}▒░{z}{h}{df}│{z}\
         {h}{hf}  {z}\
         {h}{wf}▐{z}{w}{wf}▓▓▓▓{z}{h}{wf}▌{z}\
         {h}{hf}█████████████{z}\n");

    // Row 8: Window bottoms + door  (11 + 6 + 2 + [1+6+1] + 2 + 6 + 13 = 48)
    a += &format!(
        "  {h}{hf}███████████{z}\
         {h}{wf} ▀▀▀▀ {z}\
         {h}{hf}  {z}\
         {h}{df}│{z}{d}{yf}░░▒▒░░{z}{h}{df}│{z}\
         {h}{hf}  {z}\
         {h}{wf} ▀▀▀▀ {z}\
         {h}{hf}█████████████{z}\n");

    // Row 9: Door bottom + flowers  (11 + 6 + 2 + 8 + 2 + 6 + 13 = 48)
    a += &format!(
        "  {h}{hf}███████████{z}\
         {h}{hf}      {z}\
         {h}{hf}  {z}\
         {h}{df}╰──────╯{z}\
         {h}{hf}  {z}\
         {h}{rf}✿{hf} {wf}✿{hf} {rf}✿{hf} {z}\
         {h}{hf}█████████████{z}\n");

    // Row 10: Fence + path + gate  (4 + 3 + 2 + 3 + 4 + 8 + 4 + 3 + 2 + 3 + 12 = 48)
    a += &format!(
        "  {h}{hf}████{z}\
         {h}{wf}┬─┬{z}\
         {h}{hf}██{z}\
         {h}{pf}░░░{z}\
         {h}{hf}████{z}\
         {h}{wf}════════{z}\
         {h}{hf}████{z}\
         {h}{pf}░░░{z}\
         {h}{hf}██{z}\
         {h}{wf}┬─┬{z}\
         {h}{hf}████████████{z}\n");

    // Row 11: Path winding away  (14 + 20 + 14 = 48)
    a += &format!(
        "  {h}{hf}██████████████{z}\
         {h}{pf}░░░░░░░░░░░░░░░░░░░░{z}\
         {h}{hf}██████████████{z}\n");

    a
}

fn green_dragon() -> String {
    // The Green Dragon Inn: a cozy pub front with peaked roof,
    // chimney smoke, glowing windows, hanging sign, door, and barrels.
    //
    // Every row: 2-space indent + 48 art chars = 50 visible chars.
    // Building is 40 chars wide, with 4 sky chars on each side.

    let s   = BG_CYAN;          // sky bg
    let sf  = FG_CYAN;          // sky filler
    let sm  = FG_WHITE;         // smoke
    let rb  = BG_RED;           // roof bg
    let rf  = FG_RED;           // roof fg
    let wb  = BG_BROWN;         // wall bg
    let wf  = FG_BROWN;         // wood fg
    let gf  = FG_BRIGHT_YELLOW; // glow fg
    let db  = BG_BLACK;         // door bg
    let df  = FG_BLACK;         // dark fg
    let cb  = BG_GRAY;          // cobblestone bg
    let cf  = FG_GRAY;          // cobble fg
    let gnb = BG_GREEN;         // grass bg
    let gnf = FG_GREEN;         // grass fg
    let wh  = FG_BRIGHT_WHITE;  // white fg
    let z   = Z;

    let mut a = String::with_capacity(2048);

    // Row 1: sky + smoke  (36 + 4 + 8 = 48)
    a += &format!(
        "  {s}{sf}                                    {z}\
         {s}{sm}~~~~{z}\
         {s}{sf}        {z}\n");

    // Row 2: sky + chimney + smoke  (18 + 2 + 3 + 25 = 48)
    a += &format!(
        "  {s}{sf}                  {z}\
         {s}{wf}▐▌{z}\
         {s}{sm} ~~{z}\
         {s}{sf}                         {z}\n");

    // Row 3: roof peak + chimney  (6 + 12 + 2 + 18 + 10 = 48)
    a += &format!(
        "  {s}{sf}      {z}\
         {s}{rf}▄▄▄▄▄▄▄▄▄▄▄▄{z}\
         {rb}{wf}▐▌{z}\
         {s}{rf}▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄{z}\
         {s}{sf}          {z}\n");

    // Row 4: full roof  (4 + 40 + 4 = 48)
    a += &format!(
        "  {s}{sf}    {z}\
         {rb}{rf}████████████████████████████████████████{z}\
         {s}{sf}    {z}\n");

    // Row 5: wall top + sign  (4 + 12 + 16 + 12 + 4 = 48)
    a += &format!(
        "  {s}{rf}▄▄▄▄{z}\
         {wb}{wf}████████████{z}\
         {wb}{gf}♦GREEN DRAGON♦{z}\
         {wb}{wf}██████████████{z}\
         {s}{rf}▄▄▄▄{z}\n");

    // Row 6: windows  (4 + 3 + 6 + 4 + 6 + 4 + 6 + 4 + 6 + 1 + 4 = 48)
    a += &format!(
        "  {wb}{wf}████{z}\
         {wb}{wf}███{z}\
         {wb}{gf}▐▓▓▓▓▌{z}\
         {wb}{wf}████{z}\
         {wb}{gf}▐▓▓▓▓▌{z}\
         {wb}{wf}████{z}\
         {wb}{gf}▐▓▓▓▓▌{z}\
         {wb}{wf}████{z}\
         {wb}{gf}▐▓▓▓▓▌{z}\
         {wb}{wf}█{z}\
         {wb}{wf}████{z}\n");

    // Row 7: windows + door top  (4 + 3 + 6 + 3 + 8 + 3 + 6 + 3 + 6 + 2 + 4 = 48)
    a += &format!(
        "  {wb}{wf}████{z}\
         {wb}{wf}███{z}\
         {wb}{gf}▐▓▓▓▓▌{z}\
         {wb}{wf}███{z}\
         {wb}{wh}╔══════╗{z}\
         {wb}{wf}███{z}\
         {wb}{gf}▐▓▓▓▓▌{z}\
         {wb}{wf}███{z}\
         {wb}{gf}▐▓▓▓▓▌{z}\
         {wb}{wf}██{z}\
         {wb}{wf}████{z}\n");

    // Row 8: windows + door  (4 + 3 + 6 + 3 + 8 + 3 + 6 + 3 + 6 + 2 + 4 = 48)
    a += &format!(
        "  {wb}{wf}████{z}\
         {wb}{wf}███{z}\
         {wb}{gf}▐▓▓▓▓▌{z}\
         {wb}{wf}███{z}\
         {db}{wh}║░░░░░░║{z}\
         {wb}{wf}███{z}\
         {wb}{gf}▐▓▓▓▓▌{z}\
         {wb}{wf}███{z}\
         {wb}{gf}▐▓▓▓▓▌{z}\
         {wb}{wf}██{z}\
         {wb}{wf}████{z}\n");

    // Row 9: barrels + door  (4 + 2 + 1 + 4 + 1 + 3 + 8 + 3 + 1 + 4 + 1 + 12 + 4 = 48)
    a += &format!(
        "  {wb}{wf}████{z}\
         {wb}{wf}██{z}\
         {wb}{df}◯{z}\
         {wb}{wf}████{z}\
         {wb}{df}◯{z}\
         {wb}{wf}███{z}\
         {db}{wh}║░░░░░░║{z}\
         {wb}{wf}███{z}\
         {wb}{df}◯{z}\
         {wb}{wf}████{z}\
         {wb}{df}◯{z}\
         {wb}{wf}████████████{z}\
         {wb}{wf}████{z}\n");

    // Row 10: cobblestones  (48)
    a += &format!(
        "  {cb}{cf}░░▒░░▒░░▒░░▒░░▒░░▒░░▒░░▒░░▒░░▒░░▒░░▒░░▒░░▒░░▒░░▒{z}\n");

    // Row 11: grass  (48)
    a += &format!(
        "  {gnb}{gnf}████████████████████████████████████████████████{z}\n");

    a
}

fn the_hill() -> String {
    // Panoramic view from a grassy hilltop overlooking the Shire.
    // Wide blue sky, distant rolling green hills with tiny hobbit-hole
    // doors, a signpost, wildflowers, and paths leading away.
    //
    // Every row: 2-space indent + 48 art chars = 50 visible chars.

    let s   = BG_CYAN;          // sky bg
    let sf  = FG_CYAN;          // sky fg (invisible filler)
    let cl  = FG_BRIGHT_WHITE;  // cloud fg
    let su  = FG_BRIGHT_YELLOW; // sun fg
    let g   = BG_GREEN;         // grass bg
    let gf  = FG_GREEN;         // grass fg
    let bgf = FG_BRIGHT_GREEN;  // bright grass fg
    let pf  = FG_BROWN;         // path fg
    let rf  = FG_RED;           // flower fg
    let yf  = FG_YELLOW;        // yellow flower / hobbit door fg
    let bf  = FG_BLACK;         // dark details fg (birds)
    let wf  = FG_BRIGHT_WHITE;  // signpost text fg
    let z   = Z;

    let mut a = String::with_capacity(2048);

    // Row 1: Sky with sun  (3 + 45 = 48)
    a += &format!(
        "  {s}{su} ☀ {z}\
         {s}{sf}                                             {z}\n");

    // Row 2: Clouds + birds  (10 + 6 + 8 + 1 + 3 + 1 + 7 + 4 + 8 = 48)
    a += &format!(
        "  {s}{sf}          {z}\
         {s}{cl} ░░░░ {z}\
         {s}{sf}        {z}\
         {s}{bf}v{z}\
         {s}{sf}   {z}\
         {s}{bf}v{z}\
         {s}{sf}       {z}\
         {s}{cl}░░░░{z}\
         {s}{sf}        {z}\n");

    // Row 3: Distant hill silhouettes  (4 + 6 + 4 + 16 + 4 + 6 + 8 = 48)
    a += &format!(
        "  {s}{sf}    {z}\
         {s}{gf}▄▄▄▄▄▄{z}\
         {s}{sf}    {z}\
         {s}{gf}▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄{z}\
         {s}{sf}    {z}\
         {s}{gf}▄▄▄▄▄▄{z}\
         {s}{sf}        {z}\n");

    // Row 4: Hills filled, tiny hobbit doors  (4 + 2 + 12 + 1 + 8 + 1 + 4 + 1 + 7 + 2 + 6 = 48)
    a += &format!(
        "  {g}{gf}████{z}\
         {s}{gf}▄▄{z}\
         {g}{gf}████████████{z}\
         {g}{yf}◦{z}\
         {g}{gf}████████{z}\
         {g}{yf}◦{z}\
         {g}{gf}████{z}\
         {g}{yf}◦{z}\
         {g}{gf}███████{z}\
         {s}{gf}▄▄{z}\
         {g}{gf}██████{z}\n");

    // Row 5: Shire fields, hedgerows  (8 + 3 + 14 + 3 + 6 + 3 + 11 = 48)
    a += &format!(
        "  {g}{gf}████████{z}\
         {g}{bgf}▓▓▓{z}\
         {g}{gf}██████████████{z}\
         {g}{bgf}▓▓▓{z}\
         {g}{gf}██████{z}\
         {g}{bgf}▓▓▓{z}\
         {g}{gf}███████████{z}\n");

    // Row 6: Closer green rolling hills  (48)
    a += &format!(
        "  {g}{gf}████████████████████████████████████████████████{z}\n");

    // Row 7: Hilltop with signpost  (18 + 1 + 3 + 1 + 25 = 48)
    a += &format!(
        "  {g}{gf}██████████████████{z}\
         {g}{pf}│{z}\
         {g}{wf}E→S{z}\
         {g}{pf}│{z}\
         {g}{gf}█████████████████████████{z}\n");

    // Row 8: Signpost base + wildflowers  (18 + 1 + 3 + 1 + 5 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 13 = 48)
    a += &format!(
        "  {g}{gf}██████████████████{z}\
         {g}{pf}╧{z}\
         {g}{gf}███{z}\
         {g}{pf}╧{z}\
         {g}{gf}█████{z}\
         {g}{rf}✿{z}\
         {g}{gf}█{z}\
         {g}{yf}✿{z}\
         {g}{gf}█{z}\
         {g}{rf}✿{z}\
         {g}{gf}█{z}\
         {g}{yf}✿{z}\
         {g}{gf}█████████████{z}\n");

    // Row 9: Path starts forking  (15 + 18 + 15 = 48)
    a += &format!(
        "  {g}{gf}███████████████{z}\
         {g}{pf}░░░░░░░░░░░░░░░░░░{z}\
         {g}{gf}███████████████{z}\n");

    // Row 10: Path fork - south + east  (6 + 6 + 6 + 12 + 6 + 6 + 6 = 48)
    a += &format!(
        "  {g}{gf}██████{z}\
         {g}{pf}░░░░░░{z}\
         {g}{gf}██████{z}\
         {g}{pf}░░░░░░░░░░░░{z}\
         {g}{gf}██████{z}\
         {g}{pf}░░░░░░{z}\
         {g}{gf}██████{z}\n");

    // Row 11: Paths continuing away  (4 + 8 + 4 + 16 + 4 + 8 + 4 = 48)
    a += &format!(
        "  {g}{gf}████{z}\
         {g}{pf}░░░░░░░░{z}\
         {g}{gf}████{z}\
         {g}{gf}████████████████{z}\
         {g}{gf}████{z}\
         {g}{pf}░░░░░░░░{z}\
         {g}{gf}████{z}\n");

    a
}

// -- Tests -------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    /// Count visible characters in a string, ignoring ANSI escape sequences.
    fn visible_width(s: &str) -> usize {
        let mut width = 0;
        let mut in_escape = false;
        for ch in s.chars() {
            if in_escape {
                if ch.is_ascii_alphabetic() {
                    in_escape = false;
                }
            } else if ch == '\x1b' {
                in_escape = true;
            } else if ch != '\n' && ch != '\x07' {
                width += 1;
            }
        }
        width
    }

    #[test]
    fn green_dragon_is_rectangle() {
        let art = green_dragon();
        let rows: Vec<&str> = art.lines().collect();
        assert!(!rows.is_empty(), "Art should have rows");
        let expected = 50;
        for (i, row) in rows.iter().enumerate() {
            let w = visible_width(row);
            assert_eq!(
                w, expected,
                "Row {} has visible width {} but expected {}:\n  {:?}",
                i + 1, w, expected, row
            );
        }
    }

    #[test]
    fn the_hill_is_rectangle() {
        let art = the_hill();
        let rows: Vec<&str> = art.lines().collect();
        assert!(!rows.is_empty(), "Art should have rows");
        let expected = 50; // 2 indent + 48 art
        for (i, row) in rows.iter().enumerate() {
            let w = visible_width(row);
            assert_eq!(
                w, expected,
                "Row {} has visible width {} but expected {}:\n  {:?}",
                i + 1, w, expected, row
            );
        }
    }

    #[test]
    fn bag_end_is_rectangle() {
        let art = bag_end();
        let rows: Vec<&str> = art.lines().collect();
        assert!(!rows.is_empty(), "Art should have rows");
        let expected = 50; // 2 indent + 48 art
        for (i, row) in rows.iter().enumerate() {
            let w = visible_width(row);
            assert_eq!(
                w, expected,
                "Row {} has visible width {} but expected {}:\n  {:?}",
                i + 1, w, expected, row
            );
        }
    }
}
