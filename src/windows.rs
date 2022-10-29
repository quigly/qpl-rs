/* TODO:

- XInput controller support
- DirectInput controller support


*/

use super::*;

use libc::*;
use winapi::*;
use winapi::shared::basetsd::*;
use winapi::shared::minwindef::*;
use winapi::shared::windef::*;
use winapi::shared::windowsx::*;
use winapi::shared::winerror::*;
use winapi::um::errhandlingapi::*;
use winapi::um::libloaderapi::*;
use winapi::um::memoryapi::*;
use winapi::um::profileapi::*;
use winapi::um::synchapi::*;
use winapi::um::sysinfoapi::*;
use winapi::um::winbase::*;
use winapi::um::wingdi::*;
use winapi::um::winnt::*;
use winapi::um::winuser::*;
use winapi::um::xinput::*;

use std::ffi::CString;
use std::ffi::OsStr;
use std::os::windows::ffi::*;

use std::mem::size_of;

const KEYCODES: [Key; 256] =
[
    Key::Unknown, // 0 (0x00)
    Key::Unknown, // 1 (0x01)
    Key::Unknown, // 2 (0x02)
    Key::Unknown, // 3 (0x03)
    Key::Unknown, // 4 (0x04)
    Key::Unknown, // 5 (0x05)
    Key::Unknown, // 6 (0x06)
    Key::Unknown, // 7 (0x07)
    Key::Unknown, // 8 (0x08)
    Key::Unknown, // 9 (0x09)
    Key::Unknown, // 10 (0x0A)
    Key::Unknown, // 11 (0x0B)
    Key::Minus, // 12 (0x0C)
    Key::Equals, // 13 (0x0D)
    Key::Backspace, // 14 (0x0E)
    Key::Unknown, // 15 (0x0F)
    Key::Q, // 16 (0x10)
    Key::W, // 17 (0x11)
    Key::E, // 18 (0x12)
    Key::R, // 19 (0x13)
    Key::T, // 20 (0x14)
    Key::Y, // 21 (0x15)
    Key::U, // 22 (0x16)
    Key::I, // 23 (0x17)
    Key::O, // 24 (0x18)
    Key::P, // 25 (0x19)
    Key::LeftBracket, // 26 (0x1A)
    Key::RightBracket, // 27 (0x1B)
    Key::Unknown, // 28 (0x1C)
    Key::Unknown, // 29 (0x1D)
    Key::A, // 30 (0x1E)
    Key::S, // 31 (0x1F)
    Key::D, // 32 (0x20)
    Key::F, // 33 (0x21)
    Key::G, // 34 (0x22)
    Key::H, // 35 (0x23)
    Key::J, // 36 (0x24)
    Key::Unknown, // 37 (0x25)
    Key::L, // 38 (0x26)
    Key::Semicolon, // 39 (0x27)
    Key::Apostrophe, // 40 (0x28)
    Key::Grave, // 41 (0x29)
    Key::Unknown, // 42 (0x2A)
    Key::Backslash, // 43 (0x2B)
    Key::Z, // 44 (0x2C)
    Key::X, // 45 (0x2D)
    Key::C, // 46 (0x2E)
    Key::V, // 47 (0x2F)
    Key::B, // 48 (0x30)
    Key::N, // 49 (0x31)
    Key::M, // 50 (0x32)
    Key::Comma, // 51 (0x33)
    Key::Period, // 52 (0x34)
    Key::Slash, // 53 (0x35)
    Key::Unknown, // 54 (0x36)
    Key::Unknown, // 55 (0x37)
    Key::Unknown, // 56 (0x38)
    Key::Unknown, // 57 (0x39)
    Key::Unknown, // 58 (0x3A)
    Key::Unknown, // 59 (0x3B)
    Key::Unknown, // 60 (0x3C)
    Key::Unknown, // 61 (0x3D)
    Key::Unknown, // 62 (0x3E)
    Key::Unknown, // 63 (0x3F)
    Key::Unknown, // 64 (0x40)
    Key::Unknown, // 65 (0x41)
    Key::Unknown, // 66 (0x42)
    Key::Unknown, // 67 (0x43)
    Key::Unknown, // 68 (0x44)
    Key::Unknown, // 69 (0x45)
    Key::Unknown, // 70 (0x46)
    Key::Unknown, // 71 (0x47)
    Key::Unknown, // 72 (0x48)
    Key::Unknown, // 73 (0x49)
    Key::Unknown, // 74 (0x4A)
    Key::Unknown, // 75 (0x4B)
    Key::Unknown, // 76 (0x4C)
    Key::Unknown, // 77 (0x4D)
    Key::Unknown, // 78 (0x4E)
    Key::Unknown, // 79 (0x4F)
    Key::Unknown, // 80 (0x50)
    Key::Unknown, // 81 (0x51)
    Key::Unknown, // 82 (0x52)
    Key::Unknown, // 83 (0x53)
    Key::Unknown, // 84 (0x54)
    Key::Unknown, // 85 (0x55)
    Key::Unknown, // 86 (0x56) WORLD_2
    Key::Unknown, // 87 (0x57)
    Key::Unknown, // 88 (0x58)
    Key::Unknown, // 89 (0x59)
    Key::Unknown, // 90 (0x5A)
    Key::Unknown, // 91 (0x5B)
    Key::Unknown, // 92 (0x5C)
    Key::Unknown, // 93 (0x5D)
    Key::Unknown, // 94 (0x5E)
    Key::Unknown, // 95 (0x5F)
    Key::Unknown, // 96 (0x60)
    Key::Unknown, // 97 (0x61)
    Key::Unknown, // 98 (0x62)
    Key::Unknown, // 99 (0x63)
    Key::Unknown, // 100 (0x64)
    Key::Unknown, // 101 (0x65)
    Key::Unknown, // 102 (0x66)
    Key::Unknown, // 103 (0x67)
    Key::Unknown, // 104 (0x68)
    Key::Unknown, // 105 (0x69)
    Key::Unknown, // 106 (0x6A)
    Key::Unknown, // 107 (0x6B)
    Key::Unknown, // 108 (0x6C)
    Key::Unknown, // 109 (0x6D)
    Key::Unknown, // 110 (0x6E)
    Key::Unknown, // 111 (0x6F)
    Key::Unknown, // 112 (0x70)
    Key::Unknown, // 113 (0x71)
    Key::Unknown, // 114 (0x72)
    Key::Unknown, // 115 (0x73)
    Key::Unknown, // 116 (0x74)
    Key::Unknown, // 117 (0x75)
    Key::Unknown, // 118 (0x76)
    Key::Unknown, // 119 (0x77)
    Key::Unknown, // 120 (0x78)
    Key::Unknown, // 121 (0x79)
    Key::Unknown, // 122 (0x7A)
    Key::Unknown, // 123 (0x7B)
    Key::Unknown, // 124 (0x7C)
    Key::Unknown, // 125 (0x7D)
    Key::Unknown, // 126 (0x7E)
    Key::Unknown, // 127 (0x7F)
    Key::Unknown, // 128 (0x80)
    Key::Unknown, // 129 (0x81)
    Key::Unknown, // 130 (0x82)
    Key::Unknown, // 131 (0x83)
    Key::Unknown, // 132 (0x84)
    Key::Unknown, // 133 (0x85)
    Key::Unknown, // 134 (0x86)
    Key::Unknown, // 135 (0x87)
    Key::Unknown, // 136 (0x88)
    Key::Unknown, // 137 (0x89)
    Key::Unknown, // 138 (0x8A)
    Key::Unknown, // 139 (0x8B)
    Key::Unknown, // 140 (0x8C)
    Key::Unknown, // 141 (0x8D)
    Key::Unknown, // 142 (0x8E)
    Key::Unknown, // 143 (0x8F)
    Key::Unknown, // 144 (0x90)
    Key::Unknown, // 145 (0x91)
    Key::Unknown, // 146 (0x92)
    Key::Unknown, // 147 (0x93)
    Key::Unknown, // 148 (0x94)
    Key::Unknown, // 149 (0x95)
    Key::Unknown, // 150 (0x96)
    Key::Unknown, // 151 (0x97)
    Key::Unknown, // 152 (0x98)
    Key::Unknown, // 153 (0x99)
    Key::Unknown, // 154 (0x9A)
    Key::Unknown, // 155 (0x9B)
    Key::Unknown, // 156 (0x9C)
    Key::Unknown, // 157 (0x9D)
    Key::Unknown, // 158 (0x9E)
    Key::Unknown, // 159 (0x9F)
    Key::Unknown, // 160 (0xA0)
    Key::Unknown, // 161 (0xA1)
    Key::Unknown, // 162 (0xA2)
    Key::Unknown, // 163 (0xA3)
    Key::Unknown, // 164 (0xA4)
    Key::Unknown, // 165 (0xA5)
    Key::Unknown, // 166 (0xA6)
    Key::Unknown, // 167 (0xA7)
    Key::Unknown, // 168 (0xA8)
    Key::Unknown, // 169 (0xA9)
    Key::Unknown, // 170 (0xAA)
    Key::Unknown, // 171 (0xAB)
    Key::Unknown, // 172 (0xAC)
    Key::Unknown, // 173 (0xAD)
    Key::Unknown, // 174 (0xAE)
    Key::Unknown, // 175 (0xAF)
    Key::Unknown, // 176 (0xB0)
    Key::Unknown, // 177 (0xB1)
    Key::Unknown, // 178 (0xB2)
    Key::Unknown, // 179 (0xB3)
    Key::Unknown, // 180 (0xB4)
    Key::Unknown, // 181 (0xB5)
    Key::Unknown, // 182 (0xB6)
    Key::Unknown, // 183 (0xB7)
    Key::Unknown, // 184 (0xB8)
    Key::Unknown, // 185 (0xB9)
    Key::Unknown, // 186 (0xBA)
    Key::Unknown, // 187 (0xBB)
    Key::Unknown, // 188 (0xBC)
    Key::Unknown, // 189 (0xBD)
    Key::Unknown, // 190 (0xBE)
    Key::Unknown, // 191 (0xBF)
    Key::Unknown, // 192 (0xC0)
    Key::Unknown, // 193 (0xC1)
    Key::Unknown, // 194 (0xC2)
    Key::Unknown, // 195 (0xC3)
    Key::Unknown, // 196 (0xC4)
    Key::Unknown, // 197 (0xC5)
    Key::Unknown, // 198 (0xC6)
    Key::Unknown, // 199 (0xC7)
    Key::Unknown, // 200 (0xC8)
    Key::Unknown, // 201 (0xC9)
    Key::Unknown, // 202 (0xCA)
    Key::Unknown, // 203 (0xCB)
    Key::Unknown, // 204 (0xCC)
    Key::Unknown, // 205 (0xCD)
    Key::Unknown, // 206 (0xCE)
    Key::Unknown, // 207 (0xCF)
    Key::Unknown, // 208 (0xD0)
    Key::Unknown, // 209 (0xD1)
    Key::Unknown, // 210 (0xD2)
    Key::Unknown, // 211 (0xD3)
    Key::Unknown, // 212 (0xD4)
    Key::Unknown, // 213 (0xD5)
    Key::Unknown, // 214 (0xD6)
    Key::Unknown, // 215 (0xD7)
    Key::Unknown, // 216 (0xD8)
    Key::Unknown, // 217 (0xD9)
    Key::Unknown, // 218 (0xDA)
    Key::Unknown, // 219 (0xDB)
    Key::Unknown, // 220 (0xDC)
    Key::Unknown, // 221 (0xDD)
    Key::Unknown, // 222 (0xDE)
    Key::Unknown, // 223 (0xDF)
    Key::Unknown, // 224 (0xE0)
    Key::Unknown, // 225 (0xE1)
    Key::Unknown, // 226 (0xE2)
    Key::Unknown, // 227 (0xE3)
    Key::Unknown, // 228 (0xE4)
    Key::Unknown, // 229 (0xE5)
    Key::Unknown, // 230 (0xE6)
    Key::Unknown, // 231 (0xE7)
    Key::Unknown, // 232 (0xE8)
    Key::Unknown, // 233 (0xE9)
    Key::Unknown, // 234 (0xEA)
    Key::Unknown, // 235 (0xEB)
    Key::Unknown, // 236 (0xEC)
    Key::Unknown, // 237 (0xED)
    Key::Unknown, // 238 (0xEE)
    Key::Unknown, // 239 (0xEF)
    Key::Unknown, // 240 (0xF0)
    Key::Unknown, // 241 (0xF1)
    Key::Unknown, // 242 (0xF2)
    Key::Unknown, // 243 (0xF3)
    Key::Unknown, // 244 (0xF4)
    Key::Unknown, // 245 (0xF5)
    Key::Unknown, // 246 (0xF6)
    Key::Unknown, // 247 (0xF7)
    Key::Unknown, // 248 (0xF8)
    Key::Unknown, // 249 (0xF9)
    Key::Unknown, // 250 (0xFA)
    Key::Unknown, // 251 (0xFB)
    Key::Unknown, // 252 (0xFC)
    Key::Unknown, // 253 (0xFD)
    Key::Unknown, // 254 (0xFE)
    Key::Unknown // 255 (0xFF)
];

