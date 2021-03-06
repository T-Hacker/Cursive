use Printer;
use theme::ColorStyle;
use vec::Vec2;
use view::{View, ViewWrapper};

/// Wrapper view that adds a shadow.
///
/// It reserves a 1 pixel border on each side.
pub struct ShadowView<T: View> {
    view: T,
    top_padding: bool,
    left_padding: bool,
}

impl<T: View> ShadowView<T> {
    /// Wraps the given view.
    pub fn new(view: T) -> Self {
        ShadowView {
            view: view,
            top_padding: true,
            left_padding: true,
        }
    }

    fn padding(&self) -> Vec2 {
        Vec2::new(1 + self.left_padding as usize,
                  1 + self.top_padding as usize)
    }

    /// If set, adds an empty column to the left of the view.
    ///
    /// Default to true.
    pub fn left_padding(mut self, value: bool) -> Self {
        self.left_padding = value;
        self
    }

    /// If set, adds an empty row at the top of the view.
    ///
    /// Default to true.
    pub fn top_padding(mut self, value: bool) -> Self {
        self.top_padding = value;
        self
    }
}

impl<T: View> ViewWrapper for ShadowView<T> {
    wrap_impl!(self.view: T);

    fn wrap_required_size(&mut self, req: Vec2) -> Vec2 {
        // Make sure req >= offset
        let offset = self.padding();
        self.view.required_size(req.saturating_sub(offset)) + offset
    }

    fn wrap_layout(&mut self, size: Vec2) {
        let offset = self.padding();
        self.view.layout(size.saturating_sub(offset));
    }

    fn wrap_draw(&self, printer: &Printer) {

        if printer.size.y <= self.top_padding as usize ||
           printer.size.x <= self.left_padding as usize {
            // Nothing to do if there's no place to draw.
            return;
        }

        // Skip the first row/column
        let offset = Vec2::new(self.left_padding as usize,
                               self.top_padding as usize);
        let printer = &printer.offset(offset, true);
        if printer.theme.shadow {
            let h = printer.size.y;
            let w = printer.size.x;

            if h == 0 || w == 0 {
                return;
            }

            printer.with_color(ColorStyle::Shadow, |printer| {
                printer.print_hline((1, h - 1), w - 1, " ");
                printer.print_vline((w - 1, 1), h - 1, " ");
            });
        }

        // Draw the view background
        let printer = printer.sub_printer(Vec2::zero(),
                                          printer.size.saturating_sub((1, 1)),
                                          true);
        self.view.draw(&printer);
    }
}
