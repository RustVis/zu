// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(EditLocationAlt)]
pub fn edit_location_alt(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("EditLocationAlt"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M13.95,13H9V8.05l5.61-5.61C13.78,2.16,12.9,2,12,2c-4.2,0-8,3.22-8,8.2c0,3.32,2.67,7.25,8,11.8c5.33-4.55,8-8.48,8-11.8 c0-1.01-0.16-1.94-0.45-2.8L13.95,13z"/><path d="M20.71,2L20,1.29C19.8,1.1,19.55,1,19.29,1c-0.13,0-0.48,0.07-0.71,0.29l-0.72,0.72l2.12,2.12l0.72-0.72 C21.1,3.02,21.1,2.39,20.71,2z"/>
        </SvgIcon>
    }
}