fn win32_convert_scancode_to_key(scancode: u32) -> Option<Key>
{
    match scancode
    {
        0x00B => Some(Key::Num0),
        0x002 => Some(Key::Num1),
        0x003 => Some(Key::Num2),
        0x004 => Some(Key::Num3),
        0x005 => Some(Key::Num4),
        0x006 => Some(Key::Num5),
        0x007 => Some(Key::Num6),
        0x008 => Some(Key::Num7),
        0x009 => Some(Key::Num8),
        0x00A => Some(Key::Num9),
        0x01E => Some(Key::A),
        0x030 => Some(Key::B),
        0x02E => Some(Key::C),
        0x020 => Some(Key::D),
        0x012 => Some(Key::E),
        0x021 => Some(Key::F),
        0x022 => Some(Key::G),
        0x023 => Some(Key::H),
        0x017 => Some(Key::I),
        0x024 => Some(Key::J),
        0x025 => Some(Key::K),
        0x026 => Some(Key::L),
        0x032 => Some(Key::M),
        0x031 => Some(Key::N),
        0x018 => Some(Key::O),
        0x019 => Some(Key::P),
        0x010 => Some(Key::Q),
        0x013 => Some(Key::R),
        0x01F => Some(Key::S),
        0x014 => Some(Key::T),
        0x016 => Some(Key::U),
        0x02F => Some(Key::V),
        0x011 => Some(Key::W),
        0x02D => Some(Key::X),
        0x015 => Some(Key::Y),
        0x02C => Some(Key::Z),

        0x028 => Some(Key::Apostrophe),
        0x02B => Some(Key::Backslash),
        0x033 => Some(Key::Comma),
        0x00D => Some(Key::Equals),
        0x029 => Some(Key::Grave),
        0x01A => Some(Key::LeftBracket),
        0x00C => Some(Key::Minus),
        0x034 => Some(Key::Period),
        0x01B => Some(Key::RightBracket),
        0x027 => Some(Key::Semicolon),
        0x035 => Some(Key::Slash),
        //0x056 => Some(Key::WORLD_2),

        0x00E => Some(Key::Backspace),
        0x153 => Some(Key::Delete),
        0x14F => Some(Key::End),
        0x01C => Some(Key::Return),
        0x001 => Some(Key::Escape),
        0x147 => Some(Key::Home),
        0x152 => Some(Key::Insert),
        0x15D => Some(Key::Menu),
        0x151 => Some(Key::PageDown),
        0x149 => Some(Key::PageUp),
        0x045 => Some(Key::Pause),
        0x039 => Some(Key::Space),
        0x00F => Some(Key::Tab),
        0x03A => Some(Key::CapsLock),
        0x145 => Some(Key::NumLock),
        0x046 => Some(Key::ScrollLock),
        0x03B => Some(Key::F1),
        0x03C => Some(Key::F2),
        0x03D => Some(Key::F3),
        0x03E => Some(Key::F4),
        0x03F => Some(Key::F5),
        0x040 => Some(Key::F6),
        0x041 => Some(Key::F7),
        0x042 => Some(Key::F8),
        0x043 => Some(Key::F9),
        0x044 => Some(Key::F10),
        0x057 => Some(Key::F11),
        0x058 => Some(Key::F12),
        //0x064 => Some(Key::F13),
        //0x065 => Some(Key::F14),
        //0x066 => Some(Key::F15),
        //0x067 => Some(Key::F16),
        //0x068 => Some(Key::F17),
        //0x069 => Some(Key::F18),
        //0x06A => Some(Key::F19),
        //0x06B => Some(Key::F20),
        //0x06C => Some(Key::F21),
        //0x06D => Some(Key::F22),
        //0x06E => Some(Key::F23),
        //0x076 => Some(Key::F24),
        0x038 => Some(Key::Alt),
        0x01D => Some(Key::Ctrl),
        0x02A => Some(Key::Shift),
        0x15B => Some(Key::Gui),
        0x137 => Some(Key::PrintScreen),
        0x138 => Some(Key::Alt),
        0x11D => Some(Key::Ctrl),
        0x036 => Some(Key::Shift),
        0x15C => Some(Key::Gui),
        0x150 => Some(Key::Down),
        0x14B => Some(Key::Left),
        0x14D => Some(Key::Right),
        0x148 => Some(Key::Up),

        0x052 => Some(Key::Num0),
        0x04F => Some(Key::Num1),
        0x050 => Some(Key::Num2),
        0x051 => Some(Key::Num3),
        0x04B => Some(Key::Num4),
        0x04C => Some(Key::Num5),
        0x04D => Some(Key::Num6),
        0x047 => Some(Key::Num7),
        0x048 => Some(Key::Num8),
        0x049 => Some(Key::Num9),
        //0x04E => Some(Key::KP_ADD),
        //0x053 => Some(Key::KP_DECIMAL),
        //0x135 => Some(Key::KP_DIVIDE),
        0x11C => Some(Key::Return),
        0x059 => Some(Key::Equals),
        //0x037 => Some(Key::KP_MULTIPLY),
        //0x04A => Some(Key::KP_SUBTRACT),

        _ => None
    }
}

