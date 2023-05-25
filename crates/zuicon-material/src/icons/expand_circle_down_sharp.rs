// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(ExpandCircleDownSharp)]
pub fn expand_circle_down_sharp(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("ExpandCircleDownSharp"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M12,2C6.48,2,2,6.48,2,12c0,5.52,4.48,10,10,10s10-4.48,10-10C22,6.48,17.52,2,12,2z M12,15.5L7.5,11l1.42-1.41L12,12.67 l3.08-3.08L16.5,11L12,15.5z"/>
        </SvgIcon>
    }
}
