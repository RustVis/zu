// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(Shop2Sharp)]
pub fn shop_2_sharp(props: &Props) -> Html {
    html! {
        <SvgIcon classes={props.classes.clone()}
            color={props.color}
            font_size={props.font_size}
            html_color={props.html_color.clone()}
            style={props.style.clone()}
            title_access={props.title_access.clone()}
            icon="Shop2Sharp"
            view_box={props.view_box.clone()}
            >
            <path d="M18,5V1h-8v4H5v13h18V5H18z M12,3h4v2h-4V3z M12,15V8l5.5,3.5L12,15z"/>
        </SvgIcon>
    }
}
