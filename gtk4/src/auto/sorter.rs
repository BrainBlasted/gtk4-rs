// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use glib;
use glib::object::Cast;
use glib::object::IsA;
use glib::signal::connect_raw;
use glib::signal::SignalHandlerId;
use glib::translate::*;
use glib_sys;
use gtk_sys;
use std::boxed::Box as Box_;
use std::fmt;
use std::mem::transmute;
use Ordering;
use SorterChange;
use SorterOrder;

glib_wrapper! {
    pub struct Sorter(Object<gtk_sys::GtkSorter, gtk_sys::GtkSorterClass>);

    match fn {
        get_type => || gtk_sys::gtk_sorter_get_type(),
    }
}

pub const NONE_SORTER: Option<&Sorter> = None;

pub trait SorterExt: 'static {
    fn changed(&self, change: SorterChange);

    fn compare<P: IsA<glib::Object>, Q: IsA<glib::Object>>(&self, item1: &P, item2: &Q)
        -> Ordering;

    fn get_order(&self) -> SorterOrder;

    fn connect_changed<F: Fn(&Self, SorterChange) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<Sorter>> SorterExt for O {
    fn changed(&self, change: SorterChange) {
        unsafe {
            gtk_sys::gtk_sorter_changed(self.as_ref().to_glib_none().0, change.to_glib());
        }
    }

    fn compare<P: IsA<glib::Object>, Q: IsA<glib::Object>>(
        &self,
        item1: &P,
        item2: &Q,
    ) -> Ordering {
        unsafe {
            from_glib(gtk_sys::gtk_sorter_compare(
                self.as_ref().to_glib_none().0,
                item1.as_ref().to_glib_none().0,
                item2.as_ref().to_glib_none().0,
            ))
        }
    }

    fn get_order(&self) -> SorterOrder {
        unsafe {
            from_glib(gtk_sys::gtk_sorter_get_order(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn connect_changed<F: Fn(&Self, SorterChange) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn changed_trampoline<P, F: Fn(&P, SorterChange) + 'static>(
            this: *mut gtk_sys::GtkSorter,
            change: gtk_sys::GtkSorterChange,
            f: glib_sys::gpointer,
        ) where
            P: IsA<Sorter>,
        {
            let f: &F = &*(f as *const F);
            f(
                &Sorter::from_glib_borrow(this).unsafe_cast_ref(),
                from_glib(change),
            )
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"changed\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    changed_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl fmt::Display for Sorter {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Sorter")
    }
}
