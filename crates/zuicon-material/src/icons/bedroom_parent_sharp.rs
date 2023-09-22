// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License that can be
// found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(BedroomParentSharp)]
pub fn bedroom_parent_sharp(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("BedroomParentSharp"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M0,0h24v24H0V0z" fill="none"/><path d="M22,2H2v20h20V2z M19,17h-1.5v-1.5h-11V17H5v-5l0.65-0.55V7H11c0.37,0,0.72,0.12,1,0.32C12.28,7.12,12.63,7,13,7h5.35 v4.45L19,12V17z"/>
        </SvgIcon>
    }
}
