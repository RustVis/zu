// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(CssRounded)]
pub fn css_rounded(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("CssRounded"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M8,10.25C8,10.66,7.66,11,7.25,11c-0.33,0-0.6-0.21-0.71-0.5l-2.04,0v3l2.04,0c0.1-0.29,0.38-0.5,0.71-0.5 C7.66,13,8,13.34,8,13.75V14c0,0.55-0.45,1-1,1H4c-0.55,0-1-0.45-1-1v-4c0-0.55,0.45-1,1-1h3c0.55,0,1,0.45,1,1V10.25z M13.04,10.5 c0.1,0.29,0.38,0.5,0.71,0.5c0.41,0,0.75-0.34,0.75-0.75V10c0-0.55-0.45-1-1-1h-3c-0.55,0-1,0.45-1,1v1.5c0,0.55,0.45,1,1,1H13v1 h-2.04v0c-0.1-0.29-0.38-0.5-0.71-0.5c-0.41,0-0.75,0.34-0.75,0.75V14c0,0.55,0.45,1,1,1h3c0.55,0,1-0.45,1-1v-1.5 c0-0.55-0.45-1-1-1H11v-1L13.04,10.5z M19.54,10.5c0.1,0.29,0.38,0.5,0.71,0.5c0.41,0,0.75-0.34,0.75-0.75V10c0-0.55-0.45-1-1-1h-3 c-0.55,0-1,0.45-1,1v1.5c0,0.55,0.45,1,1,1h2.5v1h-2.04v0c-0.1-0.29-0.38-0.5-0.71-0.5C16.34,13,16,13.34,16,13.75V14 c0,0.55,0.45,1,1,1h3c0.55,0,1-0.45,1-1v-1.5c0-0.55-0.45-1-1-1h-2.5v-1L19.54,10.5z"/>
        </SvgIcon>
    }
}
