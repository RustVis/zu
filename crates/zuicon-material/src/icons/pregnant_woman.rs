// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(PregnantWoman)]
pub fn pregnant_woman(props: &Props) -> Html {
    html! {
        <SvgIcon classes={props.classes.clone()}
            color={props.color}
            font_size={props.font_size}
            html_color={props.html_color.clone()}
            style={props.style.clone()}
            title_access={props.title_access.clone()}
            icon="PregnantWoman"
            view_box={props.view_box.clone()}
            >
            <path d="M9,4c0-1.11,0.89-2,2-2s2,0.89,2,2s-0.89,2-2,2S9,5.11,9,4z M16,13c-0.01-1.34-0.83-2.51-2-3c0-1.66-1.34-3-3-3 s-3,1.34-3,3v7h2v5h3v-5h3V13z"/>
        </SvgIcon>
    }
}
