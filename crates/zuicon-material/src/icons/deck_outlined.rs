// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License that can be
// found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(DeckOutlined)]
pub fn deck_outlined(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("DeckOutlined"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M22,9L12,2L2,9h9v13h2V9H22z M12,4.44L15.66,7H8.34L12,4.44z"/>
        </SvgIcon>
    }
}
