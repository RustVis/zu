// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(ContrastTwoTone)]
pub fn contrast_two_tone(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("ContrastTwoTone"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M12,22c5.52,0,10-4.48,10-10S17.52,2,12,2S2,6.48,2,12S6.48,22,12,22z M13,4.07c3.94,0.49,7,3.85,7,7.93 s-3.05,7.44-7,7.93V4.07z"/>
        </SvgIcon>
    }
}
