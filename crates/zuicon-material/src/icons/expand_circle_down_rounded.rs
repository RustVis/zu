// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(ExpandCircleDownRounded)]
pub fn expand_circle_down_rounded(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("ExpandCircleDownRounded"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M12,2C6.48,2,2,6.48,2,12c0,5.52,4.48,10,10,10s10-4.48,10-10C22,6.48,17.52,2,12,2z M15.79,11.71l-3.08,3.08 c-0.39,0.39-1.02,0.39-1.42,0l-3.08-3.08c-0.39-0.39-0.39-1.03,0-1.42c0.39-0.39,1.02-0.39,1.41,0L12,12.67l2.38-2.38 c0.39-0.39,1.02-0.39,1.41,0C16.18,10.68,16.18,11.32,15.79,11.71z"/>
        </SvgIcon>
    }
}
