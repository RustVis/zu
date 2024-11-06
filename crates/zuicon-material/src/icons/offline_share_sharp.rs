// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(OfflineShareSharp)]
pub fn offline_share_sharp(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("OfflineShareSharp"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M0,0h24v24H0V0z" fill="none"/><path d="M20,1H8v18h12V1z M18,15h-8V5h8V15z"/>
        </SvgIcon>
    }
}
