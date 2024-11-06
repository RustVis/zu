// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(SdSharp)]
pub fn sd_sharp(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("SdSharp"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M2,4v16h20V4H2z M13,9h4c0.55,0,1,0.45,1,1v4c0,0.55-0.45,1-1,1h-4V9z M9.5,13.5v-1H6V9h5v2H9.5v-0.5h-2v1H11V15H6v-2h1.5 v0.5H9.5z M14.5,13.5h2v-3h-2V13.5z"/>
        </SvgIcon>
    }
}
