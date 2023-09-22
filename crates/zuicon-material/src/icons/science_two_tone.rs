// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License that can be
// found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(ScienceTwoTone)]
pub fn science_two_tone(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("ScienceTwoTone"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M20.8,18.4L15,10.67V6.5l1.35-1.69C16.61,4.48,16.38,4,15.96,4H8.04C7.62,4,7.39,4.48,7.65,4.81L9,6.5v4.17L3.2,18.4 C2.71,19.06,3.18,20,4,20h16C20.82,20,21.29,19.06,20.8,18.4z M6,18l5-6.67V6h2v5.33L18,18H6z"/>
        </SvgIcon>
    }
}
