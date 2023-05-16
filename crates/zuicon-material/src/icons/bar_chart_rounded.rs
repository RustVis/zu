// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(BarChartRounded)]
pub fn abc(props: &Props) -> Html {
    html! {
        <SvgIcon classes={props.classes.clone()}
            color={props.color}
            font_size={props.font_size}
            html_color={props.html_color.clone()}
            style={props.style.clone()}
            title_access={props.title_access.clone()}
            icon="BarChartRounded"
            view_box={props.view_box.clone()}
            >
            <path d="M6,20L6,20c1.1,0,2-0.9,2-2v-7c0-1.1-0.9-2-2-2h0c-1.1,0-2,0.9-2,2v7C4,19.1,4.9,20,6,20z"/><path d="M16,15v3c0,1.1,0.9,2,2,2h0c1.1,0,2-0.9,2-2v-3c0-1.1-0.9-2-2-2h0C16.9,13,16,13.9,16,15z"/><path d="M12,20L12,20c1.1,0,2-0.9,2-2V6c0-1.1-0.9-2-2-2h0c-1.1,0-2,0.9-2,2v12C10,19.1,10.9,20,12,20z"/>
        </SvgIcon>
    }
}
