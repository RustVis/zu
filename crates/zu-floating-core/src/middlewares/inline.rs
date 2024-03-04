// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License that can be found
// in the LICENSE file.

//! Improves positioning for inline reference elements that span over multiple lines.

use crate::middleware::{Middleware, MiddlewareReturn, MiddlewareState};
use crate::types::{ClientRect, Padding, Rect};

#[derive(Debug, Clone)]
pub struct Inline {
    /// Viewport-relative `x` coordinate to choose a `ClientRect`.
    ///
    /// Default is None.
    pub x: Option<f64>,

    /// Viewport-relative `y` coordinate to choose a `ClientRect`.
    ///
    /// Default is None.
    pub y: Option<f64>,

    /// Represents the padding around a disjointed rect when choosing it.
    ///
    /// Default is 2
    pub padding: Option<Padding>,
}

impl Default for Inline {
    fn default() -> Self {
        // A MouseEvent's client{X,Y} coords can be up to 2 pixels off a
        // ClientRectObject's bounds, despite the event listener being triggered. A
        // padding of 2 seems to handle this issue.
        Self {
            x: None,
            y: None,
            padding: Some(Padding::Number(2.0)),
        }
    }
}

impl Inline {
    pub const NAME: &'static str = "inline";
}

impl Middleware for Inline {
    fn name(&self) -> &str {
        Self::NAME
    }

    fn run(&self, _state: &MiddlewareState) -> MiddlewareReturn {
        todo!()
    }
}

pub fn get_bounding_rect(rects: &[&ClientRect]) -> Rect {
    let min_x = rects
        .iter()
        .map(|obj| obj.rect.x)
        .min_by(f64::total_cmp)
        .unwrap_or_default();
    let min_y = rects
        .iter()
        .map(|obj| obj.rect.y)
        .min_by(f64::total_cmp)
        .unwrap_or_default();
    let min_width = rects
        .iter()
        .map(|obj| obj.rect.width)
        .min_by(f64::total_cmp)
        .unwrap_or_default();
    let min_height = rects
        .iter()
        .map(|obj| obj.rect.height)
        .min_by(f64::total_cmp)
        .unwrap_or_default();
    Rect {
        x: min_x,
        y: min_y,
        width: min_width,
        height: min_height,
    }
}

pub fn get_rect_by_line(rects: &mut [ClientRect]) -> Vec<ClientRect> {
    rects.sort_by(|a, b| a.rect.y.total_cmp(&b.rect.y));

    let mut groups: Vec<Vec<&ClientRect>> = Vec::new();
    let mut prev_rect: Option<&ClientRect> = None;
    for rect in rects {
        let new_vec = if let Some(prev_rect) = prev_rect {
            rect.rect.y - prev_rect.rect.y > prev_rect.rect.height / 2.0
        } else {
            true
        };

        if new_vec {
            groups.push(vec![rect]);
        } else if let Some(last_vec) = groups.last_mut() {
            last_vec.push(rect);
        }
        prev_rect = Some(rect);
    }

    groups
        .into_iter()
        .map(|rect_list| get_bounding_rect(&rect_list).client_rect())
        .collect()
}
