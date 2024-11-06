// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(SignalCellularNoSim)]
pub fn signal_cellular_no_sim(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("SignalCellularNoSim"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M-618-2872H782V728H-618zM-1 0h26v24H-1zm1 0h24v24H0z" fill="none"/><path d="M18.99 5c0-1.1-.89-2-1.99-2h-7L7.66 5.34 19 16.68 18.99 5zM3.65 3.88L2.38 5.15 5 7.77V19c0 1.1.9 2 2 2h10.01c.35 0 .67-.1.96-.26l1.88 1.88 1.27-1.27L3.65 3.88z"/><path d="M.01 0h24v24h-24z" fill="none"/>
        </SvgIcon>
    }
}
