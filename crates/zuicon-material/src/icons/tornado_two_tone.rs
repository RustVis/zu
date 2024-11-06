// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(TornadoTwoTone)]
pub fn tornado_two_tone(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("TornadoTwoTone"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M1,3l11,19L23,3H1z M12,18.01L10.26,15h3.48L12,18.01z M14.9,13H9.1l-1.74-3h9.27L14.9,13z M6.21,8L4.47,5h15.06l-1.74,3 H6.21z"/>
        </SvgIcon>
    }
}
