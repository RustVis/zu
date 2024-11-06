// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(DoNotTouchSharp)]
pub fn do_not_touch_sharp(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("DoNotTouchSharp"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M13,10.17l-2.5-2.5V1H13V10.17z M20,4h-2.5v7h-1V2H14v9.17l6,6V4z M9.5,3H7.01v1.18L9.5,6.67V3z M21.19,21.19L2.81,2.81 L1.39,4.22L7,9.83v4.3l-3.32-1.9L2,13.88L9.68,22h9.54l0.56,0.61L21.19,21.19z"/>
        </SvgIcon>
    }
}
