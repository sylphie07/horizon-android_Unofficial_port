//! Track total drag distance separately from per-event scroll deltas.

/// A touch drag may scroll a list only when it began inside the list's
/// horizontal and vertical viewport. Coordinates are in logical UI units.
pub(super) fn contains_origin(view: [f32; 5], x: f64, y: f64) -> bool {
    let [left, top, right, bottom, _] = view;
    x >= left as f64 && x <= right as f64 && y >= top as f64 && y <= bottom as f64
}
/// Scroll only enough to reveal the selected row, retaining the current offset.
pub(super) fn reveal(scroll: f32, row: [f32; 4], view: [f32; 5]) -> f32 {
    let [_, top, _, bottom, maximum] = view;
    let adjustment = if row[1] < top {
        row[1] - top
    } else if row[1] + row[3] > bottom {
        row[1] + row[3] - bottom
    } else {
        0.0
    };
    (scroll + adjustment).clamp(0.0, maximum.max(0.0))
}
pub(super) struct Drag {
    origin: [f64; 2],
    previous_y: f64,
    pub(super) moved: bool,
}

impl Drag {
    pub(super) fn new(x: f64, y: f64) -> Self {
        Self {
            origin: [x, y],
            previous_y: y,
            moved: false,
        }
    }

    pub(super) fn origin_x(&self) -> f64 {
        self.origin[0]
    }

    pub(super) fn origin_y(&self) -> f64 {
        self.origin[1]
    }

    pub(super) fn update(&mut self, x: f64, y: f64, density: f64) -> f32 {
        let density = density.max(0.01);
        self.moved |= (x - self.origin[0]).hypot(y - self.origin[1]) > 12.0 * density;
        let delta = if self.moved {
            ((self.previous_y - y) / density) as f32
        } else {
            0.0
        };
        self.previous_y = y;
        delta
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn focus_scroll_reveals_rows_in_both_directions_and_stops_at_bounds() {
        assert_eq!(
            reveal(0.0, [0.0, 350.0, 100.0, 60.0], [0.0, 100.0, 600.0, 400.0, 500.0]),
            10.0
        );
        assert_eq!(
            reveal(200.0, [0.0, 80.0, 100.0, 60.0], [0.0, 100.0, 600.0, 400.0, 500.0]),
            180.0
        );
        assert_eq!(
            reveal(20.0, [0.0, 150.0, 100.0, 60.0], [0.0, 100.0, 600.0, 400.0, 500.0]),
            20.0
        );
        assert_eq!(
            reveal(490.0, [0.0, 700.0, 100.0, 60.0], [0.0, 100.0, 600.0, 400.0, 500.0]),
            500.0
        );
    }

    #[test]
    fn slow_vertical_drag_crosses_the_total_threshold() {
        let mut drag = Drag::new(100.0, 300.0);
        for y in [297.0, 294.0, 291.0, 288.0] {
            assert_eq!(drag.update(100.0, y, 1.0), 0.0);
        }
        assert_eq!(drag.update(100.0, 285.0, 1.0), 3.0);
        assert!(drag.moved);
        assert_eq!(drag.origin_y(), 300.0);
        assert_eq!(drag.update(100.0, 290.0, 1.0), -5.0);
    }


    #[test]
    fn drag_origin_must_be_inside_both_viewport_axes() {
        let view = [24.0, 140.0, 720.0, 680.0, 320.0];
        assert!(contains_origin(view, 24.0, 140.0));
        assert!(contains_origin(view, 500.0, 400.0));
        assert!(!contains_origin(view, 23.9, 400.0));
        assert!(!contains_origin(view, 720.1, 400.0));
        assert!(!contains_origin(view, 500.0, 139.9));
        assert!(!contains_origin(view, 500.0, 680.1));
    }

    #[test]
    fn density_scales_threshold_and_scroll_together() {
        let mut drag = Drag::new(200.0, 600.0);
        assert_eq!(drag.update(200.0, 580.0, 2.0), 0.0);
        assert_eq!(drag.update(200.0, 570.0, 2.0), 5.0);
        assert!(drag.moved);
    }
}
