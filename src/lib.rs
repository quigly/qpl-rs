#[cfg(target_os = "windows")]
mod windows;
#[cfg(target_os = "windows")]
pub use windows::*;

#[cfg(target_os = "linux")]
mod linux;
#[cfg(target_os = "linux")]
pub use linux::*;

#[derive(Debug, Copy, Clone)]
pub enum PowerState
{
	Unknown,
	OnBattery,
	NoBattery,
	Charging,
	Charged
}

#[derive(Debug, Copy, Clone)]
pub struct PowerInfo
{
	pub state: PowerState,
	pub seconds: i32,
	pub percent: i32
}

#[derive(Debug, Copy, Clone)]
pub struct SystemMemory
{
	pub total: u64,
	pub avail: u64
}

#[derive(Debug, Copy, Clone, PartialEq, PartialOrd)]
pub enum WindowMode
{
    Windowed,
    Borderless,
    Fullscreen
}

#[derive(Debug, Copy, Clone)]
pub enum Event
{
    Quit
}

#[derive(Debug, Copy, Clone)]
pub struct WindowCreateInfo<'a>
{
    pub width: u32,
    pub height: u32,
    pub title: &'a str,
    pub mode: WindowMode,
    pub resizable: bool
}
