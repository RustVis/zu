// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(SatelliteRounded)]
pub fn satellite_rounded(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("SatelliteRounded"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M0 0h24v24H0V0z" fill="none"/><path d="M19 3H5c-1.1 0-2 .9-2 2v14c0 1.1.9 2 2 2h14c1.1 0 2-.9 2-2V5c0-1.1-.9-2-2-2zM5 5h3c0 1.66-1.34 3-3 3V5zm0 5.91c0-.49.36-.9.85-.98 2.08-.36 3.72-2 4.08-4.08.08-.49.49-.85.98-.85.61 0 1.09.53 1 1.13-.48 2.96-2.81 5.3-5.77 5.78-.6.1-1.14-.39-1.14-1zm.63 6.28l2.49-3.2c.2-.25.58-.26.78-.01l2.1 2.53 3.1-3.99c.2-.26.6-.26.8.01l3.51 4.68c.25.33.01.8-.4.8H6.02c-.41-.01-.65-.49-.39-.82z"/>
        </SvgIcon>
    }
}
