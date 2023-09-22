// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License that can be
// found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(ModeEditOutlineSharp)]
pub fn mode_edit_outline_sharp(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("ModeEditOutlineSharp"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M3,17.25V21h3.75L17.81,9.94l-3.75-3.75L3,17.25z M21.41,6.34l-3.75-3.75l-2.53,2.54l3.75,3.75L21.41,6.34z"/>
        </SvgIcon>
    }
}
