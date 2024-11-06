// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(ExpandMoreSharp)]
pub fn expand_more_sharp(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("ExpandMoreSharp"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M24 24H0V0h24v24z" fill="none" opacity=".87"/><path d="M16.59 8.59L12 13.17 7.41 8.59 6 10l6 6 6-6-1.41-1.41z"/>
        </SvgIcon>
    }
}
