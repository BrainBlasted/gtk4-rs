use auto::BorderNode;
use glib::translate::*;
use glib::IsA;

pub trait BorderNodeManualExt {
    fn peek_widths(&self) -> &[f32; 4];
}

impl<O: IsA<BorderNode>> BorderNodeManualExt for O {
    fn peek_widths(&self) -> &[f32; 4] {
        unsafe {
            (gsk_sys::gsk_border_node_peek_widths(self.as_ref().to_glib_none().0)
                as *const &[f32; 4])
                .read()
        }
    }
}
