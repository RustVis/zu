// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(AddCardSharp)]
pub fn add_card_sharp(props: &Props) -> Html {
    html! {
        <SvgIcon classes={props.classes.clone()}
            color={props.color}
            font_size={props.font_size}
            html_color={props.html_color.clone()}
            style={props.style.clone()}
            title_access={props.title_access.clone()}
            icon="AddCardSharp"
            view_box={props.view_box.clone()}
            >
            <path d="M2.01,4L2,20h12v-2H4v-6h18V4H2.01z M20,8H4V6h16V8z M24,17v2h-3v3h-2v-3h-3v-2h3v-3h2v3H24z"/>
        </SvgIcon>
    }
}
