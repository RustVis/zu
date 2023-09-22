// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License that can be
// found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(PausePresentationSharp)]
pub fn pause_presentation_sharp(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("PausePresentationSharp"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M0 0h24v24H0V0z" fill="none"/><path d="M1 3v18h22V3H1zm20 16H3V5h18v14zM9 8h2v8H9zm4 0h2v8h-2z"/>
        </SvgIcon>
    }
}
