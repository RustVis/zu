// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License that
// can be found in the LICENSE file.

//! Detects when the floating or reference element is overflowing a clipping container
//! or custom boundary.

use crate::middleware::{Boundary, ElementContext, MiddlewareState, RootBoundary};
use crate::types::{ClientRectObject, Padding, Rect, Scale, SideObject};

#[derive(Debug, Clone)]
pub struct DetectOverflowOption {
    pub boundary: Boundary,
    pub root_boundary: RootBoundary,
    pub element_context: ElementContext,
    pub alt_boundary: bool,
    pub padding: Padding,
}

impl Default for DetectOverflowOption {
    fn default() -> Self {
        Self {
            boundary: Boundary::ClippingAncestors,
            root_boundary: RootBoundary::Viewport,
            element_context: ElementContext::Floating,
            alt_boundary: false,
            padding: Padding::default(),
        }
    }
}

#[must_use]
pub fn detect_overflow(state: &MiddlewareState, option: &DetectOverflowOption) -> SideObject {
    let platform = &state.platform;
    let elements = &state.elements;

    let padding = &option.padding;
    let element_context = option.element_context;

    let padding_object: SideObject = padding.clone().into();
    let alt_context = option.element_context.alter();
    let element = if option.alt_boundary {
        elements.element(alt_context)
    } else {
        elements.element(element_context)
    };

    let clipping_client_rect: ClientRectObject = {
        // TODO(Shaohua): Call platform.is_element()
        let clipping_rect: Rect = platform.clipping_rect(
            &element,
            &option.boundary,
            &option.root_boundary,
            state.strategy,
        );
        clipping_rect.into()
    };

    let rect: Rect = if element_context == ElementContext::Floating {
        let coords = &state.coords;
        let floating = &state.rects.floating;
        Rect {
            x: coords.x,
            y: coords.y,
            width: floating.width,
            height: floating.height,
        }
    } else {
        state.rects.reference.clone()
    };

    let offset_parent = platform.offset_parent(&elements.floating);
    let offset_scale: Scale = offset_parent
        .as_ref()
        .map_or(Scale { x: 1.0, y: 1.0 }, |offset_parent| {
            platform.scale(offset_parent)
        });

    let element_client_rect: ClientRectObject =
        offset_parent
            .as_ref()
            .map_or(rect.clone().into(), |offset_parent| {
                let offset_rect =
                    platform.convert_relative_rect(&rect, offset_parent, state.strategy);
                offset_rect.into()
            });

    let top = (clipping_client_rect.side.top - element_client_rect.side.top + padding_object.top)
        / offset_scale.y;
    let bottom = (element_client_rect.side.bottom - clipping_client_rect.side.bottom
        + padding_object.bottom)
        / offset_scale.y;
    let left = (clipping_client_rect.side.left - element_client_rect.side.left
        + padding_object.left)
        / offset_scale.x;
    let right = (element_client_rect.side.right - clipping_client_rect.side.right
        + padding_object.right)
        / offset_scale.x;

    SideObject {
        top,
        right,
        bottom,
        left,
    }
}
