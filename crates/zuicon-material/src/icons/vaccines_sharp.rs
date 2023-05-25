// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(VaccinesSharp)]
pub fn vaccines_sharp(props: &Props) -> Html {
    html! {
        <SvgIcon classes={props.classes.clone()}
            color={props.color}
            font_size={props.font_size}
            html_color={props.html_color.clone()}
            style={props.style.clone()}
            title_access={props.title_access.clone()}
            icon="VaccinesSharp"
            view_box={props.view_box.clone()}
            >
            <path d="M12,5.5H8V4h1.5V2h-5v2H6v1.5H2v2c0,0,0.45,0,1,0V17h3v4l2,1.5V17h3V7.5c0.55,0,1,0,1,0V5.5z M9,9H6.5v1.5H9V12H6.5v1.5H9 L9,15H5V7.5h4V9z M19.5,10.5V10h1V8h-7l-0.01,2h1.01v0.5c0,0.5-1.5,1.16-1.5,3V22h8v-8.5C21,11.66,19.5,11,19.5,10.5z M16.5,10.5V10 h1v0.5c0,1.6,1.5,2,1.5,3V14h-4c0-0.21,0-0.39,0-0.5C15,12.5,16.5,12.1,16.5,10.5z M15,20c0,0,0-0.63,0-1.5h4V20H15z"/>
        </SvgIcon>
    }
}
