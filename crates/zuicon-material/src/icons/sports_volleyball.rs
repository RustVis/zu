// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(SportsVolleyball)]
pub fn sports_volleyball(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("SportsVolleyball"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M6,4.01C3.58,5.84,2,8.73,2,12c0,1.46,0.32,2.85,0.89,4.11L6,14.31V4.01z"/><path d="M11,11.42V2.05C9.94,2.16,8.93,2.43,8,2.84v10.32L11,11.42z"/><path d="M12,13.15l-8.11,4.68c0.61,0.84,1.34,1.59,2.18,2.2L15,14.89L12,13.15z"/><path d="M13,7.96v3.46l8.11,4.68c0.42-0.93,0.7-1.93,0.82-2.98L13,7.96z"/><path d="M8.07,21.2C9.28,21.71,10.6,22,12,22c3.34,0,6.29-1.65,8.11-4.16L17,16.04L8.07,21.2z"/><path d="M21.92,10.81c-0.55-4.63-4.26-8.3-8.92-8.76v3.6L21.92,10.81z"/>
        </SvgIcon>
    }
}
