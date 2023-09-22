// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License that can be
// found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(SystemUpdateAltSharp)]
pub fn system_update_alt_sharp(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("SystemUpdateAltSharp"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M0 0h24v24H0V0zm0 0h24v24H0V0z" fill="none"/><path d="M12 16l4-4h-3V3h-2v9H8l4 4zm9-13h-6v1.99h6v14.03H3V4.99h6V3H3c-1.1 0-2 .9-2 2v14c0 1.1.9 2 2 2h18c1.1 0 2-.9 2-2V5c0-1.1-.9-2-2-2zm-9 13l4-4h-3V3h-2v9H8l4 4zM23 3h-8v1.99h6v14.03H3V4.99h6V3H1v18h22V3z"/>
        </SvgIcon>
    }
}
