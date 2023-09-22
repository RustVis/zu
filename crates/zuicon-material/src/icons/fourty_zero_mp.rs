// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License that can be
// found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(FourtyZeroMp)]
pub fn fourty_zero_mp(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("FourtyZeroMp"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M0 0h24v24H0V0z" fill="none"/><path d="M14.5 7H16v3h-1.5zM19 3H5c-1.1 0-2 .9-2 2v14c0 1.1.9 2 2 2h14c1.1 0 2-.9 2-2V5c0-1.1-.9-2-2-2zm-7 15.5h-1.5V14h-1v3H8v-3H7v4.5H5.5v-5c0-.55.45-1 1-1H11c.55 0 1 .45 1 1v5zm3.5 0H14v-6h3.5c.55 0 1 .45 1 1V16c0 .55-.45 1-1 1h-2v1.5zm2-8c0 .55-.45 1-1 1H14c-.55 0-1-.45-1-1v-4c0-.55.45-1 1-1h2.5c.55 0 1 .45 1 1v4zM10 9H8v1h3v1.5H6.5V9c0-.55.45-1 1-1h2V7h-3V5.5H10c.55 0 1 .45 1 1V8c0 .55-.45 1-1 1zm5.5 5H17v1.5h-1.5z"/>
        </SvgIcon>
    }
}
