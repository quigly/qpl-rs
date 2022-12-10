use super::*;
use x11::{*, xlib::{XKeycodeToKeysym, XkbKeycodeToKeysym}};
use std::
{
	mem,
	ffi::{CString}, collections::VecDeque,
};
use libc::*;

/* OpenGL */

type GlXCreateContextAttribsARB = unsafe extern "C" fn(
	dpy: *mut xlib::Display,
	fbc: glx::GLXFBConfig,
	share_context: glx::GLXContext,
	direct: xlib::Bool,
	attribs: *const libc::c_int,
) -> glx::GLXContext;

type GlXSwapIntervalEXT = unsafe extern "C" fn(dpy: *mut xlib::Display, drawable: glx::GLXDrawable, interval: i32);

const GLX_FRAMEBUFFER_SRGB_CAPABLE_ARB: i32 = 0x20B2;

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

#[derive(Debug)]
pub struct GLContext
{
	glx_context: glx::GLXContext,
	handle: u64
}

impl GLContext
{
    pub fn swap_buffers(&self)
    {
        unsafe { glx::glXSwapBuffers(DISPLAY, self.handle) }
    }

	
}

/* Gamepads */

pub struct Gamepad
{
	
}

impl Gamepad
{
	pub fn is_button_down(&self, button: GamepadButton) -> bool
	{
		todo!()
	}

	pub fn is_button_up(&self, button: GamepadButton) -> bool
	{
		todo!()
	}

	pub fn is_button_pressed(&self, button: GamepadButton) -> bool
	{
		todo!()
	}

	pub fn get_button_state(&self, button: GamepadButton) -> ButtonState
	{
		todo!()
	}
}

/* Internal functions */

fn x11_convert_keysym_to_key(keysym: u64) -> Key
{
	match keysym as u32
	{
		keysym::XK_Return => Key::Return,
		keysym::XK_Escape => Key::Escape,
		keysym::XK_BackSpace => Key::Backspace,
		keysym::XK_Tab => Key::Tab,

		keysym::XK_Caps_Lock => Key::CapsLock,

		keysym::XK_F1 => Key::F1,
		keysym::XK_F2 => Key::F2,
		keysym::XK_F3 => Key::F3,
		keysym::XK_F4 => Key::F4,
		keysym::XK_F5 => Key::F5,
		keysym::XK_F6 => Key::F6,
		keysym::XK_F7 => Key::F7,
		keysym::XK_F8 => Key::F8,
		keysym::XK_F9 => Key::F9,
		keysym::XK_F10 => Key::F10,
		keysym::XK_F11 => Key::F11,
		keysym::XK_F12 => Key::F12,

		keysym::XK_Scroll_Lock => Key::ScrollLock,
		keysym::XK_Pause => Key::Pause,
		keysym::XK_Insert => Key::Insert,
		keysym::XK_Home => Key::Home,
		keysym::XK_Page_Up => Key::PageUp,
		keysym::XK_Delete => Key::Delete,
		keysym::XK_End => Key::End,
		keysym::XK_Page_Down => Key::PageDown,
		keysym::XK_Right => Key::Right,
		keysym::XK_Left => Key::Left,
		keysym::XK_Down => Key::Down,
		keysym::XK_Up => Key::Up,

		keysym::XK_Num_Lock => Key::NumLock,
		
		keysym::XK_KP_Enter => Key::Return,
		keysym::XK_KP_0 => Key::Num0,
		keysym::XK_KP_1 => Key::Num1,
		keysym::XK_KP_2 => Key::Num2,
		keysym::XK_KP_3 => Key::Num3,
		keysym::XK_KP_4 => Key::Num4,
		keysym::XK_KP_5 => Key::Num5,
		keysym::XK_KP_6 => Key::Num6,
		keysym::XK_KP_7 => Key::Num7,
		keysym::XK_KP_8 => Key::Num8,
		keysym::XK_KP_9 => Key::Num9,

		keysym::XK_Control_L => Key::Ctrl,
		keysym::XK_Control_R => Key::Ctrl,
		keysym::XK_Shift_L => Key::Shift,
		keysym::XK_Shift_R => Key::Shift,
		keysym::XK_Alt_L => Key::Alt,
		keysym::XK_Alt_R => Key::Alt,
		keysym::XK_Meta_L => Key::Gui,
		keysym::XK_Meta_R => Key::Gui,
		
		_ => Key::Unknown
	}
}

