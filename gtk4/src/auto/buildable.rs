// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use glib::object::IsA;
use glib::translate::*;
use glib::GString;
use gtk_sys;
use std::fmt;

glib_wrapper! {
    pub struct Buildable(Interface<gtk_sys::GtkBuildable>);

    match fn {
        get_type => || gtk_sys::gtk_buildable_get_type(),
    }
}

pub const NONE_BUILDABLE: Option<&Buildable> = None;

pub trait BuildableExt: 'static {
    fn get_buildable_id(&self) -> Option<GString>;
}

impl<O: IsA<Buildable>> BuildableExt for O {
    fn get_buildable_id(&self) -> Option<GString> {
        unsafe {
            from_glib_none(gtk_sys::gtk_buildable_get_buildable_id(
                self.as_ref().to_glib_none().0,
            ))
        }
    }
}

impl fmt::Display for Buildable {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Buildable")
    }
}
