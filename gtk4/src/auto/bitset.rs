// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use glib::translate::*;
use gtk_sys;

glib_wrapper! {
    #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct Bitset(Shared<gtk_sys::GtkBitset>);

    match fn {
        ref => |ptr| gtk_sys::gtk_bitset_ref(ptr),
        unref => |ptr| gtk_sys::gtk_bitset_unref(ptr),
        get_type => || gtk_sys::gtk_bitset_get_type(),
    }
}

impl Bitset {
    pub fn new_empty() -> Bitset {
        assert_initialized_main_thread!();
        unsafe { from_glib_full(gtk_sys::gtk_bitset_new_empty()) }
    }

    pub fn new_range(start: u32, n_items: u32) -> Bitset {
        assert_initialized_main_thread!();
        unsafe { from_glib_full(gtk_sys::gtk_bitset_new_range(start, n_items)) }
    }

    pub fn add(&self, value: u32) -> bool {
        unsafe { from_glib(gtk_sys::gtk_bitset_add(self.to_glib_none().0, value)) }
    }

    pub fn add_range(&self, start: u32, n_items: u32) {
        unsafe {
            gtk_sys::gtk_bitset_add_range(self.to_glib_none().0, start, n_items);
        }
    }

    pub fn add_range_closed(&self, first: u32, last: u32) {
        unsafe {
            gtk_sys::gtk_bitset_add_range_closed(self.to_glib_none().0, first, last);
        }
    }

    pub fn add_rectangle(&self, start: u32, width: u32, height: u32, stride: u32) {
        unsafe {
            gtk_sys::gtk_bitset_add_rectangle(self.to_glib_none().0, start, width, height, stride);
        }
    }

    pub fn contains(&self, value: u32) -> bool {
        unsafe { from_glib(gtk_sys::gtk_bitset_contains(self.to_glib_none().0, value)) }
    }

    pub fn copy(&self) -> Option<Bitset> {
        unsafe { from_glib_full(gtk_sys::gtk_bitset_copy(self.to_glib_none().0)) }
    }

    pub fn difference(&self, other: &Bitset) {
        unsafe {
            gtk_sys::gtk_bitset_difference(self.to_glib_none().0, other.to_glib_none().0);
        }
    }

    pub fn equals(&self, other: &Bitset) -> bool {
        unsafe {
            from_glib(gtk_sys::gtk_bitset_equals(
                self.to_glib_none().0,
                other.to_glib_none().0,
            ))
        }
    }

    pub fn get_maximum(&self) -> u32 {
        unsafe { gtk_sys::gtk_bitset_get_maximum(self.to_glib_none().0) }
    }

    pub fn get_minimum(&self) -> u32 {
        unsafe { gtk_sys::gtk_bitset_get_minimum(self.to_glib_none().0) }
    }

    pub fn get_nth(&self, nth: u32) -> u32 {
        unsafe { gtk_sys::gtk_bitset_get_nth(self.to_glib_none().0, nth) }
    }

    pub fn get_size(&self) -> u64 {
        unsafe { gtk_sys::gtk_bitset_get_size(self.to_glib_none().0) }
    }

    pub fn get_size_in_range(&self, first: u32, last: u32) -> u64 {
        unsafe { gtk_sys::gtk_bitset_get_size_in_range(self.to_glib_none().0, first, last) }
    }

    pub fn intersect(&self, other: &Bitset) {
        unsafe {
            gtk_sys::gtk_bitset_intersect(self.to_glib_none().0, other.to_glib_none().0);
        }
    }

    pub fn is_empty(&self) -> bool {
        unsafe { from_glib(gtk_sys::gtk_bitset_is_empty(self.to_glib_none().0)) }
    }

    pub fn remove(&self, value: u32) -> bool {
        unsafe { from_glib(gtk_sys::gtk_bitset_remove(self.to_glib_none().0, value)) }
    }

    pub fn remove_all(&self) {
        unsafe {
            gtk_sys::gtk_bitset_remove_all(self.to_glib_none().0);
        }
    }

    pub fn remove_range(&self, start: u32, n_items: u32) {
        unsafe {
            gtk_sys::gtk_bitset_remove_range(self.to_glib_none().0, start, n_items);
        }
    }

    pub fn remove_range_closed(&self, first: u32, last: u32) {
        unsafe {
            gtk_sys::gtk_bitset_remove_range_closed(self.to_glib_none().0, first, last);
        }
    }

    pub fn remove_rectangle(&self, start: u32, width: u32, height: u32, stride: u32) {
        unsafe {
            gtk_sys::gtk_bitset_remove_rectangle(
                self.to_glib_none().0,
                start,
                width,
                height,
                stride,
            );
        }
    }

    pub fn shift_left(&self, amount: u32) {
        unsafe {
            gtk_sys::gtk_bitset_shift_left(self.to_glib_none().0, amount);
        }
    }

    pub fn shift_right(&self, amount: u32) {
        unsafe {
            gtk_sys::gtk_bitset_shift_right(self.to_glib_none().0, amount);
        }
    }

    pub fn splice(&self, position: u32, removed: u32, added: u32) {
        unsafe {
            gtk_sys::gtk_bitset_splice(self.to_glib_none().0, position, removed, added);
        }
    }

    pub fn subtract(&self, other: &Bitset) {
        unsafe {
            gtk_sys::gtk_bitset_subtract(self.to_glib_none().0, other.to_glib_none().0);
        }
    }

    pub fn union(&self, other: &Bitset) {
        unsafe {
            gtk_sys::gtk_bitset_union(self.to_glib_none().0, other.to_glib_none().0);
        }
    }
}