fn x11_convert_keycode_to_key(keycode: u32) -> Key
{
	if keycode > 255
	{
		return Key::Unknown;
	}

	let keysym: u64 = unsafe { XkbKeycodeToKeysym(DISPLAY, keycode as _, 0, 1) };

	let key = x11_convert_keysym_to_key(keysym);
	if key != Key::Unknown
	{
		println!("Found key in match list");
		return key;
	}

	let keysym: u64 = unsafe { XkbKeycodeToKeysym(DISPLAY, keycode as _, 0, 0) };

	let key = x11_convert_keysym_to_key(keysym);
	if key != Key::Unknown
	{
		println!("Found key in match list");
		return key;
	}

	unsafe { XKEYS[keysym as usize] }
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
	key_modifiers: KeyModifiers,
	last_mouse_x: i32,
	last_mouse_y: i32,
	mouse_tracked: bool,
	keys_current: [bool; 256],
	keys_previous: [bool; 256],

	/* Platform-specific data */
	screen: *mut xlib::Screen,
	screen_id: i32,
	handle: u64
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

	fn process_xevent(&mut self, xevent: &xlib::XEvent)
	{
		let mut event: Option<Event> = None;
		
		match xevent.get_type()
		{
			xlib::KeyPress =>
			{
				let keycode: u32 = unsafe { xevent.key.keycode };
				let key = x11_convert_keycode_to_key(keycode);

				self.keys_current[key as usize] = true;

				let modifiers = KeyModifiers
				{
					..Default::default()
				};

				event = Some(Event::Key
				{
					key,
					state: 1,
					modifiers
				});
			},
			xlib::KeyRelease =>
			{
				let keycode: u32 = unsafe { xevent.key.keycode };
				let key = x11_convert_keycode_to_key(keycode);

				self.keys_current[key as usize] = false;

				let modifiers = KeyModifiers
				{
					..Default::default()
				};

				event = Some(Event::Key
				{
					key,
					state: 0,
					modifiers
				});
			},
			xlib::DestroyNotify =>
			{
				println!("DestroyNotify");
			},
			xlib::ClientMessage =>
			{
				unsafe
				{
					let wm = CString::new("WM_DELETE_WINDOW").expect("If you see this you're fucked").as_ptr();
					let mut deletemsg = xlib::XInternAtom(
						DISPLAY,
						wm,
						0);
					xlib::XSetWMProtocols(DISPLAY, self.screen_id as c_ulong, & mut deletemsg, 1);
					if xevent.client_message.data.get_long(0) == deletemsg as c_long
					{
						event = Some(Event::Quit);
						self.should_close = true;
					}
				}
			},
			_ => {}
		};

		match event
		{
			Some(event) => { self.event_queue.push_back(event); }
			_ => {}
		}
	}

	pub fn poll_events(&mut self) -> Option<Event>
	{
		let mut xevent = xlib::XEvent { pad: [0;24] };

		while unsafe { xlib::XPending(DISPLAY) } != 0
		{
			unsafe { xlib::XNextEvent(DISPLAY, &mut xevent); }
			self.process_xevent(&xevent);
		}

		self.event_queue.pop_back()
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
		let fb_attribs = [
			glx::GLX_X_RENDERABLE, 1,
			glx::GLX_X_VISUAL_TYPE, glx::GLX_TRUE_COLOR,
			glx::GLX_DRAWABLE_TYPE, glx::GLX_WINDOW_BIT,
			glx::GLX_RENDER_TYPE, glx::GLX_RGBA_BIT,
			glx::GLX_RED_SIZE, create_info.red_bits as i32,
			glx::GLX_GREEN_SIZE, create_info.green_bits as i32,
			glx::GLX_BLUE_SIZE, create_info.blue_bits as i32,
			glx::GLX_ALPHA_SIZE, create_info.alpha_bits as i32,
			glx::GLX_DEPTH_SIZE, create_info.depth_bits as i32,
			glx::GLX_STENCIL_SIZE, create_info.stencil_bits as i32,
			glx::GLX_DOUBLEBUFFER, create_info.double_buffer as i32,
			glx::GLX_SAMPLE_BUFFERS, create_info.samples.is_some() as i32,
			glx::GLX_SAMPLES, create_info.samples.unwrap_or(0) as i32,
			GLX_FRAMEBUFFER_SRGB_CAPABLE_ARB, 1,
			0,
		];

		let mut num_configs = 0;
		let fb_config = unsafe { glx::glXChooseFBConfig(DISPLAY, self.screen_id, 0 as _, &mut num_configs) };
		if num_configs <= 0
		{
			return Err(GLError::CreationFailed);
		}

		let glx_create_context_attribs_arb: GlXCreateContextAttribsARB = unsafe
		{
			let addr = gl_get_proc_address("glXCreateContextAttribsARB");
			if addr.is_null()
			{
				return Err(GLError::CreationFailed);
			}
			else
			{
				std::mem::transmute(addr)
			}
		};

		let glx_swap_interval_ext: GlXSwapIntervalEXT = unsafe
		{
			let addr = gl_get_proc_address("glXSwapIntervalEXT");
			if addr.is_null()
			{
				return Err(GLError::CreationFailed);
			}
			else
			{
				std::mem::transmute(addr)
			}
		};

		let profile_mask: i32 = glx::arb::GLX_CONTEXT_CORE_PROFILE_BIT_ARB;

		let ctx_attribs =
		[
			glx::arb::GLX_CONTEXT_MAJOR_VERSION_ARB, create_info.version.0 as i32,
			glx::arb::GLX_CONTEXT_MINOR_VERSION_ARB, create_info.version.1 as i32,
			glx::arb::GLX_CONTEXT_PROFILE_MASK_ARB, profile_mask as i32,
			0
		];

		let glx_context = unsafe
		{
			glx_create_context_attribs_arb(DISPLAY, *fb_config, std::ptr::null_mut(), 1, ctx_attribs.as_ptr())
		};

		if glx_context.is_null()
		{
			return Err(GLError::CreationFailed);
		}

		unsafe
		{
			glx::glXMakeCurrent(DISPLAY, self.handle, glx_context);
			glx_swap_interval_ext(DISPLAY, self.handle, create_info.vsync as i32);
		}

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

		Ok(GLContext { glx_context, handle: self.handle })
	}

	pub fn vk_create_surface(&self, entry: &ash::Entry, instance: &ash::Instance, allocation_callbacks: Option<&ash::vk::AllocationCallbacks>) -> ash::vk::SurfaceKHR
	{
		todo!()
	}
}

