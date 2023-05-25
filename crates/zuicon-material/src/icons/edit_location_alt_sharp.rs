// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(EditLocationAltSharp)]
pub fn edit_location_alt_sharp(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("EditLocationAltSharp"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M13.95,13H9V8.05l5.61-5.61C13.78,2.16,12.9,2,12,2c-4.2,0-8,3.22-8,8.2c0,3.32,2.67,7.25,8,11.8c5.33-4.55,8-8.48,8-11.8 c0-1.01-0.16-1.94-0.45-2.8L13.95,13z M11,11h2.12l6.16-6.16l-2.12-2.12L11,8.88V11z M19.29,0.59l-1.42,1.42l2.12,2.12l1.42-1.42 L19.29,0.59z"/>
        </SvgIcon>
    }
}
