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

pub fn create_window(create_info: &WindowCreateInfo) -> Window
{
    todo!()
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