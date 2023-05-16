// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(WalletSharp)]
pub fn abc(props: &Props) -> Html {
    html! {
        <SvgIcon classes={props.classes.clone()}
            color={props.color}
            font_size={props.font_size}
            html_color={props.html_color.clone()}
            style={props.style.clone()}
            title_access={props.title_access.clone()}
            icon="WalletSharp"
            view_box={props.view_box.clone()}
            >
            <path d="M22,4H2v16h20V4z M15.75,14.09L4,11.22V10h16v0.53L15.75,14.09z M4,6h16v2H4V6z"/>
        </SvgIcon>
    }
}
