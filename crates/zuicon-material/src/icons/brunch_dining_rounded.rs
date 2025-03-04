// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(BrunchDiningRounded)]
pub fn brunch_dining_rounded(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("BrunchDiningRounded"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M18,8h2V4h-2V8z M15,22H3c-0.55,0-1-0.45-1-1v-1h14v1C16,21.55,15.55,22,15,22z M18,15.89l-0.4-0.42 c-1.03-1.08-1.6-2.51-1.6-4V3c0-0.55,0.45-1,1-1h4c0.55,0,1,0.45,1,1v8.51c0,1.46-0.54,2.87-1.53,3.94L20,15.97V20h1 c0.55,0,1,0.45,1,1v0c0,0.55-0.45,1-1,1h-2c-0.55,0-1-0.45-1-1V15.89z M7,16v-1c0-0.55,0.45-1,1-1h2c0.55,0,1,0.45,1,1v1h4 c0.55,0,1,0.45,1,1v1H2v-1c0-0.55,0.45-1,1-1H7z"/>
        </SvgIcon>
    }
}
