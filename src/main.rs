// #![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use softui::*;
use window::*;

fn main() {
    // let mut ctx = Context::new("Softui", 800, 600);
    // unsafe { CTX = Some(Context::new("Softui", 800, 600)) };
    // let ctx = ctx();

    let ctx = create_ctx("Softui", 800, 600);

    // let mut size = 20;
    let font = fontdue::Font::from_bytes(FONT, fontdue::FontSettings::default()).unwrap();
    set_default_font(font);

    loop {
        match ctx.event() {
            Some(Event::Quit | Event::Input(Key::Escape, _)) => break,
            None => {}
            _ => {}
        }

        ctx.fill(Color::Black);

        {
            // text("test").y(20).draw();
            // empty((text("hi"), text("Tesing").y(30)));
            let str = "yipeee!\n1234567890\n!@#$%^&*()";
            let mut text = text(str)
                .font_size(32)
                .on_clicked(Left, |_| println!("Clicked text {:?}", ctx.area));
            // if text.clicked(Left) {
            //     println!("Clicked text {:?}", ctx.area);
            // }
            text.draw();
            // button().on_clicked(Left, |_| println!("hi"));
        }

        //Dragging example.
        if ctx.left_mouse.inital_position != Rect::default() {
            let inital = ctx.left_mouse.inital_position;
            let end = ctx.left_mouse.release_position.unwrap_or(ctx.mouse_pos);
            let mut drag = Rect::default();

            if end.x > inital.x {
                drag.x = inital.x;
                drag.width = end.x - inital.x;
            } else {
                drag.x = end.x;
                drag.width = inital.x - end.x;
            }

            if end.y > inital.y {
                drag.y = inital.y;
                drag.height = end.y - inital.y;
            } else {
                drag.y = end.y;
                drag.height = inital.y - end.y;
            }

            ctx.draw_rectangle(
                drag.x as usize,
                drag.y as usize,
                drag.width as usize,
                drag.height as usize,
                Color::Red.into(),
            )
            .unwrap();
        }

        // ctx.draw_circle(300, 30, 20, Color::Blue.into());

        // ctx.draw_rectangle_rounded(300, 300, 100, 50, 10, Color::White.into())
        //     .unwrap();
        ctx.draw_rectangle_rounded(300, 300, 300, 200, 50, Color::White.into())
            .unwrap();

        {
            //TODO: I'm not liking draw on drop.
            //It works for an immediate style of code but falls apart everywhere else.
            // ctx.vertical(|ctx| {
            //     if button(&ctx).clicked(Left) {
            //         println!("Clicked button im");
            //     };
            // });

            // v((
            //     button(&ctx)
            //         .wh(20)
            //         .on_clicked(Forward, |_| {
            //             if size >= 30 {
            //                 size = 20;
            //             } else {
            //                 size = 30;
            //             }
            //         })
            //         .on_clicked(Left, |_| {
            //             if size >= 30 {
            //                 size = 20;
            //             } else {
            //                 size = 40;
            //             }
            //         }),
            //     h((button(&ctx).wh(20), button(&ctx).wh(20))).p(10),
            //     h((
            //         button(&ctx).wh(size),
            //         button(&ctx).wh(size),
            //         button(&ctx).wh(size),
            //     ))
            //     .p(10),
            //     h((
            //         button(&ctx).wh(20),
            //         button(&ctx).wh(20),
            //         button(&ctx).wh(20),
            //         v((button(&ctx).w(20).h(8), button(&ctx).w(20).h(8))).p(4),
            //     ))
            //     .p(10),
            // ))
            // .p(10);
        }

        ctx.draw_frame();
    }
}
