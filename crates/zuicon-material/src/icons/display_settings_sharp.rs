// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License that can be
// found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(DisplaySettingsSharp)]
pub fn display_settings_sharp(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("DisplaySettingsSharp"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M22,3H2v16h6v2h8v-2h6V3z M20,17H4V5h16V17z"/>
        </SvgIcon>
    }
}
