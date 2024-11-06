// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(Shop2Rounded)]
pub fn shop_2_rounded(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("Shop2Rounded"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M2,9L2,9c-0.55,0-1,0.45-1,1v10c0,1.1,0.9,2,2,2h15c0.55,0,1-0.45,1-1v0c0-0.55-0.45-1-1-1H3V10C3,9.45,2.55,9,2,9z"/><path d="M18,5V3c0-1.11-0.89-2-2-2h-4c-1.11,0-2,0.89-2,2v2H5v11c0,1.11,0.89,2,2,2h14c1.11,0,2-0.89,2-2V5H18z M12,3h4v2h-4V3z M12,14.09V8.91c0-0.39,0.44-0.63,0.77-0.42l4.07,2.59c0.31,0.2,0.31,0.65,0,0.84l-4.07,2.59C12.44,14.72,12,14.48,12,14.09z"/>
        </SvgIcon>
    }
}
