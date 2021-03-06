// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use gio;
use glib::translate::*;
use gtk_sys;
use std::fmt;
use Buildable;
use Filter;
use MultiFilter;

glib_wrapper! {
    pub struct AnyFilter(Object<gtk_sys::GtkAnyFilter, gtk_sys::GtkAnyFilterClass>) @extends MultiFilter, Filter, @implements gio::ListModel, Buildable;

    match fn {
        get_type => || gtk_sys::gtk_any_filter_get_type(),
    }
}

impl AnyFilter {
    pub fn new() -> AnyFilter {
        assert_initialized_main_thread!();
        unsafe { from_glib_full(gtk_sys::gtk_any_filter_new()) }
    }
}

impl Default for AnyFilter {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for AnyFilter {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "AnyFilter")
    }
}
