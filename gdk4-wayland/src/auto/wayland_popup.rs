// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use gdk;
use gdk_wayland_sys;
use glib::translate::*;
use std::fmt;
use WaylandSurface;

glib_wrapper! {
    pub struct WaylandPopup(Object<gdk_wayland_sys::GdkWaylandPopup>) @extends WaylandSurface, gdk::Surface, @implements gdk::Popup;

    match fn {
        get_type => || gdk_wayland_sys::gdk_wayland_popup_get_type(),
    }
}

impl WaylandPopup {}

impl fmt::Display for WaylandPopup {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "WaylandPopup")
    }
}
