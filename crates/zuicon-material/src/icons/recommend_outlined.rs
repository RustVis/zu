// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(RecommendOutlined)]
pub fn abc(props: &Props) -> Html {
    html! {
        <SvgIcon classes={props.classes.clone()}
            color={props.color}
            font_size={props.font_size}
            html_color={props.html_color.clone()}
            style={props.style.clone()}
            title_access={props.title_access.clone()}
            icon="RecommendOutlined"
            view_box={props.view_box.clone()}
            >
            <path d="M12,2C6.48,2,2,6.48,2,12s4.48,10,10,10s10-4.48,10-10S17.52,2,12,2z M12,20c-4.41,0-8-3.59-8-8s3.59-8,8-8s8,3.59,8,8 S16.41,20,12,20z"/><path d="M17,10h-4.59l0.58-3.41v-0.2c-0.01-0.26-0.12-0.51-0.3-0.7L12,5l-4.6,5c-0.27,0.26-0.42,0.62-0.4,1v5c0,1.1,0.9,2,2,2h5.5 c0.56,0.03,1.08-0.29,1.3-0.8l2.1-4.9c0.08-0.15,0.12-0.33,0.1-0.5V11C18,10.45,17.55,10,17,10z"/>
        </SvgIcon>
    }
}
