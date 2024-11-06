// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(SettingsInputCompositeSharp)]
pub fn settings_input_composite_sharp(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("SettingsInputCompositeSharp"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M0 0h24v24H0V0z" fill="none"/><path d="M5 2c0-.55-.45-1-1-1s-1 .45-1 1v4H1v6h6V6H5V2zm4 16.82h2V23h2v-4.18h2V14H9v4.82zm-8 0h2V23h2v-4.18h2V14H1v4.82zM21 6V2c0-.55-.45-1-1-1s-1 .45-1 1v4h-2v6h6V6h-2zm-8-4c0-.55-.45-1-1-1s-1 .45-1 1v4H9v6h6V6h-2V2zm4 16.82h2V23h2v-4.18h2V14h-6v4.82z"/>
        </SvgIcon>
    }
}
