#[cfg(target_os = "windows")]
mod windows;
#[cfg(target_os = "windows")]
pub use windows::*;

#[cfg(target_os = "linux")]
mod linux;
#[cfg(target_os = "linux")]
pub use linux::*;

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Key
{
	Unknown = 0,

	A = 1,
	B = 2,
	C = 3,
	D = 4,
	E = 5,
	F = 6,
	G = 7,
	H = 8,
	I = 9,
	J = 10,
	K = 11,
	L = 12,
	M = 13,
	N = 14,
	O = 15,
	P = 16,
	Q = 17,
	R = 18,
	S = 19,
	T = 20,
	U = 21,
	V = 22,
	W = 23,
	X = 24,
	Y = 25,
	Z = 26,

	Num0 = 27,
	Num1 = 28,
	Num2 = 29,
	Num3 = 30,
	Num4 = 31,
	Num5 = 32,
	Num6 = 33,
	Num7 = 34,
	Num8 = 35,
	Num9 = 36,

	Return = 37,
	Escape = 38,
	Backspace = 39,
	Tab = 40,
	Space = 41,

	Minus = 42,
	Equals = 43,
	LeftBracket = 44,
	RightBracket = 45,
	Backslash = 46,
	Semicolon = 47,
	Apostrophe = 48,
	Grave = 49,
	Comma = 50,
	Period = 51,
	Slash = 52,

	CapsLock = 53,

	ScrollLock = 54,
	Pause = 55,
	Insert = 56,
	Home = 57,
	PageUp = 58,
	Delete = 59,
	End = 60,
	PageDown = 61,
	Right = 62,
	Left = 63,
	Down = 64,
	Up = 65,

	NumLock = 66,

	Ctrl = 67,
	Shift = 68,
	Alt = 69,
	F1 = 70,
	F2 = 71,
	F3 = 72,
	F4 = 73,
	F5 = 74,
	F6 = 75,
	F7 = 76,
	F8 = 77,
	F9 = 78,
	F10 = 79,
	F11 = 80,
	F12 = 81,
}

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
    Quit,
    KeyPress
    {
        key: Key
    },
    KeyRelease
    {
        key: Key
    }
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
