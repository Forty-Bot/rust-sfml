//! Access to the real-time state of the mouse.
//!
//! `mouse` provides an interface to the state of the mouse.
//!
//! This module allows users to query the mouse state at any time and directly, without having to
//! deal with a window and its events. Compared to the `MouseMoved`, `MouseButtonPressed` and
//! `MouseButtonReleased` events, `mouse` can retrieve the state of the cursor and
//! the buttons at any time (you don't need to store and update a boolean on your side in order to
//! know if a button is pressed or released), and you always get the real state of the mouse, even
//! if it is moved, pressed or released when your window is out of focus and no event is triggered.
//!
//! # Usage example
//!
//! ```ignore
//! use sfml::window::mouse;
//!
//! if mouse::Button::Left.is_pressed() {
//!     // left click
//! }
//!
//! // get global mouse position
//! let _position = mouse::desktop_position();
//!
//! // set mouse position relative to a window
//! window.set_mouse_position(Vector2i::new(100, 200));
//! ```

use csfml_window_sys as ffi;
use sf_bool_ext::SfBoolExt;
use system::Vector2i;
use system::raw_conv::{FromRaw, Raw};

/// Mouse buttons.
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Debug, Copy)]
#[repr(u32)]
pub enum Button {
    /// The left mouse button.
    Left = 0,
    /// The right mouse button.
    Right = 1,
    /// The middle (wheel) mouse button.
    Middle = 2,
    /// The first extra mouse button.
    XButton1 = 3,
    /// The second extra mouse button.
    XButton2 = 4,
    /// The total number of mouse buttons.
    Count = 5,
}

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Debug, Copy)]
/// Mouse wheels.
pub enum Wheel {
    /// The vertical mouse wheel.
    Vertical,
    /// The horizontal mouse wheel.
    Horizontal,
}

impl Raw for Wheel {
    type Raw = ffi::sfMouseWheel;

    fn raw(&self) -> Self::Raw {
        match *self {
            Wheel::Vertical => ffi::sfMouseWheel::sfMouseVerticalWheel,
            Wheel::Horizontal => ffi::sfMouseWheel::sfMouseHorizontalWheel,
        }
    }
}

impl FromRaw for Wheel {
    type RawFrom = ffi::sfMouseWheel;
    unsafe fn from_raw(raw: Self::RawFrom) -> Self {
        match raw {
            ffi::sfMouseWheel::sfMouseVerticalWheel => Wheel::Vertical,
            ffi::sfMouseWheel::sfMouseHorizontalWheel => Wheel::Horizontal,
        }
    }
}

impl Raw for Button {
    type Raw = ffi::sfMouseButton;

    fn raw(&self) -> Self::Raw {
        unsafe { ::std::mem::transmute(*self) }
    }
}

impl FromRaw for Button {
    type RawFrom = ffi::sfMouseButton;
    unsafe fn from_raw(raw: Self::RawFrom) -> Self {
        ::std::mem::transmute(raw)
    }
}

impl Button {
    /// Return whether this mouse button is currently pressed.
    ///
    /// Queries the real-time state of the mouse, even if buttons have been
    /// pressed or released while no window was focused and no events were
    /// triggered.
    pub fn is_pressed(self) -> bool {
        unsafe { ffi::sfMouse_isButtonPressed(self.raw()) }.to_bool()
    }
}

/// Get the current position of the mouse in desktop coordinates.
///
/// This function returns the global position of the mouse cursor on the desktop.
pub fn desktop_position() -> Vector2i {
    unsafe { FromRaw::from_raw(ffi::sfMouse_getPosition(::std::ptr::null())) }
}

/// Set the current position of the mouse in desktop coordinates.
///
/// This function sets the global position of the mouse cursor on the desktop.
pub fn set_desktop_position(position: &Vector2i) {
    unsafe { ffi::sfMouse_setPosition(position.raw(), ::std::ptr::null()) }
}
