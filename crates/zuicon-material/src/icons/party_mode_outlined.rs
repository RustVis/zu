// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(PartyModeOutlined)]
pub fn party_mode_outlined(props: &Props) -> Html {
    html! {
        <SvgIcon classes={props.classes.clone()}
            color={props.color}
            font_size={props.font_size}
            html_color={props.html_color.clone()}
            style={props.style.clone()}
            title_access={props.title_access.clone()}
            icon="PartyModeOutlined"
            view_box={props.view_box.clone()}
            >
            <path d="M0 0h24v24H0V0z" fill="none"/><path d="M20 4h-3.17L15 2H9L7.17 4H4c-1.1 0-2 .9-2 2v12c0 1.1.9 2 2 2h16c1.1 0 2-.9 2-2V6c0-1.1-.9-2-2-2zm0 14H4V6h4.05l.59-.65L9.88 4h4.24l1.24 1.35.59.65H20v12zM9 12c0-1.66 1.34-3 3-3h3.98c-.92-1.21-2.35-2-3.98-2-2.76 0-5 2.24-5 5 0 .34.04.68.1 1h2.08c-.11-.31-.18-.65-.18-1zm6 0c0 1.66-1.34 3-3 3H8.02c.92 1.21 2.35 2 3.98 2 2.76 0 5-2.24 5-5 0-.34-.03-.68-.1-1h-2.08c.11.31.18.65.18 1z"/>
        </SvgIcon>
    }
}
