// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(BorderClearTwoTone)]
pub fn border_clear_two_tone(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("BorderClearTwoTone"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M0 0h24v24H0V0z" fill="none"/><path d="M7 3h2v2H7zm0 16h2v2H7zM3 3h2v2H3zm16 0h2v2h-2zm0 4h2v2h-2zm0 4h2v2h-2zM3 7h2v2H3zm0 12h2v2H3zm16 0h2v2h-2zm0-4h2v2h-2zM3 15h2v2H3zm0-4h2v2H3zm4 0h2v2H7zm8 0h2v2h-2zm-4 8h2v2h-2zm4 0h2v2h-2zm0-16h2v2h-2zm-4 0h2v2h-2zm0 4h2v2h-2zm0 8h2v2h-2zm0-4h2v2h-2z"/>
        </SvgIcon>
    }
}
