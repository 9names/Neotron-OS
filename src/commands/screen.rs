//! Screen-related commands for Neotron OS

use crate::{print, println, Ctx, API, VGA_CONSOLE};

/// Called when the "clear" command is executed.
pub fn clear(_menu: &menu::Menu<Ctx>, _item: &menu::Item<Ctx>, _args: &[&str], _ctx: &mut Ctx) {
    if let Some(ref mut console) = unsafe { &mut VGA_CONSOLE } {
        console.clear();
    }
}

/// Called when the "fill" command is executed.
pub fn fill(_menu: &menu::Menu<Ctx>, _item: &menu::Item<Ctx>, _args: &[&str], _ctx: &mut Ctx) {
    if let Some(ref mut console) = unsafe { &mut VGA_CONSOLE } {
        console.clear();
    }
    let api = API.get();
    let mode = (api.video_get_mode)();
    let (Some(width), Some(height)) = (mode.text_width(), mode.text_height()) else {
        println!("Unable to get console size");
        return;
    };
    // A range of printable ASCII compatible characters
    let mut char_cycle = (' '..='~').cycle();
    // Scroll two screen fulls
    for _row in 0..height * 2 {
        for _col in 0..width {
            print!("{}", char_cycle.next().unwrap());
        }
    }
}
