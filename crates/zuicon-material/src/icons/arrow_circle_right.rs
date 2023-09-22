// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License that can be
// found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(ArrowCircleRight)]
pub fn arrow_circle_right(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("ArrowCircleRight"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M22,12c0-5.52-4.48-10-10-10S2,6.48,2,12c0,5.52,4.48,10,10,10S22,17.52,22,12z M12,13l-4,0v-2l4,0V8l4,4l-4,4V13z"/>
        </SvgIcon>
    }
}
