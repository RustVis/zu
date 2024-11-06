// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(NearbyErrorSharp)]
pub fn nearby_error_sharp(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("NearbyErrorSharp"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M0,0h24v24H0V0z" fill="none"/><path d="M12,7.58L16.42,12L12,16.42L7.58,12L12,7.58z M12,19.2L4.8,12L12,4.8l6,6V7.17l-5.99-5.99L1.18,12.01l10.83,10.83 L18,16.83V13.2L12,19.2z M20,20h2v2h-2V20z M22,10h-2v8h2V10"/>
        </SvgIcon>
    }
}