/* OpenGL */

type WglCreateContextAttribsARB = extern "system" fn(HDC, HGLRC, *const i32) -> HGLRC;

const WGL_CONTEXT_MAJOR_VERSION_ARB: i32 = 0x2091;
const WGL_CONTEXT_MINOR_VERSION_ARB: i32 = 0x2092;
const WGL_CONTEXT_PROFILE_MASK_ARB: i32 = 0x9126;

const WGL_CONTEXT_CORE_PROFILE_BIT_ARB: i32 = 0x00000001;
const WGL_CONTEXT_COMPATIBILITY_PROFILE_BIT_ARB: i32 = 0x00000002;

const WGL_DRAW_TO_WINDOW_ARB: i32 = 0x2001;
const WGL_ACCELERATION_ARB: i32 = 0x2003;
const WGL_SUPPORT_OPENGL_ARB: i32 = 0x2010;
const WGL_DOUBLE_BUFFER_ARB: i32 = 0x2011;
const WGL_PIXEL_TYPE_ARB: i32 = 0x2013;
const WGL_RED_BITS_ARB: i32 = 0x2015;
const WGL_GREEN_BITS_ARB: i32 = 0x2017;
const WGL_BLUE_BITS_ARB: i32 = 0x2019;
const WGL_ALPHA_BITS_ARB: i32 = 0x201B;
const WGL_DEPTH_BITS_ARB: i32 = 0x2022;
const WGL_STENCIL_BITS_ARB: i32 = 0x2023;

