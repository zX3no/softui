// #![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
#![allow(unused, static_mut_refs)]
use mini::defer_results;
// use softui::{create_ctx, text, v, Color, MouseButton::*, Text, Widget};
use softui::*;
use window::{Event, Key};

fn main() {
    defer_results!();
    let ctx = create_ctx("Softui", 800, 600);

    #[cfg(feature = "dwrite")]
    let dwrite = DWrite::new_cached(32.0);

    let font_size = 10;
    let font_size_large = 72;

    #[cfg(feature = "image")]
    let mut image = image("img/smol.png");

    let mut r = rect().bg(Color::BLACK).on_click(Left, |_| println!("hi"));

    loop {
        match ctx.event() {
            Some(Event::Quit | Event::Input(Key::Escape, _)) => break,
            _ => {}
        }

        ctx.fill(Color::WHITE);

        #[cfg(feature = "dwrite")]
        ctx.draw_text_subpixel("This is some text....", &dwrite, 0, 36 * 3, 10, 0, Color::WHITE);
        ctx.draw_text(
            "This is some text...",
            &default_font().unwrap(),
            0,
            36 * 3 + 10,
            10,
            0,
            Color::BLACK,
        );

        // flex!(
        //     Flex::TopLeft,
        //     Direction::Horizontal,
        //     ctx.area.width,
        //     ctx.area.height,
        //     r,
        //     r,
        //     r,
        //     r
        // );

        ctx.draw_frame();
    }
}
