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
		DestroyWindow(h_wnd);
		return 0;
	}
    else if msg == WM_QUIT
	{
		return 0;
	}
    else if msg == WM_INPUT
    {

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
    handle: HWND,
    hdc: HDC,
    hglrc: HGLRC,
    gl_library: HMODULE
}

impl Window
{
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

                        self.keys_previous[key as usize] = self.keys_current[key as usize];
                        self.keys_current[key as usize] = if state == 1 { true } else { false };
                        // TODO(quigly): fix this shit above ^      (is_key_pressed is always firing as pressed when key is down)
                        
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

    pub fn swap_buffers(&self)
    {
        let gl_error: u32 = unsafe { gl::GetError() };
        if gl_error != gl::NO_ERROR
        {
            panic!("OpenGL error {}", gl_error);
        }
        
        unsafe { SwapBuffers(self.hdc) };
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

    // Create fake opengl context
    println!("Creating fake window context");
    let mut fake_class_name = win32_to_wstring("OpenGL-Context");
    let fake_wnd_class = unsafe
    {
        WNDCLASSW
        {
            style: CS_OWNDC,
            lpfnWndProc: Some(DefWindowProcW),
            hInstance: hinstance,
            lpszClassName: fake_class_name.as_ptr(),
            ..std::mem::zeroed()
        }
    };
    let fake_class: u16 = unsafe { RegisterClassW(&fake_wnd_class) };
    if fake_class == 0
    {
        panic!("OpenGL context creation failed!");
    }
    let fake_handle = unsafe { CreateWindowExW(0, fake_class as *const WCHAR, [0].as_ptr(), 0, CW_USEDEFAULT, CW_USEDEFAULT, CW_USEDEFAULT, CW_USEDEFAULT, std::ptr::null_mut(), std::ptr::null_mut(), hinstance, std::ptr::null_mut()) };
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
    unsafe { UnregisterClassW(fake_class as *const WCHAR, hinstance) };
    unsafe { DestroyWindow(fake_handle) };

    // Create real opengl context
    println!("Creating real window context");
    let hdc = unsafe { GetDC(handle) };
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
        panic!("OpenGL context creation failed!");
    }
    let gl_library_name = CString::new("opengl32.dll").unwrap();
    let gl_library = unsafe { LoadLibraryA(gl_library_name.as_ptr()) };
    unsafe { wglMakeCurrent(hdc, hglrc) };
    wglSwapIntervalEXT.unwrap()(0); // no vsync for you!

    gl::load_with(|s| unsafe { gl_get_proc_address(s) as *const _ });

    gl::Viewport::load_with(|s| unsafe { gl_get_proc_address(s) as *const _ });

    if !gl::Viewport::is_loaded()
    {
        panic!("Failed to load opengl viewport functions!");
    }

    unsafe
    {
        gl::Enable(gl::DEBUG_OUTPUT);
        gl::DebugMessageCallback(Some(gl_debug_message_callback), std::ptr::null());

        println!("GL_VENDOR: {}", std::ffi::CStr::from_ptr(gl::GetString(gl::VENDOR) as *const i8).to_str().unwrap());
        println!("GL_RENDERER: {}", std::ffi::CStr::from_ptr(gl::GetString(gl::RENDERER) as *const i8).to_str().unwrap());
        println!("GL_VERSION: {}", std::ffi::CStr::from_ptr(gl::GetString(gl::VERSION) as *const i8).to_str().unwrap());
        println!("GL_SHADING_LANGUAGE_VERSION: {}", std::ffi::CStr::from_ptr(gl::GetString(gl::SHADING_LANGUAGE_VERSION) as *const i8).to_str().unwrap());
    }

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
        handle,
        hdc,
        hglrc,
        gl_library
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
