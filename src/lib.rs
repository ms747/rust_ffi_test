use std::ffi::CString;
use std::os::raw::{c_void, c_char};

#[link(name = "raylib")]
extern "C" {
    fn InitWindow(width: i32, height: i32, title: *const c_char);
    fn CloseWindow();
    fn WindowShouldClose() -> bool;
    fn BeginDrawing();
    fn EndDrawing();
    fn DrawRectangle(posX: i32, posY: i32, width: i32, height: i32, color: Color);
    fn DrawLineEx(start_pos: Vector2, end_pos: Vector2, thick: f32, color: Color);
    fn ClearBackground(color: Color);
    fn LoadTexture(fileName: *const c_char) -> Texture2D;
    fn UnloadTexture(texture: Texture2D);
    fn IsKeyPressed(key: i32) -> bool;
    fn IsKeyReleased(key: i32) -> bool;
    fn GetMousePosition() -> Vector2;
    fn IsMouseButtonReleased(button: i32) -> bool;
    fn GetFontDefault() -> Font;
    fn MeasureTextEx(font: Font, text: *const c_char, fontSize: f32, spacing: f32) -> Vector2;
    fn DrawTextEx(
        font: Font,
        text: *const c_char,
        position: Vector2,
        fontSize: f32,
        spacing: f32,
        tint: Color,
    );
    fn DrawTexturePro(
        texture: Texture2D,
        source: Rectangle,
        dest: Rectangle,
        origin: Vector2,
        rotation: f32,
        tint: Color,
    );
    fn SetTraceLogLevel(logLevel: i32);
    fn SetTargetFPS(fps: i32);  
    fn SetTextureFilter(texture: Texture2D, filter: TextureFilter);
}

#[repr(i32)]
pub enum KeyboardKey {
    Q = 81,
    R = 82,
}

#[repr(i32)]
pub enum MouseKey {
    Left = 0,
    Right = 1,
    Middle = 2,
}

#[repr(i32)]
pub enum TraceLogLevel{
    All = 0,
    Trace,
    Debug,
    Info,
    Warning,
    Error,
    Fatal,
    None
}

#[repr(i32)]
enum TextureFilter {
    Point = 0,
    Bilinear,
    Trilinear,
    Anisotropic4x,
    Anisotropic8x,
    Anisotropic16x,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct Texture {
    id: u32,
    width: i32,
    height: i32,
    mipmaps: i32,
    format: i32,
}

type Texture2D = Texture;

#[repr(C)]
pub struct Image {
    pub data: *mut c_void,
    pub width: i32,
    pub height: i32,
    pub mipmaps: i32,
    pub format: i32,
}

#[repr(C)]
#[allow(non_snake_case)]
pub struct GlyphInfo {
    pub value: i32,
    pub offsetX: i32,
    pub offsetY: i32,
    pub advanceX: i32,
    pub image: Image,
}

#[repr(C)]
#[derive(Debug)]
#[allow(non_snake_case)]
pub struct Font {
    pub baseSize: i32,
    pub glyphCount: i32,
    pub glyphPadding: i32,
    pub texture: Texture2D,
    pub recs: *mut Rectangle,
    pub glyphs: *mut GlyphInfo,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct Rectangle {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
}

#[repr(C)]
#[derive(Debug)]
pub struct Vector2 {
    pub x: f32,
    pub y: f32,
}

impl Vector2 {
    pub fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }
}

#[repr(C)]
pub struct Color(u8, u8, u8, u8);

impl Color {
    pub const WHITE: Color = Color(255, 255, 255, 255);
    pub const BLACK: Color = Color(0, 0, 0, 255);
    pub const RAYWHITE: Color = Color(245, 245, 245, 255);
    pub const DARKGRAY: Color = Color(80, 80, 80, 255);

    pub fn new(r: u8, g: u8, b: u8, a: u8) -> Self {
        Color(r, g, b, a)
    }
}

impl Rectangle {
    pub fn new(x: f32, y: f32, width: f32, height: f32) -> Self {
        Self {
            x,
            y,
            width,
            height,
        }
    }
}

pub struct Brush {}

struct RayObj {}

pub struct Raylib<'a> {
    _obj_ref: &'a RayObj,
}

impl<'a> Drop for Raylib<'a> {
    fn drop(&mut self) {
        unsafe { CloseWindow() }
    }
}

impl Drop for Brush {
    fn drop(&mut self) {
        unsafe { EndDrawing() }
    }
}

impl<'a> Raylib<'a> {
    pub fn set_trace_log(level: TraceLogLevel) {
        unsafe { SetTraceLogLevel(level as i32) }
    }

    pub fn set_target_fps(&self, fps: i32) {
        unsafe { SetTargetFPS(fps) }
    }

    pub fn init(width: i32, height: i32, title: *const c_char) -> Raylib<'a> {
        // let title = CString::new(title).unwrap();
        // unsafe { InitWindow(width, height, title.as_ptr()) }
        unsafe { InitWindow(width, height, title) }
        Raylib {
            _obj_ref: &RayObj {},
        }
    }

    pub fn should_window_close(&self) -> bool {
        unsafe { WindowShouldClose() }
    }

    pub fn get_mouse_position(&self) -> Vector2 {
        unsafe { GetMousePosition() }
    }

    pub fn begin_drawing(&self) -> Brush {
        unsafe { BeginDrawing() }
        Brush {}
    }

    pub fn load_texture(&self, filename: &str) -> Texture2D {
        let filename = CString::new(filename).unwrap();
        unsafe { LoadTexture(filename.as_ptr()) }
    }

    pub fn unload_texture(&self, texture: Texture2D) {
        unsafe { UnloadTexture(texture) }
    }

    pub fn is_key_pressed(&self, key: KeyboardKey) -> bool {
        unsafe { IsKeyPressed(key as i32) }
    }

    pub fn is_key_released(&self, key: KeyboardKey) -> bool {
        unsafe { IsKeyReleased(key as i32) }
    }

    pub fn is_mouse_button_released(&self, button: MouseKey) -> bool {
        unsafe { IsMouseButtonReleased(button as i32) }
    }

    pub fn get_font_default(&self) -> Font {
        unsafe { GetFontDefault() }
    }

    pub fn measure_text_ex(&self, font: Font, text: &str, font_size: f32, spacing: f32) -> Vector2 {
        let text = CString::new(text).unwrap();
        unsafe { MeasureTextEx(font, text.as_ptr(), font_size, spacing) }
    }
}

impl Brush {
    pub fn clear_background(&self, color: Color) {
        unsafe { ClearBackground(color) }
    }

    pub fn draw_rectangle(&self, x: i32, y: i32, width: i32, height: i32, color: Color) {
        unsafe { DrawRectangle(x, y, width, height, color) }
    }

    pub fn draw_text_ex(
        &self,
        font: Font,
        text: &str,
        position: Vector2,
        font_size: f32,
        spacing: f32,
        tint: Color,
    ) {
        let text = CString::new(text).unwrap();
        unsafe { DrawTextEx(font, text.as_ptr(), position, font_size, spacing, tint) }
    }

    pub fn draw_texture_pro(
        &self,
        texture: Texture2D,
        source: Rectangle,
        dest: Rectangle,
        origin: Vector2,
        rotation: f32,
        tint: Color
    ) {
        unsafe { DrawTexturePro(texture, source, dest, origin, rotation, tint) }
    }

    pub fn draw_line_ex(&self, start_pos: Vector2, end_pos: Vector2, thick: f32, color: Color) {
        unsafe { DrawLineEx(start_pos, end_pos, thick, color) }
    }
}

impl Texture {

}
