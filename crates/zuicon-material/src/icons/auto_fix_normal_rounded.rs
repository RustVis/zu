// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(AutoFixNormalRounded)]
pub fn auto_fix_normal_rounded(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("AutoFixNormalRounded"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M20.45,6l0.49-1.06L22,4.45c0.39-0.18,0.39-0.73,0-0.91l-1.06-0.49L20.45,2c-0.18-0.39-0.73-0.39-0.91,0l-0.49,1.06 L18,3.55c-0.39,0.18-0.39,0.73,0,0.91l1.06,0.49L19.55,6C19.72,6.39,20.28,6.39,20.45,6z"/><path d="M17.71,9.12l-2.83-2.83c-0.39-0.39-1.02-0.39-1.41,0L2.29,17.46c-0.39,0.39-0.39,1.02,0,1.41l2.83,2.83 c0.39,0.39,1.02,0.39,1.41,0l11.17-11.17C18.1,10.15,18.1,9.51,17.71,9.12z M14.21,11.21l-1.41-1.41l1.38-1.38l1.41,1.41 L14.21,11.21z"/>
        </SvgIcon>
    }
}