const WGL_FULL_ACCELERATION_ARB: i32 = 0x2027;
const WGL_TYPE_RGBA_ARB: i32 = 0x202B;

type WglChoosePixelFormatARB = extern "system" fn(HDC, *const i32, *const f32, u32, *mut i32, *mut u32) -> i32;

const WGL_SAMPLE_BUFFERS_ARB: i32 = 0x2041;
const WGL_SAMPLES_ARB: i32 = 0x2042;

const WGL_FRAMEBUFFER_SRGB_CAPABLE_ARB: i32 = 0x20A9;

type WglSwapIntervalEXT = extern "system" fn(i32) -> i32;

extern "system" fn gl_debug_message_callback(
	source: u32, error_type: u32,
	id: u32, severity: u32,
	length: i32, message: *const i8,
	user_param: *mut libc::c_void)
{
	let message_str = unsafe { std::ffi::CStr::from_ptr(message) }.to_str().unwrap();
	println!("GL CALLBACK: {} type = 0x{:X}, severity = 0x{:X}, message = {}",
		if error_type == gl::DEBUG_TYPE_ERROR { "** GL ERROR **" } else { "" },
		error_type, severity, message_str);
}

pub struct GLContext
{
    hdc: HDC,
    hglrc: HGLRC,
    gl_library: HMODULE
}

