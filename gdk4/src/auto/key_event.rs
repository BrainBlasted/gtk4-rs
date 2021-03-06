// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use gdk_sys;
use glib::translate::*;
use std::fmt;
use std::mem;
use Event;
use KeyMatch;
use ModifierType;

glib_wrapper! {
    pub struct KeyEvent(Object<gdk_sys::GdkKeyEvent>) @extends Event;

    match fn {
        get_type => || gdk_sys::gdk_key_event_get_type(),
    }
}

impl KeyEvent {
    pub fn get_consumed_modifiers(&self) -> ModifierType {
        unsafe {
            from_glib(gdk_sys::gdk_key_event_get_consumed_modifiers(
                self.to_glib_none().0,
            ))
        }
    }

    pub fn get_keycode(&self) -> u32 {
        unsafe { gdk_sys::gdk_key_event_get_keycode(self.to_glib_none().0) }
    }

    pub fn get_keyval(&self) -> u32 {
        unsafe { gdk_sys::gdk_key_event_get_keyval(self.to_glib_none().0) }
    }

    pub fn get_layout(&self) -> u32 {
        unsafe { gdk_sys::gdk_key_event_get_layout(self.to_glib_none().0) }
    }

    pub fn get_level(&self) -> u32 {
        unsafe { gdk_sys::gdk_key_event_get_level(self.to_glib_none().0) }
    }

    pub fn get_match(&self) -> Option<(u32, ModifierType)> {
        unsafe {
            let mut keyval = mem::MaybeUninit::uninit();
            let mut modifiers = mem::MaybeUninit::uninit();
            let ret = from_glib(gdk_sys::gdk_key_event_get_match(
                self.to_glib_none().0,
                keyval.as_mut_ptr(),
                modifiers.as_mut_ptr(),
            ));
            let keyval = keyval.assume_init();
            let modifiers = modifiers.assume_init();
            if ret {
                Some((keyval, from_glib(modifiers)))
            } else {
                None
            }
        }
    }

    pub fn is_modifier(&self) -> bool {
        unsafe { from_glib(gdk_sys::gdk_key_event_is_modifier(self.to_glib_none().0)) }
    }

    pub fn matches(&self, keyval: u32, modifiers: ModifierType) -> KeyMatch {
        unsafe {
            from_glib(gdk_sys::gdk_key_event_matches(
                self.to_glib_none().0,
                keyval,
                modifiers.to_glib(),
            ))
        }
    }
}

impl fmt::Display for KeyEvent {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "KeyEvent")
    }
}
