use super::*;

use libc::*;
use winapi::*;
use winapi::shared::basetsd::*;
use winapi::shared::minwindef::*;
use winapi::shared::windef::*;
use winapi::shared::winerror::ERROR_SUCCESS;
use winapi::um::errhandlingapi::GetLastError;
use winapi::um::libloaderapi::*;
use winapi::um::memoryapi::VirtualAlloc;
use winapi::um::memoryapi::VirtualFree;
use winapi::um::profileapi::QueryPerformanceCounter;
use winapi::um::profileapi::QueryPerformanceFrequency;
use winapi::um::synchapi::*;
use winapi::um::sysinfoapi::*;
use winapi::um::winbase::FORMAT_MESSAGE_ALLOCATE_BUFFER;
use winapi::um::winbase::FORMAT_MESSAGE_FROM_SYSTEM;
use winapi::um::winbase::FORMAT_MESSAGE_IGNORE_INSERTS;
use winapi::um::winbase::FormatMessageW;
use winapi::um::wingdi::*;
use winapi::um::winnt::*;
use winapi::um::winuser::*;
use winapi::um::xinput::*;

use std::ffi::CString;
use std::ffi::OsStr;
use std::os::windows::ffi::*;

use std::mem::size_of;

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

    /* Platform-specific data */
    hinstance: HINSTANCE,
    handle: HWND
}

impl Window
{
    pub fn poll_events(&mut self) -> Option<Event>
    {
        unsafe
        {
            let mut msg = MSG
            {
                hwnd: self.handle,
                message: 0 as UINT,
                wParam: 0 as WPARAM,
                lParam : 0 as LPARAM,
                time : 0 as DWORD,
                pt : POINT { x: 0, y: 0, }
            };

            if PeekMessageW(&mut msg, 0 as *mut HWND__, 0, 0, PM_REMOVE) == TRUE
            {
                match msg.message
                {
                    WM_QUIT =>
                    {
                        return Some(Event::Quit);
                    },
                    _ => {}
                }

                TranslateMessage(&mut msg);
                DispatchMessageW(&mut msg);
            }
        }

        None
    }

    fn set_window_mode(&mut self, mode: WindowMode)
    {
        todo!()
    }

    fn set_resizable(&mut self, enabled: bool)
    {
        todo!()
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

    let win_error = win32_get_last_error();
    if win_error != ERROR_SUCCESS
    {
        panic!("Win32 error: {}", win_error);
    }

    Window
    {
        width: create_info.width,
        height: create_info.height,
        title: create_info.title.to_owned(),
        mode: create_info.mode,
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

pub fn vk_get_surface_extension() -> &'static str
{
    "VK_KHR_win32_surface"
}