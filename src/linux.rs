use std::collections::VecDeque;

use super::*;

lazy_static::lazy_static!
{
    static ref XLIB: x11_dl::xlib::Xlib = x11_dl::xlib::Xlib::open().unwrap();
    static ref XCURSOR: x11_dl::xcursor::Xcursor = x11_dl::xcursor::Xcursor::open().unwrap();
    static ref XRANDR: x11_dl::xrandr::Xrandr = x11_dl::xrandr::Xrandr::open().unwrap();
    static ref XINPUT2: x11_dl::xinput2::XInput2 = x11_dl::xinput2::XInput2::open().unwrap();
    static ref XRENDER: x11_dl::xrender::Xrender = x11_dl::xrender::Xrender::open().unwrap();
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
    event_queue: VecDeque<Event>,

    /* Platform-specific data */
    display: *mut x11_dl::xlib::Display,
    x11_fd: i32,
    root: u64,
    screen: i32,
    handle: u64
}

impl Window
{
    pub fn poll_events(&mut self) -> Option<Event>
    {
        let mut xevent = x11_dl::xlib::XEvent { pad: [0;24] };

        while unsafe { (XLIB.XPending)(self.display) } != 0
        {
            unsafe { (XLIB.XNextEvent)(self.display, &mut xevent); }
            self.process_xevent(&xevent);
        }

        self.event_queue.pop_back()
    }

    fn process_xevent(&mut self, xevent: &x11_dl::xlib::XEvent)
    {
        match xevent.get_type()
        {
            x11_dl::xlib::KeyPress =>
            {
                self.event_queue.push_front(Event::KeyPress { key: Key::A });
            },
            x11_dl::xlib::KeyRelease =>
            {
                self.event_queue.push_front(Event::KeyRelease { key: Key::A });
            },
            x11_dl::xlib::DestroyNotify =>
            {
                self.event_queue.push_front(Event::Quit);
            },
            _ => {}
        };
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

pub fn get_name() -> &'static str
{
    "Linux"
}

pub fn init()
{
    todo!()
}

pub fn create_window(create_info: &WindowCreateInfo) -> Window
{
    let display = unsafe
    {
        let display = (XLIB.XOpenDisplay)(std::ptr::null());
        if display.is_null()
        {
            panic!("Could not open display");
        }
        display
    };

    let fd = unsafe { (XLIB.XConnectionNumber)(display) };

    let root = unsafe { (XLIB.XDefaultRootWindow)(display) };
    let screen = unsafe { (XLIB.XDefaultScreen)(display) };

    let depth = unsafe { (XLIB.XDefaultDepth)(display, screen) };
    let visual = unsafe { (XLIB.XDefaultVisual)(display, screen) };
    let cmap = unsafe { (XLIB.XCreateColormap)(display, root, visual, x11_dl::xlib::AllocNone) };
    let mut window_attributes: x11_dl::xlib::XSetWindowAttributes = unsafe { std::mem::zeroed() };
    window_attributes.event_mask = x11_dl::xlib::ExposureMask |
        x11_dl::xlib::KeyPressMask;
    window_attributes.colormap = cmap;
    let handle = unsafe { (XLIB.XCreateWindow)(
        display, root,
        0, 0,
        create_info.width, create_info.height, 0, depth, 1, visual,
        x11_dl::xlib::CWColormap | x11_dl::xlib::CWEventMask,
        &mut window_attributes) };

    let window_title = std::ffi::CString::new(create_info.title).unwrap();

    unsafe { (XLIB.XMapWindow)(display, handle) };
    unsafe { (XLIB.XStoreName)(display, handle, window_title.as_ptr()) };

    unsafe { (XLIB.XSelectInput)(display, handle, x11_dl::xlib::ExposureMask |
        x11_dl::xlib::KeyPressMask |
        x11_dl::xlib::KeyReleaseMask |
        x11_dl::xlib::ButtonPressMask |
        x11_dl::xlib::ButtonReleaseMask); }
    unsafe { (XLIB.XMapWindow)(display, handle); }

    Window
    {
        width: create_info.width,
        height: create_info.height,
        title: create_info.title.to_owned(),
        should_close: false,
        events: Vec::new(),
        resizable: create_info.resizable,
        mode: create_info.mode,
        event_queue: VecDeque::new(),
        display,
        x11_fd: fd,
        root,
        screen,
        handle
    }
}

pub fn get_performance_counter() -> u64
{
    todo!()
}

pub fn get_performance_frequency() -> u64
{
    todo!()
}

pub fn get_ticks() -> u64
{
    todo!()
}

pub fn delay(ms: u32)
{
    todo!()
}

pub fn get_cpu_count() -> i32
{
    todo!()
}

pub fn get_system_memory() -> SystemMemory
{
    todo!()
}

pub fn get_power_info() -> PowerInfo
{
    todo!()
}

pub fn vk_get_surface_extension() -> &'static str
{
    todo!()
}