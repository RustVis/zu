// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(ViewStreamSharp)]
pub fn view_stream_sharp(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("ViewStreamSharp"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M3,19v-6h18v6H3z M3,5v6h18V5H3z"/>
        </SvgIcon>
    }
}
