// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(WebAssetSharp)]
pub fn web_asset_sharp(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("WebAssetSharp"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M0 0h24v24H0V0z" fill="none"/><path d="M3 4v16h18V4H3zm16 14H5V8h14v10z"/>
        </SvgIcon>
    }
}
