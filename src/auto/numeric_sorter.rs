// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use glib::object::Cast;
use glib::object::IsA;
use glib::signal::connect_raw;
use glib::signal::SignalHandlerId;
use glib::translate::*;
use glib::StaticType;
use glib::ToValue;
use glib_sys;
use gtk_sys;
use std::boxed::Box as Box_;
use std::fmt;
use std::mem::transmute;
use SortType;
use Sorter;

glib_wrapper! {
    pub struct NumericSorter(Object<gtk_sys::GtkNumericSorter, gtk_sys::GtkNumericSorterClass, NumericSorterClass>) @extends Sorter;

    match fn {
        get_type => || gtk_sys::gtk_numeric_sorter_get_type(),
    }
}

impl NumericSorter {
    //pub fn new(expression: /*Ignored*/Option<&Expression>) -> NumericSorter {
    //    unsafe { TODO: call gtk_sys:gtk_numeric_sorter_new() }
    //}
}

#[derive(Clone, Default)]
pub struct NumericSorterBuilder {
    //expression: /*Unknown type*/,
    sort_order: Option<SortType>,
}

impl NumericSorterBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn build(self) -> NumericSorter {
        let mut properties: Vec<(&str, &dyn ToValue)> = vec![];
        if let Some(ref sort_order) = self.sort_order {
            properties.push(("sort-order", sort_order));
        }
        let ret = glib::Object::new(NumericSorter::static_type(), &properties)
            .expect("object new")
            .downcast::<NumericSorter>()
            .expect("downcast");
        ret
    }

    pub fn sort_order(mut self, sort_order: SortType) -> Self {
        self.sort_order = Some(sort_order);
        self
    }
}

pub const NONE_NUMERIC_SORTER: Option<&NumericSorter> = None;

pub trait NumericSorterExt: 'static {
    //fn get_expression(&self) -> /*Ignored*/Option<Expression>;

    fn get_sort_order(&self) -> SortType;

    //fn set_expression(&self, expression: /*Ignored*/Option<&Expression>);

    fn set_sort_order(&self, sort_order: SortType);

    fn connect_property_expression_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_sort_order_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<NumericSorter>> NumericSorterExt for O {
    //fn get_expression(&self) -> /*Ignored*/Option<Expression> {
    //    unsafe { TODO: call gtk_sys:gtk_numeric_sorter_get_expression() }
    //}

    fn get_sort_order(&self) -> SortType {
        unsafe {
            from_glib(gtk_sys::gtk_numeric_sorter_get_sort_order(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    //fn set_expression(&self, expression: /*Ignored*/Option<&Expression>) {
    //    unsafe { TODO: call gtk_sys:gtk_numeric_sorter_set_expression() }
    //}

    fn set_sort_order(&self, sort_order: SortType) {
        unsafe {
            gtk_sys::gtk_numeric_sorter_set_sort_order(
                self.as_ref().to_glib_none().0,
                sort_order.to_glib(),
            );
        }
    }

    fn connect_property_expression_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_expression_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut gtk_sys::GtkNumericSorter,
            _param_spec: glib_sys::gpointer,
            f: glib_sys::gpointer,
        ) where
            P: IsA<NumericSorter>,
        {
            let f: &F = &*(f as *const F);
            f(&NumericSorter::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::expression\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_expression_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_property_sort_order_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_sort_order_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut gtk_sys::GtkNumericSorter,
            _param_spec: glib_sys::gpointer,
            f: glib_sys::gpointer,
        ) where
            P: IsA<NumericSorter>,
        {
            let f: &F = &*(f as *const F);
            f(&NumericSorter::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::sort-order\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_sort_order_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl fmt::Display for NumericSorter {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "NumericSorter")
    }
}
