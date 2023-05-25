// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(SimCardDownloadRounded)]
pub fn sim_card_download_rounded(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("SimCardDownloadRounded"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M0,0h24v24H0V0z" fill="none"/><path d="M18,2h-7.17C10.3,2,9.79,2.21,9.41,2.59L4.59,7.41C4.21,7.79,4,8.3,4,8.83V20c0,1.1,0.9,2,2,2h12c1.1,0,2-0.9,2-2V4 C20,2.9,19.1,2,18,2z M11.65,16.65l-2.79-2.79C8.54,13.54,8.76,13,9.21,13H11v-2.99c0-0.55,0.44-0.99,0.99-1h0 C12.55,9,13,9.45,13,10.01V13h1.79c0.45,0,0.67,0.54,0.35,0.85l-2.79,2.79C12.16,16.84,11.84,16.84,11.65,16.65z"/>
        </SvgIcon>
    }
}
