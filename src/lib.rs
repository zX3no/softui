use std::{ops::Range, path::Path};
use window::*;

pub const FONT: &[u8] = include_bytes!("../fonts/JetBrainsMono.ttf");
pub const CHAR: char = 'g';

//https://freetype.org/freetype2/docs/glyphs/glyphs-3.html

// pub fn draw_buffer(buffer: &mut text_buffer::Buffer, atlas: &mut Atlas, canvas: &mut Canvas) {
//     let mut y = atlas.font_size as usize;
//     for line in buffer.as_str().lines() {
//         atlas.draw_text(canvas, line, 0, y);
//         y += atlas.font_size as usize;
//     }
// }

pub struct Buffer {
    //TODO: Swap to mmap.
    pub text: String,
    //TODO: Scrolling.
    pub window: Range<usize>,
}

impl Buffer {
    pub fn new<P: AsRef<Path>>(path: P) -> Self {
        Self {
            text: std::fs::read_to_string(path).unwrap(),
            window: 0..100,
        }
    }

    pub fn draw(&self, atlas: &mut Atlas, canvas: &mut Canvas) {
        //TODO: How do I draw only part of a line?
        //Maybe this can help? https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-scrollwindow
        //https://github.com/rxi/lite/blob/38bd9b3326c02e43f244623f97a622b11f074415/data/core/view.lua#L20C21-L20C21
        let mut y = atlas.font_size as usize;
        // let mut y = atlas.font_size as usize - 10;
        for line in self.text.lines() {
            atlas.draw_text(canvas, line, 0, y);
            y += atlas.font_size as usize;
        }
    }
}

pub struct Atlas {
    pub glyphs: [(fontdue::Metrics, Vec<u8>); 128],
    pub font_size: f32,
}

impl Atlas {
    pub fn new(font_size: f32) -> Self {
        let font = fontdue::Font::from_bytes(FONT, fontdue::FontSettings::default()).unwrap();

        #[allow(invalid_value)]
        let mut glyphs: [(fontdue::Metrics, Vec<u8>); 128] = unsafe { std::mem::zeroed() };

        // use std::mem::MaybeUninit;
        // let mut glyphs: [MaybeUninit<(fontdue::Metrics, Vec<u8>)>; 127] = MaybeUninit::uninit_array();

        //https://www.ascii-code.com/
        for char in 32..127u8 {
            let (metrics, bitmap) = font.rasterize(char as char, font_size);
            // glyphs[char as usize].write((metrics, bitmap));
            glyphs[char as usize] = (metrics, bitmap);
        }

        Self { glyphs, font_size }
    }

    //TODO: Allow the drawing text over multiple lines. Maybe draw text should return the y pos?
    //or maybe the buffer should just include all the text related code and the metrics should be static.
    //TODO: If the text is longer than canvas width it needs to be clipped.
    pub fn draw_text(&self, canvas: &mut Canvas, text: &str, x: usize, y: usize) {
        let mut glyph_x = x;

        for char in text.chars() {
            let (metrics, bitmap) = &self.glyphs[char as usize];
            let glyph_y = (y as f32
                - (metrics.height as f32 - metrics.advance_height)
                - metrics.ymin as f32) as usize;

            for y in 0..metrics.height {
                for x in 0..metrics.width {
                    let color = bitmap[x + y * metrics.width];
                    let i = 4 * (x + glyph_x + canvas.width * (y + glyph_y));

                    if i >= canvas.buffer.len() {
                        return;
                    }

                    canvas.buffer[i] = color;
                    canvas.buffer[i + 1] = color;
                    canvas.buffer[i + 2] = color;
                    canvas.buffer[i + 3] = 0;
                }
            }

            glyph_x += metrics.advance_width as usize;

            //TODO: Still not enough.
            if glyph_x >= canvas.width {
                return;
            }
        }
    }
}