/* Internal functions */

fn win32_to_wstring(str: &str) -> Vec<u16>
{
    OsStr::new(str).encode_wide().chain(Some(0).into_iter()).collect()
}

fn win32_decode_wstring(mut wide_c_string: &[u16]) -> std::ffi::OsString
{
    if let Some(null_pos) = wide_c_string.iter().position(|c| *c == 0)
    {
        wide_c_string = &wide_c_string[..null_pos];
    }

    std::ffi::OsString::from_wide(wide_c_string)
}

fn win32_get_last_error() -> u32
{
    unsafe { GetLastError() }
}

fn win32_register_class(name: &str, style: u32) -> Vec<u16>
{
	let class_name = win32_to_wstring(name);

	unsafe
	{
		let wnd = WNDCLASSW
		{
			style,
			lpfnWndProc: Some(win32_window_proc),
			cbClsExtra: 0,
			cbWndExtra: 0,
			hInstance: 0 as HINSTANCE,
			hIcon: LoadIconW(0 as HINSTANCE, IDI_APPLICATION),
			hCursor: LoadCursorW(0 as HINSTANCE, IDI_APPLICATION),
			hbrBackground: 16 as HBRUSH,
			lpszMenuName: 0 as LPCWSTR,
			lpszClassName: class_name.as_ptr(),
		};

		RegisterClassW(&wnd);
	}

	class_name
}

unsafe extern "system" fn win32_window_proc(h_wnd: HWND, msg: UINT, w_param: WPARAM, l_param: LPARAM) -> LRESULT
{
    if msg == WM_DESTROY
	{
		//println!("window_proc WM_DESTROY");
        PostQuitMessage(0);
		return 0;
    }
    else if msg == WM_CLOSE
	{
		//println!("window_proc WM_CLOSE");
		DestroyWindow(h_wnd);
		return 0;
	}
    else if msg == WM_QUIT
	{
		//println!("window_proc WM_QUIT");
		return 0;
	}
    else if msg == WM_INPUT
    {
        //println!("WM_INPUT");

        /*if let Some(data) = win32_get_raw_input_data(l_param as _)
        {
            println!("win32_window_proc WM_INPUT");
        }*/
    }

    return DefWindowProcW(h_wnd, msg, w_param, l_param);
}

/* Window-specific functions */

pub struct Window
{
    /* Public data */
    pub width: u32,
    pub height: u32,
    pub title: String,
    pub should_close: bool,
    pub events: Vec<Event>,

    /* Internals */
    resizable: bool,
    mode: WindowMode,
    key_modifiers: KeyModifiers,
    last_mouse_x: i32,
    last_mouse_y: i32,
    mouse_tracked: bool,
    keys_current: [bool; 256],
    keys_previous: [bool; 256],

    /* Platform-specific data */
    hinstance: HINSTANCE,
    handle: HWND
}

impl Window
{
    pub fn update_input_state(&mut self)
    {
        for i in 0..self.keys_current.len()
        {
            self.keys_previous[i] = self.keys_current[i];
        }
    }

