// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(ViewSidebarRounded)]
pub fn view_sidebar_rounded(props: &Props) -> Html {
    html! {
        <SvgIcon classes={props.classes.clone()}
            color={props.color}
            font_size={props.font_size}
            html_color={props.html_color.clone()}
            style={props.style.clone()}
            title_access={props.title_access.clone()}
            icon="ViewSidebarRounded"
            view_box={props.view_box.clone()}
            >
            <path d="M15,20H3c-0.55,0-1-0.45-1-1V5c0-0.55,0.45-1,1-1h12c0.55,0,1,0.45,1,1v14C16,19.55,15.55,20,15,20z M19,8h2 c0.55,0,1-0.45,1-1V5c0-0.55-0.45-1-1-1h-2c-0.55,0-1,0.45-1,1v2C18,7.55,18.45,8,19,8z M19,20h2c0.55,0,1-0.45,1-1v-2 c0-0.55-0.45-1-1-1h-2c-0.55,0-1,0.45-1,1v2C18,19.55,18.45,20,19,20z M19,14h2c0.55,0,1-0.45,1-1v-2c0-0.55-0.45-1-1-1h-2 c-0.55,0-1,0.45-1,1v2C18,13.55,18.45,14,19,14z"/>
        </SvgIcon>
    }
}
