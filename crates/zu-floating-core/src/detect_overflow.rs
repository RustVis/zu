// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License that can be found
// in the LICENSE file.

//! Detects when the floating or reference element is overflowing a clipping container
//! or custom boundary.

use crate::middleware::MiddlewareState;
use crate::platform::{Boundary, ElementContext, RootBoundary};
use crate::types::{ClientRect, Padding, Rect, Scale, SideObject};

#[derive(Debug, Default, Clone)]
pub struct DetectOverflowOption {
    /// This describes the clipping element(s) or area that overflow will be checked relative to.
    pub boundary: Boundary,

    /// This describes the root boundary that the element will be checked for overflow relative to.
    pub root_boundary: RootBoundary,

    /// This describes the virtual padding around the boundary to check for overflow.
    pub padding: Padding,

    /// By default, the floating element is the one being checked for overflow.
    pub element_context: ElementContext,

    /// This is a boolean value which determines whether to check the alternate
    /// element context’s boundary.
    ///
    /// Default is `false`.
    pub alt_boundary: bool,
}

/// A clipping container (or boundary) is one that causes child elements inside it to be clipped
/// if they overflow it.
///
/// Visibility optimizer middleware use this function for collision detection, making it useful
/// for your own custom middleware that do the same.
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

    let clipping_client_rect: ClientRect = {
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
        .map_or_else(Scale::default, |offset_parent| {
            platform.scale(offset_parent)
        });

    let element_client_rect: ClientRect =
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
