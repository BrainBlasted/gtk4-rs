// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use gio;
use glib::object::IsA;
use glib::translate::*;
use gtk_sys;
use std::fmt;
use Buildable;
use Filter;

glib_wrapper! {
    pub struct MultiFilter(Object<gtk_sys::GtkMultiFilter, gtk_sys::GtkMultiFilterClass>) @extends Filter, @implements gio::ListModel, Buildable;

    match fn {
        get_type => || gtk_sys::gtk_multi_filter_get_type(),
    }
}

pub const NONE_MULTI_FILTER: Option<&MultiFilter> = None;

pub trait MultiFilterExt: 'static {
    fn append<P: IsA<Filter>>(&self, filter: &P);

    fn remove(&self, position: u32);
}

impl<O: IsA<MultiFilter>> MultiFilterExt for O {
    fn append<P: IsA<Filter>>(&self, filter: &P) {
        unsafe {
            gtk_sys::gtk_multi_filter_append(
                self.as_ref().to_glib_none().0,
                filter.as_ref().to_glib_full(),
            );
        }
    }

    fn remove(&self, position: u32) {
        unsafe {
            gtk_sys::gtk_multi_filter_remove(self.as_ref().to_glib_none().0, position);
        }
    }
}

impl fmt::Display for MultiFilter {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "MultiFilter")
    }
}
