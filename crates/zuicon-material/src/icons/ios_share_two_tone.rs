// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License that can be
// found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(IosShareTwoTone)]
pub fn ios_share_two_tone(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("IosShareTwoTone"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M18,8h-3v2h3v11H6V10h3V8H6c-1.11,0-2,0.89-2,2v11c0,1.1,0.89,2,2,2h12c1.1,0,2-0.9,2-2V10C20,8.89,19.1,8,18,8z"/>
        </SvgIcon>
    }
}
