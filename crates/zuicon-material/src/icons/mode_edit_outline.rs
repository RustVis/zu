// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(ModeEditOutline)]
pub fn mode_edit_outline(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("ModeEditOutline"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M20.71,5.63l-2.34-2.34c-0.39-0.39-1.02-0.39-1.41,0l-1.83,1.83l3.75,3.75l1.83-1.83C21.1,6.65,21.1,6.02,20.71,5.63z"/>
        </SvgIcon>
    }
}
