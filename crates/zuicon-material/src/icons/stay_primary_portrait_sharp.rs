// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(StayPrimaryPortraitSharp)]
pub fn stay_primary_portrait_sharp(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("StayPrimaryPortraitSharp"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M0 0h24v24H0V0z" fill="none"/><path d="M5.01 1v22H19V1H5.01zM17 19H7V5h10v14z"/>
        </SvgIcon>
    }
}
