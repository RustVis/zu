// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License that can be
// found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(FormatColorTextTwoTone)]
pub fn format_color_text_two_tone(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("FormatColorTextTwoTone"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M2,20h20v4H2V20z M5.49,17h2.42l1.27-3.58h5.65L16.09,17h2.42L13.25,3h-2.5L5.49,17z M9.91,11.39l2.03-5.79h0.12l2.03,5.79 H9.91z"/>
        </SvgIcon>
    }
}
