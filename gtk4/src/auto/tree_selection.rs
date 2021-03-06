// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use glib::object::Cast;
use glib::object::ObjectType as ObjectType_;
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
use std::ptr;
use SelectionMode;
use TreeIter;
use TreeModel;
use TreePath;
use TreeView;

glib_wrapper! {
    pub struct TreeSelection(Object<gtk_sys::GtkTreeSelection>);

    match fn {
        get_type => || gtk_sys::gtk_tree_selection_get_type(),
    }
}

impl TreeSelection {
    pub fn count_selected_rows(&self) -> i32 {
        unsafe { gtk_sys::gtk_tree_selection_count_selected_rows(self.to_glib_none().0) }
    }

    pub fn get_mode(&self) -> SelectionMode {
        unsafe { from_glib(gtk_sys::gtk_tree_selection_get_mode(self.to_glib_none().0)) }
    }

    //pub fn get_select_function(&self) -> Option<Box_<dyn Fn(&TreeSelection, &TreeModel, &TreePath, bool) -> bool + 'static>> {
    //    unsafe { TODO: call gtk_sys:gtk_tree_selection_get_select_function() }
    //}

    pub fn get_selected(&self) -> Option<(TreeModel, TreeIter)> {
        unsafe {
            let mut model = ptr::null_mut();
            let mut iter = TreeIter::uninitialized();
            let ret = from_glib(gtk_sys::gtk_tree_selection_get_selected(
                self.to_glib_none().0,
                &mut model,
                iter.to_glib_none_mut().0,
            ));
            if ret {
                Some((from_glib_none(model), iter))
            } else {
                None
            }
        }
    }

    pub fn get_selected_rows(&self) -> (Vec<TreePath>, TreeModel) {
        unsafe {
            let mut model = ptr::null_mut();
            let ret = FromGlibPtrContainer::from_glib_full(
                gtk_sys::gtk_tree_selection_get_selected_rows(self.to_glib_none().0, &mut model),
            );
            (ret, from_glib_none(model))
        }
    }

    pub fn get_tree_view(&self) -> Option<TreeView> {
        unsafe {
            from_glib_none(gtk_sys::gtk_tree_selection_get_tree_view(
                self.to_glib_none().0,
            ))
        }
    }

    //pub fn get_user_data(&self) -> /*Unimplemented*/Option<Fundamental: Pointer> {
    //    unsafe { TODO: call gtk_sys:gtk_tree_selection_get_user_data() }
    //}

    pub fn iter_is_selected(&self, iter: &TreeIter) -> bool {
        unsafe {
            from_glib(gtk_sys::gtk_tree_selection_iter_is_selected(
                self.to_glib_none().0,
                mut_override(iter.to_glib_none().0),
            ))
        }
    }

    pub fn path_is_selected(&self, path: &TreePath) -> bool {
        unsafe {
            from_glib(gtk_sys::gtk_tree_selection_path_is_selected(
                self.to_glib_none().0,
                mut_override(path.to_glib_none().0),
            ))
        }
    }

    pub fn select_all(&self) {
        unsafe {
            gtk_sys::gtk_tree_selection_select_all(self.to_glib_none().0);
        }
    }

    pub fn select_iter(&self, iter: &TreeIter) {
        unsafe {
            gtk_sys::gtk_tree_selection_select_iter(
                self.to_glib_none().0,
                mut_override(iter.to_glib_none().0),
            );
        }
    }

    pub fn select_path(&self, path: &TreePath) {
        unsafe {
            gtk_sys::gtk_tree_selection_select_path(
                self.to_glib_none().0,
                mut_override(path.to_glib_none().0),
            );
        }
    }

    pub fn select_range(&self, start_path: &TreePath, end_path: &TreePath) {
        unsafe {
            gtk_sys::gtk_tree_selection_select_range(
                self.to_glib_none().0,
                mut_override(start_path.to_glib_none().0),
                mut_override(end_path.to_glib_none().0),
            );
        }
    }

    pub fn selected_foreach<P: FnMut(&TreeModel, &TreePath, &TreeIter)>(&self, func: P) {
        let func_data: P = func;
        unsafe extern "C" fn func_func<P: FnMut(&TreeModel, &TreePath, &TreeIter)>(
            model: *mut gtk_sys::GtkTreeModel,
            path: *mut gtk_sys::GtkTreePath,
            iter: *mut gtk_sys::GtkTreeIter,
            data: glib_sys::gpointer,
        ) {
            let model = from_glib_borrow(model);
            let path = from_glib_borrow(path);
            let iter = from_glib_borrow(iter);
            let callback: *mut P = data as *const _ as usize as *mut P;
            (*callback)(&model, &path, &iter);
        }
        let func = Some(func_func::<P> as _);
        let super_callback0: &P = &func_data;
        unsafe {
            gtk_sys::gtk_tree_selection_selected_foreach(
                self.to_glib_none().0,
                func,
                super_callback0 as *const _ as usize as *mut _,
            );
        }
    }

