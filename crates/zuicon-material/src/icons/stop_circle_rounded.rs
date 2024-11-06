// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(StopCircleRounded)]
pub fn stop_circle_rounded(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("StopCircleRounded"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M12,2C6.48,2,2,6.48,2,12c0,5.52,4.48,10,10,10s10-4.48,10-10C22,6.48,17.52,2,12,2z M15,16H9c-0.55,0-1-0.45-1-1V9 c0-0.55,0.45-1,1-1h6c0.55,0,1,0.45,1,1v6C16,15.55,15.55,16,15,16z"/>
        </SvgIcon>
    }
}
