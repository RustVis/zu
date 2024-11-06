// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(BrowserNotSupportedSharp)]
pub fn browser_not_supported_sharp(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("BrowserNotSupportedSharp"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M3.22,3.32L1.95,4.59L3,5.64L3,20h14.36l2.06,2.06l1.27-1.27L3.22,3.32z M15,18H5V7.64L15.36,18H15z"/>
        </SvgIcon>
    }
}
