// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use glib::object::Cast;
use glib::object::IsA;
use glib::signal::connect_raw;
use glib::signal::SignalHandlerId;
use glib::translate::*;
use glib_sys;
use gtk_sys;
use libc;
use std::boxed::Box as Box_;
use std::fmt;
use std::mem;
use std::mem::transmute;
use EventController;
use Gesture;
use GestureSingle;

glib_wrapper! {
    pub struct GestureDrag(Object<gtk_sys::GtkGestureDrag, gtk_sys::GtkGestureDragClass>) @extends GestureSingle, Gesture, EventController;

    match fn {
        get_type => || gtk_sys::gtk_gesture_drag_get_type(),
    }
}

impl GestureDrag {
    pub fn new() -> GestureDrag {
        assert_initialized_main_thread!();
        unsafe { Gesture::from_glib_full(gtk_sys::gtk_gesture_drag_new()).unsafe_cast() }
    }
}

impl Default for GestureDrag {
    fn default() -> Self {
        Self::new()
    }
}

pub const NONE_GESTURE_DRAG: Option<&GestureDrag> = None;

pub trait GestureDragExt: 'static {
    fn get_offset(&self) -> Option<(f64, f64)>;

    fn get_start_point(&self) -> Option<(f64, f64)>;

    fn connect_drag_begin<F: Fn(&Self, f64, f64) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_drag_end<F: Fn(&Self, f64, f64) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_drag_update<F: Fn(&Self, f64, f64) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<GestureDrag>> GestureDragExt for O {
    fn get_offset(&self) -> Option<(f64, f64)> {
        unsafe {
            let mut x = mem::MaybeUninit::uninit();
            let mut y = mem::MaybeUninit::uninit();
            let ret = from_glib(gtk_sys::gtk_gesture_drag_get_offset(
                self.as_ref().to_glib_none().0,
                x.as_mut_ptr(),
                y.as_mut_ptr(),
            ));
            let x = x.assume_init();
            let y = y.assume_init();
            if ret {
                Some((x, y))
            } else {
                None
            }
        }
    }

    fn get_start_point(&self) -> Option<(f64, f64)> {
        unsafe {
            let mut x = mem::MaybeUninit::uninit();
            let mut y = mem::MaybeUninit::uninit();
            let ret = from_glib(gtk_sys::gtk_gesture_drag_get_start_point(
                self.as_ref().to_glib_none().0,
                x.as_mut_ptr(),
                y.as_mut_ptr(),
            ));
            let x = x.assume_init();
            let y = y.assume_init();
            if ret {
                Some((x, y))
            } else {
                None
            }
        }
    }

    fn connect_drag_begin<F: Fn(&Self, f64, f64) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn drag_begin_trampoline<P, F: Fn(&P, f64, f64) + 'static>(
            this: *mut gtk_sys::GtkGestureDrag,
            start_x: libc::c_double,
            start_y: libc::c_double,
            f: glib_sys::gpointer,
        ) where
            P: IsA<GestureDrag>,
        {
            let f: &F = &*(f as *const F);
            f(
                &GestureDrag::from_glib_borrow(this).unsafe_cast_ref(),
                start_x,
                start_y,
            )
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"drag-begin\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    drag_begin_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_drag_end<F: Fn(&Self, f64, f64) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn drag_end_trampoline<P, F: Fn(&P, f64, f64) + 'static>(
            this: *mut gtk_sys::GtkGestureDrag,
            offset_x: libc::c_double,
            offset_y: libc::c_double,
            f: glib_sys::gpointer,
        ) where
            P: IsA<GestureDrag>,
        {
            let f: &F = &*(f as *const F);
            f(
                &GestureDrag::from_glib_borrow(this).unsafe_cast_ref(),
                offset_x,
                offset_y,
            )
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"drag-end\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    drag_end_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_drag_update<F: Fn(&Self, f64, f64) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn drag_update_trampoline<P, F: Fn(&P, f64, f64) + 'static>(
            this: *mut gtk_sys::GtkGestureDrag,
            offset_x: libc::c_double,
            offset_y: libc::c_double,
            f: glib_sys::gpointer,
        ) where
            P: IsA<GestureDrag>,
        {
            let f: &F = &*(f as *const F);
            f(
                &GestureDrag::from_glib_borrow(this).unsafe_cast_ref(),
                offset_x,
                offset_y,
            )
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"drag-update\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    drag_update_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl fmt::Display for GestureDrag {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "GestureDrag")
    }
}
