// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(LocalCafeTwoTone)]
pub fn local_cafe_two_tone(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("LocalCafeTwoTone"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M0 0h24v24H0V0z" fill="none"/><path d="M8 15h6c1.1 0 2-.9 2-2V5H6v8c0 1.1.9 2 2 2z" opacity=".3"/><path d="M2 19h18v2H2zm2-6c0 2.21 1.79 4 4 4h6c2.21 0 4-1.79 4-4v-3h2c1.11 0 2-.89 2-2V5c0-1.11-.89-2-2-2H4v10zm14-8h2v3h-2V5zM6 5h10v8c0 1.1-.9 2-2 2H8c-1.1 0-2-.9-2-2V5z"/>
        </SvgIcon>
    }
}