    pub fn poll_events(&mut self) -> Option<Event>
    {
        let mut event: Option<Event> = None;

        unsafe
        {
            let mut msg = MSG
            {
                hwnd: self.handle,
                message: 0 as UINT,
                wParam: 0 as WPARAM,
                lParam: 0 as LPARAM,
                time: 0 as DWORD,
                pt: POINT { x: 0, y: 0, }
            };

            if PeekMessageW(&mut msg, 0 as *mut HWND__, 0, 0, PM_REMOVE) == TRUE
            {
                match msg.message
                {
                    WM_QUIT =>
                    {
                        event = Some(Event::Quit);
                    },
                    WM_KEYDOWN | WM_KEYUP =>
                    {
                        let scancode: u16 = HIWORD(msg.lParam as DWORD) & (KF_EXTENDED | 0xFF);
                        let key: Key = win32_convert_scancode_to_key(scancode as u32).unwrap_or(Key::Unknown);
                        let state = if msg.message == WM_KEYDOWN { 1 } else { 0 };
                        
                        if msg.wParam == VK_SHIFT as _ { self.key_modifiers.shift = state == 1; }
                        if msg.wParam == VK_CONTROL as _ { self.key_modifiers.ctrl = state == 1; }
                        if msg.wParam == VK_MENU as _ { self.key_modifiers.alt = state == 1; }
                        if msg.wParam == VK_LWIN as _ || msg.wParam == VK_RWIN as _ { self.key_modifiers.gui = state == 1; }
                        if msg.wParam == VK_CAPITAL as _ { self.key_modifiers.caps = state == 1; }
                        let modifiers: KeyModifiers = self.key_modifiers;

                        self.keys_current[key as usize] = if state == 1 { true } else { false };
                        
                        event = Some(Event::Key { key, state, modifiers });
                    },
                    WM_MOUSEMOVE =>
                    {
                        let x: i32 = GET_X_LPARAM(msg.lParam);
                        let y: i32 = GET_Y_LPARAM(msg.lParam);
                        let xrel: i32 = x - self.last_mouse_x;
                        let yrel: i32 = y - self.last_mouse_y;

                        self.last_mouse_x = x;
                        self.last_mouse_y = y;

                        self.mouse_tracked = true;

                        event = Some(Event::MouseMotion { x, y, xrel, yrel });
                    },
                    WM_LBUTTONDOWN =>
                    {
                        let x: i32 = GET_X_LPARAM(msg.lParam);
                        let y: i32 = GET_Y_LPARAM(msg.lParam);
                        let button: u8 = 0;
                        let state: u8 = 1;

                        event = Some(Event::MouseButton { x, y, button, state });
                    },
                    WM_MBUTTONDOWN =>
                    {
                        let x: i32 = GET_X_LPARAM(msg.lParam);
                        let y: i32 = GET_Y_LPARAM(msg.lParam);
                        let button: u8 = 1;
                        let state: u8 = 1;

                        event = Some(Event::MouseButton { x, y, button, state });
                    },
                    WM_RBUTTONDOWN =>
                    {
                        let x: i32 = GET_X_LPARAM(msg.lParam);
                        let y: i32 = GET_Y_LPARAM(msg.lParam);
                        let button: u8 = 2;
                        let state: u8 = 1;

                        event = Some(Event::MouseButton { x, y, button, state });
                    },
                    WM_LBUTTONUP =>
                    {
                        let x: i32 = GET_X_LPARAM(msg.lParam);
                        let y: i32 = GET_Y_LPARAM(msg.lParam);
                        let button: u8 = 0;
                        let state: u8 = 0;

                        event = Some(Event::MouseButton { x, y, button, state });
                    },
                    WM_MBUTTONUP =>
                    {
                        let x: i32 = GET_X_LPARAM(msg.lParam);
                        let y: i32 = GET_Y_LPARAM(msg.lParam);
                        let button: u8 = 1;
                        let state: u8 = 0;

                        event = Some(Event::MouseButton { x, y, button, state });
                    },
                    WM_RBUTTONUP =>
                    {
                        let x: i32 = GET_X_LPARAM(msg.lParam);
                        let y: i32 = GET_Y_LPARAM(msg.lParam);
                        let button: u8 = 2;
                        let state: u8 = 0;

                        event = Some(Event::MouseButton { x, y, button, state });
                    },
                    WM_MOUSELEAVE =>
                    {
                        self.mouse_tracked = false;
                    },
                    WM_MOUSEWHEEL =>
                    {
                        let x: i32 = GET_X_LPARAM(msg.lParam);
                        let y: i32 = GET_Y_LPARAM(msg.lParam);
                        let xscroll: f32 = 0.0;
                        let yscroll: f32 = (HIWORD(msg.wParam as _) as f32) / (WHEEL_DELTA as f32);

                        event = Some(Event::MouseScroll { x, y, xscroll, yscroll });
                    }
                    _ => {}
                };

                TranslateMessage(&mut msg);
                DispatchMessageW(&mut msg);
            }
        }

        event
    }

    fn set_window_mode(&mut self, mode: WindowMode)
    {
        todo!()
    }

    fn set_resizable(&mut self, enabled: bool)
    {
        todo!()
    }

    pub fn is_key_down(&self, key: Key) -> bool
    {
        self.keys_current[key as usize]
    }

    pub fn is_key_up(&self, key: Key) -> bool
    {
        !self.keys_current[key as usize]
    }

    pub fn is_key_pressed(&self, key: Key) -> bool
    {
        self.keys_current[key as usize] && !self.keys_previous[key as usize]
    }

