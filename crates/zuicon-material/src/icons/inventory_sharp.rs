// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(InventorySharp)]
pub fn abc(props: &Props) -> Html {
    html! {
        <SvgIcon classes={props.classes.clone()}
            color={props.color}
            font_size={props.font_size}
            html_color={props.html_color.clone()}
            style={props.style.clone()}
            title_access={props.title_access.clone()}
            icon="InventorySharp"
            view_box={props.view_box.clone()}
            >
            <path d="M5,5h2v3h10V5h2v5h2V3h-6.18C14.4,1.84,13.3,1,12,1S9.6,1.84,9.18,3H3v18h8v-2H5V5z M12,3c0.55,0,1,0.45,1,1s-0.45,1-1,1 s-1-0.45-1-1S11.45,3,12,3z"/>
        </SvgIcon>
    }
}