/* Public platform functions */

static mut COUNTER_START: u64 = 0;
static mut COUNTER_FREQUENCY: u64 = 0;
static mut DISPLAY: *mut xlib::Display = std::ptr::null_mut();
static mut XKEYS: [Key; 512] = [ Key::Unknown; 512 ];

pub fn get_name() -> &'static str
{
	"Linux"
}

pub fn init()
{
	unsafe
	{
		COUNTER_START = get_performance_counter();
		COUNTER_FREQUENCY = get_performance_frequency();

		DISPLAY = xlib::XOpenDisplay(0 as _);
		if DISPLAY.is_null()
		{
			panic!("Unable to open XDisplay.");
		}

		XKEYS[keysym::XK_a as usize] = Key::A;
		XKEYS[keysym::XK_b as usize] = Key::B;
		XKEYS[keysym::XK_c as usize] = Key::C;
		XKEYS[keysym::XK_d as usize] = Key::D;
		XKEYS[keysym::XK_e as usize] = Key::E;
		XKEYS[keysym::XK_f as usize] = Key::F;
		XKEYS[keysym::XK_g as usize] = Key::G;
		XKEYS[keysym::XK_h as usize] = Key::H;
		XKEYS[keysym::XK_i as usize] = Key::I;
		XKEYS[keysym::XK_j as usize] = Key::J;
		XKEYS[keysym::XK_k as usize] = Key::K;
		XKEYS[keysym::XK_l as usize] = Key::L;
		XKEYS[keysym::XK_m as usize] = Key::M;
		XKEYS[keysym::XK_n as usize] = Key::N;
		XKEYS[keysym::XK_o as usize] = Key::O;
		XKEYS[keysym::XK_p as usize] = Key::P;
		XKEYS[keysym::XK_q as usize] = Key::Q;
		XKEYS[keysym::XK_r as usize] = Key::R;
		XKEYS[keysym::XK_s as usize] = Key::S;
		XKEYS[keysym::XK_t as usize] = Key::T;
		XKEYS[keysym::XK_u as usize] = Key::U;
		XKEYS[keysym::XK_v as usize] = Key::V;
		XKEYS[keysym::XK_w as usize] = Key::W;
		XKEYS[keysym::XK_x as usize] = Key::X;
		XKEYS[keysym::XK_y as usize] = Key::Y;
		XKEYS[keysym::XK_z as usize] = Key::Z;

		XKEYS[keysym::XK_0 as usize] = Key::Num0;
		XKEYS[keysym::XK_1 as usize] = Key::Num1;
		XKEYS[keysym::XK_2 as usize] = Key::Num2;
		XKEYS[keysym::XK_3 as usize] = Key::Num3;
		XKEYS[keysym::XK_4 as usize] = Key::Num4;
		XKEYS[keysym::XK_5 as usize] = Key::Num5;
		XKEYS[keysym::XK_6 as usize] = Key::Num6;
		XKEYS[keysym::XK_7 as usize] = Key::Num7;
		XKEYS[keysym::XK_8 as usize] = Key::Num8;
		XKEYS[keysym::XK_9 as usize] = Key::Num9;

		XKEYS[keysym::XK_space as usize] = Key::Space;

		XKEYS[keysym::XK_minus as usize] = Key::Minus;
		XKEYS[keysym::XK_equal as usize] = Key::Equals;
		XKEYS[keysym::XK_bracketleft as usize] = Key::LeftBracket;
		XKEYS[keysym::XK_bracketright as usize] = Key::RightBracket;
		XKEYS[keysym::XK_backslash as usize] = Key::Backslash;
		XKEYS[keysym::XK_semicolon as usize] = Key::Semicolon;
		XKEYS[keysym::XK_apostrophe as usize] = Key::Apostrophe;
		XKEYS[keysym::XK_grave as usize] = Key::Grave;
		XKEYS[keysym::XK_comma as usize] = Key::Comma;
		XKEYS[keysym::XK_period as usize] = Key::Period;
		XKEYS[keysym::XK_slash as usize] = Key::Slash;
	}
}

