// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(ViewTimelineSharp)]
pub fn view_timeline_sharp(props: &Props) -> Html {
    html! {
        <SvgIcon classes={props.classes.clone()}
            color={props.color}
            font_size={props.font_size}
            html_color={props.html_color.clone()}
            style={props.style.clone()}
            title_access={props.title_access.clone()}
            icon="ViewTimelineSharp"
            view_box={props.view_box.clone()}
            >
            <path d="M21,3H3v18h18V3z M12,17H6v-2h6V17z M15,13H9v-2h6V13z M18,9h-6V7h6V9z"/>
        </SvgIcon>
    }
}
