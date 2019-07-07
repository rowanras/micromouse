use std::f64;

use plotters::drawing::backend::BackendCoord;
use plotters::drawing::backend::BackendStyle;
use plotters::drawing::backend::DrawingBackend;
use plotters::drawing::backend::DrawingErrorKind;
use plotters::style::Color;
use plotters::style::FontDesc;
use plotters::style::FontTransform;
use plotters::style::RGBAColor;

use cairo::Context;
use cairo::FontSlant;
use cairo::FontWeight;

#[derive(Debug)]
pub struct CairoBackendError {}

impl std::error::Error for CairoBackendError {}

impl std::fmt::Display for CairoBackendError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        write!(f, "Cairo backend error!")
    }
}

pub struct CairoBackend<'a> {
    cr: &'a Context,
    size: (u32, u32),
}

impl<'a> CairoBackend<'a> {
    pub fn new(cr: &'a Context, width: u32, height: u32) -> CairoBackend {
        CairoBackend {
            cr,
            size: (width, height),
        }
    }
}

impl<'a> DrawingBackend for CairoBackend<'a> {
    type ErrorType = CairoBackendError;

    fn get_size(&self) -> (u32, u32) {
        self.size
    }

    fn ensure_prepared(
        &mut self,
    ) -> Result<(), DrawingErrorKind<CairoBackendError>> {
        Ok(())
    }

    fn present(&mut self) -> Result<(), DrawingErrorKind<CairoBackendError>> {
        Ok(())
    }

    fn draw_pixel(
        &mut self,
        point: BackendCoord,
        color: &RGBAColor,
    ) -> Result<(), DrawingErrorKind<CairoBackendError>> {
        self.cr.set_line_width(1.0);
        self.cr.set_source_rgba(
            color.rgb().0 as f64,
            color.rgb().1 as f64,
            color.rgb().2 as f64,
            color.alpha(),
        );
        self.cr.move_to(point.0 as f64, point.1 as f64);
        self.cr.rel_line_to(1.0, 1.0);
        self.cr.stroke();

        Ok(())
    }

    fn draw_line<S: BackendStyle>(
        &mut self,
        from: BackendCoord,
        to: BackendCoord,
        style: &S,
    ) -> Result<(), DrawingErrorKind<CairoBackendError>> {
        self.cr.set_line_width(1.0);
        self.cr.set_source_rgba(
            style.as_color().rgb().0 as f64,
            style.as_color().rgb().1 as f64,
            style.as_color().rgb().2 as f64,
            style.as_color().alpha(),
        );
        self.cr.move_to(from.0 as f64, from.1 as f64);
        self.cr.line_to(to.0 as f64, from.1 as f64);
        self.cr.stroke();

        Ok(())
    }

    fn draw_rect<S: BackendStyle>(
        &mut self,
        upper_left: BackendCoord,
        bottom_right: BackendCoord,
        style: &S,
        fill: bool,
    ) -> Result<(), DrawingErrorKind<CairoBackendError>> {
        self.cr.set_line_width(1.0);
        self.cr.set_source_rgba(
            style.as_color().rgb().0 as f64,
            style.as_color().rgb().1 as f64,
            style.as_color().rgb().2 as f64,
            style.as_color().alpha(),
        );
        self.cr.rectangle(
            upper_left.0 as f64,
            upper_left.1 as f64,
            bottom_right.0 as f64 - upper_left.0 as f64,
            bottom_right.1 as f64 - upper_left.1 as f64,
        );

        if fill {
            self.cr.fill();
        } else {
            self.cr.stroke();
        }

        Ok(())
    }

    fn draw_path<S: BackendStyle, I: IntoIterator<Item = BackendCoord>>(
        &mut self,
        path: I,
        style: &S,
    ) -> Result<(), DrawingErrorKind<CairoBackendError>> {
        self.cr.set_line_width(1.0);
        self.cr.set_source_rgba(
            style.as_color().rgb().0 as f64,
            style.as_color().rgb().1 as f64,
            style.as_color().rgb().2 as f64,
            style.as_color().alpha(),
        );

        let mut path_iter = path.into_iter();

        if let Some(first_point) = path_iter.next() {
            self.cr.move_to(first_point.0 as f64, first_point.1 as f64);
        }

        for point in path_iter {
            self.cr.line_to(point.0 as f64, point.1 as f64);
        }

        self.cr.stroke();

        Ok(())
    }

    fn draw_circle<S: BackendStyle>(
        &mut self,
        center: BackendCoord,
        radius: u32,
        style: &S,
        fill: bool,
    ) -> Result<(), DrawingErrorKind<CairoBackendError>> {
        self.cr.set_line_width(1.0);
        self.cr.set_source_rgba(
            style.as_color().rgb().0 as f64,
            style.as_color().rgb().1 as f64,
            style.as_color().rgb().2 as f64,
            style.as_color().alpha(),
        );

        self.cr.move_to(center.0 as f64, center.1 as f64);
        self.cr.arc(
            center.0 as f64,
            center.1 as f64,
            radius as f64,
            0.0,
            2.0 * f64::consts::PI,
        );

        if fill {
            self.cr.fill();
        } else {
            self.cr.stroke();
        }

        Ok(())
    }

    fn draw_text(
        &mut self,
        text: &str,
        font: &FontDesc,
        pos: BackendCoord,
        color: &RGBAColor,
    ) -> Result<(), DrawingErrorKind<CairoBackendError>> {
        self.cr.set_source_rgba(
            color.rgb().0 as f64,
            color.rgb().1 as f64,
            color.rgb().2 as f64,
            color.alpha(),
        );

        self.cr
            .translate(pos.0 as f64, pos.1 as f64 + font.get_size());

        self.cr.rotate(match font.get_transform() {
            FontTransform::None => 0.0,
            FontTransform::Rotate90 => f64::consts::PI / 2.0,
            FontTransform::Rotate180 => f64::consts::PI,
            FontTransform::Rotate270 => -f64::consts::PI / 2.0,
        });

        self.cr.select_font_face(
            font.get_name(),
            FontSlant::Normal,
            FontWeight::Normal,
        );

        self.cr.set_font_size(font.get_size());
        self.cr.show_text(text);

        self.cr.rotate(match font.get_transform() {
            FontTransform::None => 0.0,
            FontTransform::Rotate90 => -f64::consts::PI / 2.0,
            FontTransform::Rotate180 => f64::consts::PI,
            FontTransform::Rotate270 => f64::consts::PI / 2.0,
        });

        self.cr
            .translate(-pos.0 as f64, -(pos.1 as f64 + font.get_size()));

        //self.cr.identity_matrix();

        Ok(())
    }
}