pub fn create_window(create_info: &WindowCreateInfo) -> Window
{
	let screen: *mut xlib::Screen = unsafe { xlib::XDefaultScreenOfDisplay(DISPLAY) };
	let screen_id: i32 = unsafe { xlib::XDefaultScreen(DISPLAY) };
	
	let mut nelements: i32 = 0;
	let fbc = unsafe { glx::glXChooseFBConfig(DISPLAY, screen_id, 0 as _, &mut nelements) };
	let root = unsafe { xlib::XDefaultRootWindow(DISPLAY) };
	let depth = unsafe { xlib::XDefaultDepth(DISPLAY, screen_id) };
	let visual = unsafe { xlib::XDefaultVisual(DISPLAY, screen_id) };
	let cmap = unsafe { xlib::XCreateColormap(DISPLAY, root, visual, xlib::AllocNone) };
	let mut window_attributes: xlib::XSetWindowAttributes = unsafe { std::mem::zeroed() };
	window_attributes.event_mask =
		xlib::StructureNotifyMask | xlib::KeyPressMask | xlib::KeyReleaseMask |
		xlib::PointerMotionMask | xlib::ButtonPressMask | xlib::ButtonReleaseMask |
		xlib::ExposureMask | xlib::FocusChangeMask | xlib::VisibilityChangeMask |
		xlib::EnterWindowMask | xlib::LeaveWindowMask | xlib::PropertyChangeMask;
	window_attributes.colormap = cmap;
	let handle: u64 = unsafe { xlib::XCreateSimpleWindow(DISPLAY, root, 10, 10, 1280, 720, 0, 0, 0) };

	let window_title = std::ffi::CString::new(create_info.title).unwrap();

	unsafe { xlib::XMapWindow(DISPLAY, handle) };
	unsafe { xlib::XStoreName(DISPLAY, handle, window_title.as_ptr()) };

	unsafe { xlib::XSelectInput(DISPLAY, handle, xlib::ExposureMask |
		xlib::KeyPressMask |
		xlib::KeyReleaseMask |
		xlib::ButtonPressMask |
		xlib::ButtonReleaseMask); }

	Window
	{
		width: create_info.width,
		height: create_info.height,
		title: create_info.title.to_owned(),
		mode: create_info.mode,
		event_queue: VecDeque::new(),
		key_modifiers: KeyModifiers::default(),
		last_mouse_x: 0,
		last_mouse_y: 0,
		mouse_tracked: false,
		keys_current: [false; 256],
		keys_previous: [false; 256],
		resizable: create_info.resizable,
		should_close: false,
		events: Vec::new(),

		screen,
		screen_id,
		handle
	}
}

pub fn get_performance_counter() -> u64
{
	let mut now: timespec = timespec { tv_nsec: 0, tv_sec: 0 };
	unsafe { clock_gettime(CLOCK_MONOTONIC as _, &mut now) };
	let mut ticks: u64 = now.tv_sec as _;
	ticks *= 1000000000;
	ticks += now.tv_nsec as u64;

	ticks
}

pub fn get_performance_frequency() -> u64
{
	1000000000
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

pub unsafe fn gl_get_proc_address(symbol: &str) -> *const c_void
{
	let symbol = CString::new(symbol).unwrap();
	unsafe { glx::glXGetProcAddress(symbol.as_ptr() as *const u8).unwrap() as *const c_void }
}

pub fn get_num_gamepads() -> u32
{
	todo!()
}

pub fn open_gamepad(index: u32) -> Result<Gamepad, GamepadError>
{
	todo!()
}

pub fn vk_get_surface_extension() -> &'static str
{
	todo!()
}
