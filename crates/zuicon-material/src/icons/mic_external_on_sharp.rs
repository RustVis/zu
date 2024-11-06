// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(MicExternalOnSharp)]
pub fn mic_external_on_sharp(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("MicExternalOnSharp"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M9.22,7H4.78C4.3,6.47,4,5.77,4,5c0-1.66,1.34-3,3-3s3,1.34,3,3C10,5.77,9.7,6.47,9.22,7z M20,2v20h-2V4h-4v18H6 c0,0,0-1.79,0-4H5L4,8h6L9,18H8v2h4V2H20z"/>
        </SvgIcon>
    }
}
