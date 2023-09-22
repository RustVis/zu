// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License that can be
// found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(FeaturedPlayListSharp)]
pub fn featured_play_list_sharp(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("FeaturedPlayListSharp"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M0 0h24v24H0V0z" fill="none"/><path d="M23 3H1v18h22V3zm-11 8H3V9h9v2zm0-4H3V5h9v2z"/>
        </SvgIcon>
    }
}
