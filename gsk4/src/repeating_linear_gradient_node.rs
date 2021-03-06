use glib::translate::*;
use gsk_sys;
use std::fmt;
use graphene::{Rect, Point};
use RenderNode;
use ColorStop;
use RepeatingLinearGradientNode;


impl RepeatingLinearGradientNode {
    pub fn new(
        bounds: &Rect,
        start: &Point,
        end: &Point,
        color_stops: &[ColorStop],
    ) -> RepeatingLinearGradientNode {
        assert_initialized_main_thread!();
        let n_color_stops = color_stops.len() as usize;
        unsafe {
            from_glib_full(gsk_sys::gsk_repeating_linear_gradient_node_new(
                bounds.to_glib_none().0,
                start.to_glib_none().0,
                end.to_glib_none().0,
                color_stops.to_glib_none().0,
                n_color_stops,
            ))
        }
    }
}
