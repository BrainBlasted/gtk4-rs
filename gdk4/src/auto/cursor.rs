// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use gdk_sys;
use glib::object::Cast;
use glib::object::IsA;
use glib::translate::*;
use glib::GString;
use glib::StaticType;
use glib::ToValue;
use std::fmt;
use Texture;

glib_wrapper! {
    pub struct Cursor(Object<gdk_sys::GdkCursor>);

    match fn {
        get_type => || gdk_sys::gdk_cursor_get_type(),
    }
}

impl Cursor {
    pub fn from_name(name: &str, fallback: Option<&Cursor>) -> Option<Cursor> {
        assert_initialized_main_thread!();
        unsafe {
            from_glib_full(gdk_sys::gdk_cursor_new_from_name(
                name.to_glib_none().0,
                fallback.to_glib_none().0,
            ))
        }
    }

    pub fn from_texture<P: IsA<Texture>>(
        texture: &P,
        hotspot_x: i32,
        hotspot_y: i32,
        fallback: Option<&Cursor>,
    ) -> Cursor {
        skip_assert_initialized!();
        unsafe {
            from_glib_full(gdk_sys::gdk_cursor_new_from_texture(
                texture.as_ref().to_glib_none().0,
                hotspot_x,
                hotspot_y,
                fallback.to_glib_none().0,
            ))
        }
    }

    pub fn get_fallback(&self) -> Option<Cursor> {
        unsafe { from_glib_none(gdk_sys::gdk_cursor_get_fallback(self.to_glib_none().0)) }
    }

    pub fn get_hotspot_x(&self) -> i32 {
        unsafe { gdk_sys::gdk_cursor_get_hotspot_x(self.to_glib_none().0) }
    }

    pub fn get_hotspot_y(&self) -> i32 {
        unsafe { gdk_sys::gdk_cursor_get_hotspot_y(self.to_glib_none().0) }
    }

    pub fn get_name(&self) -> Option<GString> {
        unsafe { from_glib_none(gdk_sys::gdk_cursor_get_name(self.to_glib_none().0)) }
    }

    pub fn get_texture(&self) -> Option<Texture> {
        unsafe { from_glib_none(gdk_sys::gdk_cursor_get_texture(self.to_glib_none().0)) }
    }
}

#[derive(Clone, Default)]
pub struct CursorBuilder {
    fallback: Option<Cursor>,
    hotspot_x: Option<i32>,
    hotspot_y: Option<i32>,
    name: Option<String>,
    texture: Option<Texture>,
}

impl CursorBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn build(self) -> Cursor {
        let mut properties: Vec<(&str, &dyn ToValue)> = vec![];
        if let Some(ref fallback) = self.fallback {
            properties.push(("fallback", fallback));
        }
        if let Some(ref hotspot_x) = self.hotspot_x {
            properties.push(("hotspot-x", hotspot_x));
        }
        if let Some(ref hotspot_y) = self.hotspot_y {
            properties.push(("hotspot-y", hotspot_y));
        }
        if let Some(ref name) = self.name {
            properties.push(("name", name));
        }
        if let Some(ref texture) = self.texture {
            properties.push(("texture", texture));
        }
        let ret = glib::Object::new(Cursor::static_type(), &properties)
            .expect("object new")
            .downcast::<Cursor>()
            .expect("downcast");
        ret
    }

    pub fn fallback(mut self, fallback: &Cursor) -> Self {
        self.fallback = Some(fallback.clone());
        self
    }

    pub fn hotspot_x(mut self, hotspot_x: i32) -> Self {
        self.hotspot_x = Some(hotspot_x);
        self
    }

    pub fn hotspot_y(mut self, hotspot_y: i32) -> Self {
        self.hotspot_y = Some(hotspot_y);
        self
    }

    pub fn name(mut self, name: &str) -> Self {
        self.name = Some(name.to_string());
        self
    }

    pub fn texture<P: IsA<Texture>>(mut self, texture: &P) -> Self {
        self.texture = Some(texture.clone().upcast());
        self
    }
}

impl fmt::Display for Cursor {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Cursor")
    }
}
