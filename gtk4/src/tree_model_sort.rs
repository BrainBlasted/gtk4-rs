// Copyright 2013-2015, The Gtk-rs Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <https://opensource.org/licenses/MIT>

use glib::object::IsA;
use glib::translate::*;
use TreeModel;
use TreeModelSort;

impl TreeModelSort {
    pub fn new<T: IsA<TreeModel>>(child_model: &T) -> TreeModelSort {
        skip_assert_initialized!();
        unsafe {
            TreeModelSort::from_glib_none(gtk_sys::gtk_tree_model_sort_new_with_model(
                child_model.as_ref().to_glib_none().0,
            ))
        }
    }
}
