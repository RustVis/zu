// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(AirplayRounded)]
pub fn airplay_rounded(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("AirplayRounded"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M8.41 22h7.17c.89 0 1.34-1.08.71-1.71L12.7 16.7c-.39-.39-1.02-.39-1.41 0L7.7 20.29c-.62.63-.18 1.71.71 1.71zM21 3H3c-1.1 0-2 .9-2 2v12c0 1.1.9 2 2 2h3c.55 0 1-.45 1-1s-.45-1-1-1H4c-.55 0-1-.45-1-1V6c0-.55.45-1 1-1h16c.55 0 1 .45 1 1v10c0 .55-.45 1-1 1h-2c-.55 0-1 .45-1 1s.45 1 1 1h3c1.1 0 2-.9 2-2V5c0-1.1-.9-2-2-2z"/>
        </SvgIcon>
    }
}
