// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(TakeoutDiningTwoTone)]
pub fn takeout_dining_two_tone(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("TakeoutDiningTwoTone"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M20.59,6.05L19,7.63l0.03-0.56L14.98,3H9.02L4.97,7.07L5,7.57L3.41,6.01L2,7.44l3.23,3.11L5.93,20h12.14l0.7-9.44L22,7.46 L20.59,6.05z M16.23,18H7.79l-0.51-7h9.46L16.23,18z M16.87,9H7.12L7.03,7.73L9.83,5h4.33l2.8,2.73L16.87,9z"/>
        </SvgIcon>
    }
}
