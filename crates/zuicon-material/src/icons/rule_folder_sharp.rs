// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(RuleFolderSharp)]
pub fn rule_folder_sharp(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("RuleFolderSharp"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M22,6H12l-2-2H2v16h20V6z M7.83,16L5,13.17l1.41-1.41l1.41,1.41l3.54-3.54l1.41,1.41L7.83,16z M17.41,13L19,14.59L17.59,16 L16,14.41L14.41,16L13,14.59L14.59,13L13,11.41L14.41,10L16,11.59L17.59,10L19,11.41L17.41,13z"/>
        </SvgIcon>
    }
}
