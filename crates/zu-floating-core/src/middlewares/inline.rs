// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

//! Improves positioning for inline reference elements that span over multiple lines.

use crate::middleware::{Middleware, MiddlewareData, MiddlewareReturn, MiddlewareState};
use crate::platform::ElementRects;
use crate::types::{ClientRect, Padding, Rect, SideObject};

#[derive(Debug, Default, Clone, PartialEq)]
pub struct InlineMiddlewareData {
    pub rects: ElementRects,
}

impl InlineMiddlewareData {
    #[must_use]
    #[inline]
    pub fn from(data: &MiddlewareData) -> Option<&Self> {
        data.get(Inline::NAME).map(|boxed| boxed.downcast_ref())?
    }
}

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
    pub padding: Padding,
}

impl Default for Inline {
    fn default() -> Self {
        // A MouseEvent's client{X,Y} coords can be up to 2 pixels off a
        // ClientRectObject's bounds, despite the event listener being triggered. A
        // padding of 2 seems to handle this issue.
        Self {
            x: None,
            y: None,
            padding: Padding::Number(2.0),
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

    fn run(&self, state: &MiddlewareState) -> MiddlewareReturn {
        let mut native_client_rects = state
            .platform
            .client_rects(&state.elements.reference)
            .unwrap_or_default();
        let client_rects = get_rect_by_line(&mut native_client_rects);
        let fallback = get_bounding_rect(&native_client_rects).client_rect();
        let padding_object: SideObject = self.padding.clone().into();

        #[allow(clippy::no_effect_underscore_binding)]
        let _get_bounding_client_rect = || -> ClientRect {
            // There are two rects and they are disjointed.
            if client_rects.len() == 2
                && client_rects[0].side.left > client_rects[1].side.left
                && self.x.is_some()
                && self.y.is_some()
            {
                let x = self.x.unwrap_or_default();
                let y = self.y.unwrap_or_default();
                // Find the first rect in which the point is fully inside.
                return client_rects
                    .iter()
                    .find(|rect| {
                        x > rect.side.left - padding_object.left
                            && x < rect.side.right + padding_object.right
                            && y > rect.side.top - padding_object.top
                            && y < rect.side.bottom + padding_object.bottom
                    })
                    .unwrap_or(&fallback)
                    .clone();
            }
            // TODO(Shaohua): There are two or more connected rects.

            fallback
        };

        let reset_rects = state.platform.element_rects(
            &state.elements.reference,
            &state.elements.floating,
            state.strategy,
        );

        if state.rects.reference == reset_rects.reference {
            MiddlewareReturn::default()
        } else {
            let inline_data = Box::new(InlineMiddlewareData { rects: reset_rects });
            let data = MiddlewareData::with_value(Self::NAME, inline_data);
            MiddlewareReturn {
                data,
                ..Default::default()
            }
        }
    }
}

pub fn get_bounding_rect(rects: &[ClientRect]) -> Rect {
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

    let mut groups: Vec<Vec<ClientRect>> = Vec::new();
    let mut prev_rect: Option<&ClientRect> = None;
    for rect in rects {
        let new_vec = if let Some(prev_rect) = prev_rect {
            rect.rect.y - prev_rect.rect.y > prev_rect.rect.height / 2.0
        } else {
            true
        };

        if new_vec {
            groups.push(vec![rect.clone()]);
        } else if let Some(last_vec) = groups.last_mut() {
            last_vec.push(rect.clone());
        }
        prev_rect = Some(rect);
    }

    groups
        .into_iter()
        .map(|rect_list| get_bounding_rect(&rect_list).client_rect())
        .collect()
}
