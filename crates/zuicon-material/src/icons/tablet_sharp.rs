// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(TabletSharp)]
pub fn tablet_sharp(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("TabletSharp"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M0 0h24v24H0V0z" fill="none"/><path d="M23 4H1v16h21.99L23 4zm-4 14H5V6h14v12z"/>
        </SvgIcon>
    }
}
