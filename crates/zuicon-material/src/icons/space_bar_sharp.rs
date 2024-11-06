// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(SpaceBarSharp)]
pub fn space_bar_sharp(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("SpaceBarSharp"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M18 9v4H6V9H4v6h16V9h-2z"/>
        </SvgIcon>
    }
}
