// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(LabelSharp)]
pub fn label_sharp(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("LabelSharp"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M0 0h24v24H0V0z" fill="none"/><path d="M17.03 5L3 5.01v13.98l14.03.01L22 12l-4.97-7z"/>
        </SvgIcon>
    }
}