pub fn fontdue_subpixel(canvas: &mut Canvas, x: usize, y: usize) {
    let font = fontdue::Font::from_bytes(FONT, fontdue::FontSettings::default()).unwrap();
    let (metrics, bitmap) = font.rasterize_subpixel(CHAR, 50.0);

    let start_x = x;
    let start_y = y;

    for y in 0..metrics.height {
        for x in 0..metrics.width {
            let i = ((start_y + y) * canvas.width + start_x + x) * 4;
            let j = (y * metrics.width + x) * 3;

            //Bitmap is BGR_ not RGB.
            canvas.buffer[i] = bitmap[j + 2];
            canvas.buffer[i + 1] = bitmap[j + 1];
            canvas.buffer[i + 2] = bitmap[j];
            canvas.buffer[i + 3] = 0;
        }
    }
}

pub struct Canvas {
    pub buffer: Vec<u8>,
    pub area: Rect,
    pub width: usize,
    pub height: usize,
    pub window: Window,
    pub context: *mut VOID,
    pub bitmap: BITMAPINFO,
}

impl Canvas {
    pub fn new(window: Window) -> Self {
        let context = unsafe { GetDC(window.hwnd) };
        let area = window.area();
        let width = area.width();
        let height = area.height();

        Self {
            window,
            context,
            area,
            width: width as usize,
            height: height as usize,
            buffer: vec![0; width as usize * height as usize * std::mem::size_of::<RGBQUAD>()],
            bitmap: create_bitmap(width, height),
        }
    }

    fn update_area(&mut self) {
        let area = self.window.area();
        if self.area != area {
            self.area = area;
            self.width = self.area.width() as usize;
            self.height = self.area.height() as usize;
            self.buffer.clear();
            self.buffer
                .resize(self.width * self.height * std::mem::size_of::<RGBQUAD>(), 0);
            self.bitmap = create_bitmap(self.width as i32, self.height as i32);
        }
    }

    pub fn draw(&mut self) {
        unsafe {
            self.update_area();
            StretchDIBits(
                self.context,
                0,
                0,
                self.width as i32,
                self.height as i32,
                0,
                0,
                self.width as i32,
                self.height as i32,
                self.buffer.as_mut_ptr() as *const VOID,
                &self.bitmap,
                0,
                SRCCOPY,
            );
        }
    }

    //This is essentially just a memset.
    pub fn fill(&mut self, color: u32) {
        self.buffer.fill(color as u8);
    }

    ///Note color order is BGR_. The last byte is reserved.
    pub fn draw_pixel(&mut self, x: usize, y: usize, color: u32) {
        let buffer = unsafe { self.buffer.align_to_mut::<u32>().1 };
        buffer[y * self.width + x] = color;
    }

    #[track_caller]
    pub fn draw_rectangle(&mut self, x: usize, y: usize, width: usize, height: usize, color: u32) {
        let buffer = unsafe { self.buffer.align_to_mut::<u32>().1 };
        let canvas_width = self.width;

        #[cfg(debug_assertions)]
        {
            let canvas_height = self.height;
            if x + width >= canvas_width {
                panic!("x: {x} + width: {width} cannot be >= to the canvas width: {canvas_width}");
            }
            if y + height >= canvas_height {
                panic!(
                    "y: {y} + height: {height} cannot be >= to the canvas height: {canvas_height}"
                );
            }
        }

        for i in y..y + height {
            let pos = x + canvas_width * i;
            for px in &mut buffer[pos..pos + width] {
                *px = color;
            }
        }
    }

    //TODO: Allow for variable length outlines.
    #[track_caller]
    pub fn draw_rectangle_outline(
        &mut self,
        x: usize,
        y: usize,
        width: usize,
        height: usize,
        color: u32,
    ) {
        let buffer = unsafe { self.buffer.align_to_mut::<u32>().1 };
        let canvas_width = self.width;

        #[cfg(debug_assertions)]
        {
            let canvas_height = self.height;
            if x + width >= canvas_width {
                panic!("x: {x} + width: {width} cannot be >= to the canvas width: {canvas_width}");
            }
            if y + height >= canvas_height {
                panic!(
                    "y: {y} + height: {height} cannot be >= to the canvas height: {canvas_height}"
                );
            }
        }

        for i in y..y + height {
            if i > y && i < (y + height).saturating_sub(1) {
                buffer[i * canvas_width + x] = color;
                buffer[(i * canvas_width) + x + width - 1] = color;
            } else {
                let pos = i * canvas_width + x;
                for px in &mut buffer[pos..pos + width] {
                    *px = color;
                }
            }
        }
    }
}
