// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(WebAssetOffTwoTone)]
pub fn web_asset_off_two_tone(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("WebAssetOffTwoTone"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M20,17.17V8h-9.17L20,17.17z M5.17,8H4v10h11.17L5.17,8z" opacity=".3"/><path d="M6.83,4H20c1.11,0,2,0.9,2,2v12c0,0.34-0.09,0.66-0.23,0.94L20,17.17V8h-9.17L6.83,4z M20.49,23.31L17.17,20H4 c-1.11,0-2-0.9-2-2V6c0-0.34,0.08-0.66,0.23-0.94L0.69,3.51L2.1,2.1l19.8,19.8L20.49,23.31z M15.17,18l-10-10H4v10H15.17z"/>
        </SvgIcon>
    }
}