    pub fn gl_create_context(&self, create_info: &GLContextCreateInfo) -> Result<GLContext, GLError>
    {
        // Create fake opengl context
        println!("Creating fake window context");
        let mut fake_class_name = win32_to_wstring("OpenGL-Context");
        let fake_wnd_class = unsafe
        {
            WNDCLASSW
            {
                style: CS_OWNDC,
                lpfnWndProc: Some(DefWindowProcW),
                hInstance: self.hinstance,
                lpszClassName: fake_class_name.as_ptr(),
                ..std::mem::zeroed()
            }
        };
        let fake_class: u16 = unsafe { RegisterClassW(&fake_wnd_class) };
        if fake_class == 0
        {
            panic!("OpenGL context creation failed!");
        }
        let fake_handle = unsafe { CreateWindowExW(0, fake_class as *const WCHAR, [0].as_ptr(), 0, CW_USEDEFAULT, CW_USEDEFAULT, CW_USEDEFAULT, CW_USEDEFAULT, std::ptr::null_mut(), std::ptr::null_mut(), self.hinstance, std::ptr::null_mut()) };
        if fake_handle.is_null()
        {
            panic!("OpenGL context creation failed!");
        }
        let fake_hdc = unsafe { GetDC(fake_handle) };
        let fake_pfd = unsafe
        {
            PIXELFORMATDESCRIPTOR
            {
                nSize: std::mem::size_of::<PIXELFORMATDESCRIPTOR>() as u16,
                nVersion: 1,
                dwFlags: PFD_DRAW_TO_WINDOW | PFD_SUPPORT_OPENGL | PFD_DOUBLEBUFFER,
                iPixelType: PFD_TYPE_RGBA,
                cColorBits: 32,
                cAlphaBits: 8,
                cDepthBits: 24,
                cStencilBits: 8,
                iLayerType: PFD_MAIN_PLANE,
                ..std::mem::zeroed()
            }
        };
        unsafe { SetPixelFormat(fake_hdc, ChoosePixelFormat(fake_hdc, &fake_pfd), &fake_pfd) };
        let fake_hglrc = unsafe { wglCreateContext(fake_hdc) };
        if fake_hglrc.is_null()
        {
            panic!("OpenGL context creation failed!");
        }
        unsafe { wglMakeCurrent(fake_hdc, fake_hglrc) };
        #[allow(non_snake_case)]
        let wglCreateContextAttribsARB: Option<WglCreateContextAttribsARB> =
        {
            let symbol = CString::new("wglCreateContextAttribsARB").unwrap();
            let addr = unsafe { wglGetProcAddress(symbol.as_ptr()) };
            if !addr.is_null()
            {
                Some(unsafe { std::mem::transmute(addr) })
            }
            else
            {
                None
            }
        };
        #[allow(non_snake_case)]
        let wglChoosePixelFormatARB: Option<WglChoosePixelFormatARB> =
        {
            let symbol = CString::new("wglChoosePixelFormatARB").unwrap();
            let addr = unsafe { wglGetProcAddress(symbol.as_ptr()) };
            if !addr.is_null()
            {
                Some(unsafe { std::mem::transmute(addr) })
            }
            else
            {
                None
            }
        };
        #[allow(non_snake_case)]
        let wglSwapIntervalEXT: Option<WglSwapIntervalEXT> =
        {
            let symbol = CString::new("wglSwapIntervalEXT").unwrap();
            let addr = unsafe { wglGetProcAddress(symbol.as_ptr()) };
            if !addr.is_null()
            {
                Some(unsafe { std::mem::transmute(addr) })
            }
            else
            {
                None
            }
        };
        unsafe { wglMakeCurrent(fake_hdc, fake_hglrc) };
        unsafe { ReleaseDC(fake_handle, fake_hdc) };
        unsafe { UnregisterClassW(fake_class as *const WCHAR, self.hinstance) };
        unsafe { DestroyWindow(fake_handle) };

        // Create real opengl context
        println!("Creating real window context");
        let hdc = unsafe { GetDC(self.handle) };
        let pixel_format_attribs =
        [
            WGL_DRAW_TO_WINDOW_ARB, 1,
            WGL_ACCELERATION_ARB, WGL_FULL_ACCELERATION_ARB,
            WGL_SUPPORT_OPENGL_ARB, 1,
            WGL_DOUBLE_BUFFER_ARB, 1,
            WGL_PIXEL_TYPE_ARB, WGL_TYPE_RGBA_ARB,
            WGL_RED_BITS_ARB, 8,
            WGL_GREEN_BITS_ARB, 8,
            WGL_BLUE_BITS_ARB, 8,
            WGL_ALPHA_BITS_ARB, 8,
            WGL_DEPTH_BITS_ARB, 24,
            WGL_STENCIL_BITS_ARB, 8,
            WGL_SAMPLE_BUFFERS_ARB, 0,
            WGL_SAMPLES_ARB, 0,
            WGL_FRAMEBUFFER_SRGB_CAPABLE_ARB, 1,
            0
        ];
        let mut pixel_format: i32 = 0;
        let mut num_formats: u32 = 0;
        wglChoosePixelFormatARB.unwrap()(hdc, pixel_format_attribs.as_ptr(), std::ptr::null(), 1, &mut pixel_format, &mut num_formats);
        let mut pfd: PIXELFORMATDESCRIPTOR = unsafe { std::mem::zeroed() };
        unsafe { DescribePixelFormat(hdc, pixel_format, std::mem::size_of::<PIXELFORMATDESCRIPTOR>() as u32, &mut pfd) };
        unsafe { SetPixelFormat(hdc, pixel_format, &pfd) };
        let profile_mask: i32 = WGL_CONTEXT_CORE_PROFILE_BIT_ARB;
        let ctx_attribs = 
        [
            WGL_CONTEXT_MAJOR_VERSION_ARB, 3,
            WGL_CONTEXT_MINOR_VERSION_ARB, 2,
            WGL_CONTEXT_PROFILE_MASK_ARB, profile_mask,
            0
        ];
        let hglrc = wglCreateContextAttribsARB.unwrap()(hdc, std::ptr::null_mut(), ctx_attribs.as_ptr());
        if hglrc == std::ptr::null_mut()
        {
            println!("OpenGL context creation failed!");
            return Err(GLError::CreationFailed);
        }
        let gl_library_name = CString::new("opengl32.dll").unwrap();
        let gl_library = unsafe { LoadLibraryA(gl_library_name.as_ptr()) };
        unsafe { wglMakeCurrent(hdc, hglrc) };
        wglSwapIntervalEXT.unwrap()(0); // no vsync for you!

        gl::load_with(|s| unsafe { gl_get_proc_address(s) as *const _ });

        gl::Viewport::load_with(|s| unsafe { gl_get_proc_address(s) as *const _ });

        if !gl::Viewport::is_loaded()
        {
            println!("Failed to load opengl viewport functions!");
            return Err(GLError::CreationFailed);
        }

        unsafe
        {
            gl::Enable(gl::DEBUG_OUTPUT);
            gl::DebugMessageCallback(Some(gl_debug_message_callback), std::ptr::null());
        }

        Ok(GLContext
        {
            hdc,
            hglrc,
            gl_library
        })
    }

