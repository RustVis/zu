// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(EditCalendar)]
pub fn edit_calendar(props: &Props) -> Html {
    html! {
        <SvgIcon classes={props.classes.clone()}
            color={props.color}
            font_size={props.font_size}
            html_color={props.html_color.clone()}
            style={props.style.clone()}
            title_access={props.title_access.clone()}
            icon="EditCalendar"
            view_box={props.view_box.clone()}
            >
            <path d="M12,22H5c-1.11,0-2-0.9-2-2L3.01,6c0-1.1,0.88-2,1.99-2h1V2h2v2h8V2h2v2h1c1.1,0,2,0.9,2,2v6h-2v-2H5v10h7V22z M22.13,16.99 l0.71-0.71c0.39-0.39,0.39-1.02,0-1.41l-0.71-0.71c-0.39-0.39-1.02-0.39-1.41,0l-0.71,0.71L22.13,16.99z M21.42,17.7l-5.3,5.3H14 v-2.12l5.3-5.3L21.42,17.7z"/>
        </SvgIcon>
    }
}
