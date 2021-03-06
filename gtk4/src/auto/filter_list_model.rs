// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use gio;
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
use Filter;

glib_wrapper! {
    pub struct FilterListModel(Object<gtk_sys::GtkFilterListModel, gtk_sys::GtkFilterListModelClass>) @implements gio::ListModel;

    match fn {
        get_type => || gtk_sys::gtk_filter_list_model_get_type(),
    }
}

impl FilterListModel {
    pub fn new<P: IsA<gio::ListModel>, Q: IsA<Filter>>(
        model: Option<&P>,
        filter: Option<&Q>,
    ) -> FilterListModel {
        assert_initialized_main_thread!();
        unsafe {
            from_glib_full(gtk_sys::gtk_filter_list_model_new(
                model.map(|p| p.as_ref()).to_glib_full(),
                filter.map(|p| p.as_ref()).to_glib_full(),
            ))
        }
    }
}

#[derive(Clone, Default)]
pub struct FilterListModelBuilder {
    filter: Option<Filter>,
    incremental: Option<bool>,
    model: Option<gio::ListModel>,
}

impl FilterListModelBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn build(self) -> FilterListModel {
        let mut properties: Vec<(&str, &dyn ToValue)> = vec![];
        if let Some(ref filter) = self.filter {
            properties.push(("filter", filter));
        }
        if let Some(ref incremental) = self.incremental {
            properties.push(("incremental", incremental));
        }
        if let Some(ref model) = self.model {
            properties.push(("model", model));
        }
        let ret = glib::Object::new(FilterListModel::static_type(), &properties)
            .expect("object new")
            .downcast::<FilterListModel>()
            .expect("downcast");
        ret
    }

    pub fn filter<P: IsA<Filter>>(mut self, filter: &P) -> Self {
        self.filter = Some(filter.clone().upcast());
        self
    }

    pub fn incremental(mut self, incremental: bool) -> Self {
        self.incremental = Some(incremental);
        self
    }

    pub fn model<P: IsA<gio::ListModel>>(mut self, model: &P) -> Self {
        self.model = Some(model.clone().upcast());
        self
    }
}

pub const NONE_FILTER_LIST_MODEL: Option<&FilterListModel> = None;

pub trait FilterListModelExt: 'static {
    fn get_filter(&self) -> Option<Filter>;

    fn get_incremental(&self) -> bool;

    fn get_model(&self) -> Option<gio::ListModel>;

    fn get_pending(&self) -> u32;

    fn set_filter<P: IsA<Filter>>(&self, filter: Option<&P>);

    fn set_incremental(&self, incremental: bool);

    fn set_model<P: IsA<gio::ListModel>>(&self, model: Option<&P>);

    fn connect_property_filter_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_incremental_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_model_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_pending_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<FilterListModel>> FilterListModelExt for O {
    fn get_filter(&self) -> Option<Filter> {
        unsafe {
            from_glib_none(gtk_sys::gtk_filter_list_model_get_filter(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn get_incremental(&self) -> bool {
        unsafe {
            from_glib(gtk_sys::gtk_filter_list_model_get_incremental(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn get_model(&self) -> Option<gio::ListModel> {
        unsafe {
            from_glib_none(gtk_sys::gtk_filter_list_model_get_model(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn get_pending(&self) -> u32 {
        unsafe { gtk_sys::gtk_filter_list_model_get_pending(self.as_ref().to_glib_none().0) }
    }

    fn set_filter<P: IsA<Filter>>(&self, filter: Option<&P>) {
        unsafe {
            gtk_sys::gtk_filter_list_model_set_filter(
                self.as_ref().to_glib_none().0,
                filter.map(|p| p.as_ref()).to_glib_none().0,
            );
        }
    }

    fn set_incremental(&self, incremental: bool) {
        unsafe {
            gtk_sys::gtk_filter_list_model_set_incremental(
                self.as_ref().to_glib_none().0,
                incremental.to_glib(),
            );
        }
    }

    fn set_model<P: IsA<gio::ListModel>>(&self, model: Option<&P>) {
        unsafe {
            gtk_sys::gtk_filter_list_model_set_model(
                self.as_ref().to_glib_none().0,
                model.map(|p| p.as_ref()).to_glib_none().0,
            );
        }
    }

    fn connect_property_filter_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_filter_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut gtk_sys::GtkFilterListModel,
            _param_spec: glib_sys::gpointer,
            f: glib_sys::gpointer,
        ) where
            P: IsA<FilterListModel>,
        {
            let f: &F = &*(f as *const F);
            f(&FilterListModel::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::filter\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_filter_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_property_incremental_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_incremental_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut gtk_sys::GtkFilterListModel,
            _param_spec: glib_sys::gpointer,
            f: glib_sys::gpointer,
        ) where
            P: IsA<FilterListModel>,
        {
            let f: &F = &*(f as *const F);
            f(&FilterListModel::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::incremental\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_incremental_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_property_model_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_model_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut gtk_sys::GtkFilterListModel,
            _param_spec: glib_sys::gpointer,
            f: glib_sys::gpointer,
        ) where
            P: IsA<FilterListModel>,
        {
            let f: &F = &*(f as *const F);
            f(&FilterListModel::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::model\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_model_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_property_pending_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_pending_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut gtk_sys::GtkFilterListModel,
            _param_spec: glib_sys::gpointer,
            f: glib_sys::gpointer,
        ) where
            P: IsA<FilterListModel>,
        {
            let f: &F = &*(f as *const F);
            f(&FilterListModel::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::pending\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_pending_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl fmt::Display for FilterListModel {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "FilterListModel")
    }
}
