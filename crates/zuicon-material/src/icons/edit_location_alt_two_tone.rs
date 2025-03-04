// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(EditLocationAltTwoTone)]
pub fn edit_location_alt_two_tone(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("EditLocationAltTwoTone"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M17.9,9.05C17.96,9.41,18,9.79,18,10.2c0,1.71-1.08,4.64-6,9.14c-4.92-4.49-6-7.43-6-9.14C6,6.17,9.09,4,12,4 c0.32,0,0.65,0.03,0.97,0.08l1.65-1.65C13.78,2.16,12.9,2,12,2c-4.2,0-8,3.22-8,8.2c0,3.32,2.67,7.25,8,11.8 c5.33-4.55,8-8.48,8-11.8c0-1.01-0.16-1.94-0.45-2.8L17.9,9.05z M20.71,2L20,1.29c-0.39-0.39-1.02-0.39-1.41,0l-0.72,0.72 l2.12,2.12l0.72-0.72C21.1,3.02,21.1,2.39,20.71,2z M11,11h2.12l6.16-6.16l-2.12-2.12L11,8.88V11z"/><path d="M13.95,13H9V8.05l3.97-3.97C12.65,4.03,12.32,4,12,4c-2.91,0-6,2.17-6,6.2 c0,1.71,1.08,4.64,6,9.14c4.92-4.49,6-7.43,6-9.14c0-0.4-0.04-0.78-0.1-1.15L13.95,13z" enable-background="new" opacity=".3"/>
        </SvgIcon>
    }
}
