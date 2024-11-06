// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(SystemUpdateAlt)]
pub fn system_update_alt(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("SystemUpdateAlt"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M0 .5h24v24H0z" fill="none"/><path d="M12 16.5l4-4h-3v-9h-2v9H8l4 4zm9-13h-6v1.99h6v14.03H3V5.49h6V3.5H3c-1.1 0-2 .9-2 2v14c0 1.1.9 2 2 2h18c1.1 0 2-.9 2-2v-14c0-1.1-.9-2-2-2z"/>
        </SvgIcon>
    }
}