    pub fn gl_swap_buffers(&self, gl_context: &GLContext)
    {
        unsafe { SwapBuffers(gl_context.hdc) };
    }
}

/* Public platform functions */

static mut COUNTER_START: u64 = 0;
static mut COUNTER_FREQUENCY: u64 = 0;

pub fn get_name() -> &'static str
{
    "Windows"
}

pub fn init()
{
    unsafe { COUNTER_START = get_performance_counter(); }
    unsafe { COUNTER_FREQUENCY = get_performance_frequency(); }
}

pub fn create_window(create_info: &WindowCreateInfo) -> Window
{
    let mut window_rect = RECT
    {
        left: 0 as LONG,
        right: create_info.width as LONG,
        top: 0 as LONG,
        bottom: create_info.height as LONG
    };

    let hinstance = unsafe { GetModuleHandleW(core::ptr::null()) };

    let class_name_w = win32_register_class("my class", CS_HREDRAW | CS_VREDRAW | CS_OWNDC);

    let handle: HWND = unsafe { CreateWindowExW(
        WS_EX_APPWINDOW | WS_EX_WINDOWEDGE,
        class_name_w.as_ptr(),
        win32_to_wstring(create_info.title).as_ptr(),
        WS_OVERLAPPEDWINDOW,
        CW_USEDEFAULT,
        CW_USEDEFAULT,
        create_info.width as i32,
        create_info.height as i32,
        0 as HWND,
        0 as HMENU,
        hinstance,
        std::ptr::null_mut()
    )};

    unsafe { AdjustWindowRect(&mut window_rect, WS_OVERLAPPEDWINDOW, FALSE); }

    unsafe { ShowWindow(handle, SW_SHOW); }
    unsafe { UpdateWindow(handle); }
    
    let win_error: u32 = win32_get_last_error();
    if win_error != ERROR_SUCCESS
    {
        panic!("Win32 error: {}", win_error);
    }

    println!("Created window");

    Window
    {
        width: create_info.width,
        height: create_info.height,
        title: create_info.title.to_owned(),
        mode: create_info.mode,
        key_modifiers: KeyModifiers::default(),
        last_mouse_x: 0,
        last_mouse_y: 0,
        mouse_tracked: false,
        keys_current: [false; 256],
        keys_previous: [false; 256],
        resizable: create_info.resizable,
        should_close: false,
        events: Vec::new(),

        hinstance,
        handle
    }
}

pub fn get_performance_counter() -> u64
{
    let mut now: LARGE_INTEGER = unsafe { std::mem::zeroed() };
    unsafe { QueryPerformanceCounter(&mut now) };
    return unsafe { *now.QuadPart() } as u64;
}

pub fn get_performance_frequency() -> u64
{
    let mut now: LARGE_INTEGER = unsafe { std::mem::zeroed() };
    unsafe { QueryPerformanceFrequency(&mut now) };
    return unsafe { *now.QuadPart() } as u64;
}

pub fn get_ticks() -> u64
{
    let mut now = get_performance_counter();

    now -= unsafe { COUNTER_START };
    now *= 1000;
    now /= unsafe { COUNTER_FREQUENCY };

    now
}

pub fn delay(ms: u32)
{
    unsafe { Sleep(ms) }
}

pub fn get_cpu_count() -> i32
{
    todo!()
}

pub fn get_system_memory() -> SystemMemory
{
    let info = unsafe
    {
        let mut info = _core::mem::MaybeUninit::<MEMORYSTATUSEX>::uninit();
        std::ptr::addr_of_mut!((*info.as_mut_ptr()).dwLength).write(_core::mem::size_of::<MEMORYSTATUSEX>() as _);

        if GlobalMemoryStatusEx(info.as_mut_ptr()) == 0
        {
            return SystemMemory
            {
                total: 0,
                avail: 0
            }
        }
        info.assume_init()
    };

    SystemMemory
    {
        total: info.ullTotalPhys,
        avail: info.ullAvailPhys
    }
}

pub fn get_power_info() -> PowerInfo
{
    todo!()
}

pub unsafe fn gl_get_proc_address(symbol: &str) -> *const c_void
{
    let symbol = CString::new(symbol).unwrap();
    let addr = wglGetProcAddress(symbol.as_ptr()) as *const c_void;
    if !addr.is_null()
    {
        return addr;
    }

    let module_file_name = CString::new("opengl32.dll").unwrap();
    let module = LoadLibraryA(module_file_name.as_ptr());
    GetProcAddress(module, symbol.as_ptr()) as *const c_void
}