    pub fn set_mode(&self, type_: SelectionMode) {
        unsafe {
            gtk_sys::gtk_tree_selection_set_mode(self.to_glib_none().0, type_.to_glib());
        }
    }

    pub fn set_select_function(
        &self,
        func: Option<Box_<dyn Fn(&TreeSelection, &TreeModel, &TreePath, bool) -> bool + 'static>>,
    ) {
        let func_data: Box_<
            Option<Box_<dyn Fn(&TreeSelection, &TreeModel, &TreePath, bool) -> bool + 'static>>,
        > = Box_::new(func);
        unsafe extern "C" fn func_func(
            selection: *mut gtk_sys::GtkTreeSelection,
            model: *mut gtk_sys::GtkTreeModel,
            path: *mut gtk_sys::GtkTreePath,
            path_currently_selected: glib_sys::gboolean,
            data: glib_sys::gpointer,
        ) -> glib_sys::gboolean {
            let selection = from_glib_borrow(selection);
            let model = from_glib_borrow(model);
            let path = from_glib_borrow(path);
            let path_currently_selected = from_glib(path_currently_selected);
            let callback: &Option<
                Box_<dyn Fn(&TreeSelection, &TreeModel, &TreePath, bool) -> bool + 'static>,
            > = &*(data as *mut _);
            let res = if let Some(ref callback) = *callback {
                callback(&selection, &model, &path, path_currently_selected)
            } else {
                panic!("cannot get closure...")
            };
            res.to_glib()
        }
        let func = if func_data.is_some() {
            Some(func_func as _)
        } else {
            None
        };
        unsafe extern "C" fn destroy_func(data: glib_sys::gpointer) {
            let _callback: Box_<
                Option<Box_<dyn Fn(&TreeSelection, &TreeModel, &TreePath, bool) -> bool + 'static>>,
            > = Box_::from_raw(data as *mut _);
        }
        let destroy_call3 = Some(destroy_func as _);
        let super_callback0: Box_<
            Option<Box_<dyn Fn(&TreeSelection, &TreeModel, &TreePath, bool) -> bool + 'static>>,
        > = func_data;
        unsafe {
            gtk_sys::gtk_tree_selection_set_select_function(
                self.to_glib_none().0,
                func,
                Box_::into_raw(super_callback0) as *mut _,
                destroy_call3,
            );
        }
    }

    pub fn unselect_all(&self) {
        unsafe {
            gtk_sys::gtk_tree_selection_unselect_all(self.to_glib_none().0);
        }
    }

    pub fn unselect_iter(&self, iter: &TreeIter) {
        unsafe {
            gtk_sys::gtk_tree_selection_unselect_iter(
                self.to_glib_none().0,
                mut_override(iter.to_glib_none().0),
            );
        }
    }

    pub fn unselect_path(&self, path: &TreePath) {
        unsafe {
            gtk_sys::gtk_tree_selection_unselect_path(
                self.to_glib_none().0,
                mut_override(path.to_glib_none().0),
            );
        }
    }

    pub fn unselect_range(&self, start_path: &TreePath, end_path: &TreePath) {
        unsafe {
            gtk_sys::gtk_tree_selection_unselect_range(
                self.to_glib_none().0,
                mut_override(start_path.to_glib_none().0),
                mut_override(end_path.to_glib_none().0),
            );
        }
    }

    pub fn connect_changed<F: Fn(&TreeSelection) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn changed_trampoline<F: Fn(&TreeSelection) + 'static>(
            this: *mut gtk_sys::GtkTreeSelection,
            f: glib_sys::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"changed\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    changed_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    pub fn connect_property_mode_notify<F: Fn(&TreeSelection) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_mode_trampoline<F: Fn(&TreeSelection) + 'static>(
            this: *mut gtk_sys::GtkTreeSelection,
            _param_spec: glib_sys::gpointer,
            f: glib_sys::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::mode\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_mode_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

#[derive(Clone, Default)]
pub struct TreeSelectionBuilder {
    mode: Option<SelectionMode>,
}

impl TreeSelectionBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn build(self) -> TreeSelection {
        let mut properties: Vec<(&str, &dyn ToValue)> = vec![];
        if let Some(ref mode) = self.mode {
            properties.push(("mode", mode));
        }
        let ret = glib::Object::new(TreeSelection::static_type(), &properties)
            .expect("object new")
            .downcast::<TreeSelection>()
            .expect("downcast");
        ret
    }

    pub fn mode(mut self, mode: SelectionMode) -> Self {
        self.mode = Some(mode);
        self
    }
}

impl fmt::Display for TreeSelection {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "TreeSelection")
    }
}
