// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(SimCardAlertSharp)]
pub fn sim_card_alert_sharp(props: &Props) -> Html {
    html! {
        <SvgIcon classes={props.classes.clone()}
            color={props.color}
            font_size={props.font_size}
            html_color={props.html_color.clone()}
            style={props.style.clone()}
            title_access={props.title_access.clone()}
            icon="SimCardAlertSharp"
            view_box={props.view_box.clone()}
            >
            <path d="M20,2H10L4,8v14h16V2z M13,17h-2v-2h2V17z M13,13h-2V8h2V13z"/>
        </SvgIcon>
    }
}
