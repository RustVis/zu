// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(TurnSlightRightRounded)]
pub fn turn_slight_right_rounded(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("TurnSlightRightRounded"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M12.34,5L12.34,5c0-0.55,0.45-1,1-1H17c0.55,0,1,0.45,1,1v3.66c0,0.55-0.45,1-1,1h0c-0.55,0-1-0.45-1-1V7.41l-5,5V19 c0,0.55-0.45,1-1,1h0c-0.55,0-1-0.45-1-1v-6.58c0-0.53,0.21-1.04,0.59-1.41l5-5h-1.24C12.79,6,12.34,5.55,12.34,5z"/>
        </SvgIcon>
    }
}
