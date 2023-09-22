// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License that can be
// found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(InvertColorsOffRounded)]
pub fn invert_colors_off_rounded(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("InvertColorsOffRounded"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M20.49,20.49L3.51,3.51c-0.39-0.39-1.02-0.39-1.41,0l0,0c-0.39,0.39-0.39,1.02,0,1.41l3.5,3.5c-1,1.31-1.6,2.94-1.6,4.7 C4,17.48,7.58,21,12,21c1.75,0,3.36-0.56,4.67-1.5l2.4,2.4c0.39,0.39,1.02,0.39,1.41,0l0,0C20.88,21.51,20.88,20.88,20.49,20.49z M12,19c-3.31,0-6-2.63-6-5.87c0-1.19,0.36-2.32,1.02-3.28L12,14.83V19z M8.38,5.56l2.91-2.87c0.39-0.38,1.01-0.38,1.4,0l4.95,4.87 l0,0C19.1,8.99,20,10.96,20,13.13c0,1.18-0.27,2.29-0.74,3.3L12,9.17V4.81L9.8,6.97L8.38,5.56z"/>
        </SvgIcon>
    }
}
