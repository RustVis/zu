// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(CommentsDisabledRounded)]
pub fn comments_disabled_rounded(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("CommentsDisabledRounded"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M1.39,2.81C1,3.2,1,3.83,1.39,4.22L2,4.83V16c0,1.1,0.9,2,2,2h11.17l4.61,4.61c0.39,0.39,1.02,0.39,1.41,0 c0.39-0.39,0.39-1.02,0-1.41L2.81,2.81C2.42,2.42,1.78,2.42,1.39,2.81z M6.38,9.21L8.17,11H7c-0.55,0-1-0.45-1-1 C6,9.68,6.15,9.4,6.38,9.21z M7,14c-0.55,0-1-0.45-1-1c0-0.55,0.45-1,1-1h2.17l2,2H7z M14.83,12l-1-1H17c0.55,0,1-0.45,1-1v0 c0-0.55-0.45-1-1-1h-5.17l-1-1H17c0.55,0,1-0.45,1-1v0c0-0.55-0.45-1-1-1H8.83l-4-4H20c1.1,0,2,0.9,2,2v15.17L16.83,14H17 c0.55,0,1-0.45,1-1v0c0-0.55-0.45-1-1-1H14.83z"/>
        </SvgIcon>
    }
}
