// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(KitchenSharp)]
pub fn abc(props: &Props) -> Html {
    html! {
        <SvgIcon classes={props.classes.clone()}
            color={props.color}
            font_size={props.font_size}
            html_color={props.html_color.clone()}
            style={props.style.clone()}
            title_access={props.title_access.clone()}
            icon="KitchenSharp"
            view_box={props.view_box.clone()}
            >
            <path d="M0 0h24v24H0V0z" fill="none"/><path d="M20 2.01 4 2v20h16V2.01zM18 20H6v-9.02h12V20zm0-11H6V4h12v5zM8 5h2v3H8V5zm0 7h2v5H8v-5z"/>
        </SvgIcon>
    }
}
