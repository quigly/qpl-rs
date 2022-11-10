/* TODO:

- Controller mapping enum
- Controller type enum

*/

#[cfg(target_os = "windows")]
mod windows;
#[cfg(target_os = "windows")]
pub use windows::*;

#[cfg(target_os = "linux")]
mod linux;
#[cfg(target_os = "linux")]
pub use linux::*;

pub const WINDOW_POS_CENTER: i32 = 0xB00B5;

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
	Gui = 70,
	F1 = 71,
	F2 = 72,
	F3 = 73,
	F4 = 74,
	F5 = 75,
	F6 = 76,
	F7 = 77,
	F8 = 78,
	F9 = 79,
	F10 = 80,
	F11 = 81,
	F12 = 82,

	PrintScreen = 83,
	Menu = 84
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

#[derive(Debug, Copy, Clone, PartialEq, PartialOrd)]
pub struct KeyModifiers
{
	pub shift: bool,
	pub ctrl: bool,
	pub alt: bool,
	pub gui: bool,
	pub caps: bool
}

impl Default for KeyModifiers
{
	fn default() -> Self
	{
		Self
		{
			shift: false,
			ctrl: false,
			alt: false,
			gui: false,
			caps: false
		}
	}
}

#[derive(Debug, Copy, Clone)]
pub enum Event
{
	Quit,
	Key
	{
		key: Key,
		state: u8,
		modifiers: KeyModifiers
	},
	MouseMotion
	{
		x: i32,
		y: i32,
		xrel: i32,
		yrel: i32
	},
	MouseButton
	{
		x: i32,
		y: i32,
		button: u8,
		state: u8
	},
	MouseScroll
	{
		x: i32,
		y: i32,
		xscroll: f32,
		yscroll: f32
	}
}

#[derive(Debug, Copy, Clone)]
pub struct WindowCreateInfo<'a>
{
	pub x: i32,
	pub y: i32,
	pub width: u32,
	pub height: u32,
	pub title: &'a str,
	pub mode: WindowMode,
	pub resizable: bool
}

impl Default for WindowCreateInfo<'_>
{
	fn default() -> Self
	{
		Self
		{
			x: WINDOW_POS_CENTER,
			y: WINDOW_POS_CENTER,
			width: 1280,
			height: 720,
			title: "Game",
			mode: WindowMode::Windowed,
			resizable: false
		}
	}
}

#[derive(Debug, Copy, Clone)]
pub struct GLContextCreateInfo
{
	pub version: (u8, u8),
	pub red_bits: u8,
	pub green_bits: u8,
	pub blue_bits: u8,
	pub alpha_bits: u8,
	pub depth_bits: u8,
	pub stencil_bits: u8,
	pub samples: Option<u8>,
	pub double_buffer: bool,
	pub vsync: bool
}

impl Default for GLContextCreateInfo
{
	fn default() -> Self
	{
		Self
		{
			version: (3, 2),
			red_bits: 8,
			green_bits: 8,
			blue_bits: 8,
			alpha_bits: 8,
			depth_bits: 24,
			stencil_bits: 8,
			samples: None,
			double_buffer: true,
			vsync: false
		}	
	}
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum GLError
{
	InvalidWindowHandle,
	VersionNotSupported,
	CreationFailed,
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum GamepadButton
{
	A,
	B,
	X,
	Y,
	Back,
	Guide,
	Start,
	LeftStick,
	RightStick,
	LeftShoulder,
	RightShoulder,
	DpadUp,
	DpadDown,
	DpadLeft,
	DpadRight
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum ButtonState
{
	Up,
	Down,
	Pressed
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum GamepadType
{
	Xbox360,
	XboxOne,
	Playstation3,
	Playstation4,
	Playstation5,
	NintendoSwitch
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum GamepadAxis
{
	LeftX,
	LeftY,
	RightX,
	RightY,
	LeftTrigger,
	RightTrigger
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum GamepadError
{
	NotConnected,
}
