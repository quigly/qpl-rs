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

}

impl Window
{
    pub fn poll_events(&mut self) -> Option<Event>
    {
        todo!()
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