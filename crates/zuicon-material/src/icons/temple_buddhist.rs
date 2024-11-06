// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(TempleBuddhist)]
pub fn temple_buddhist(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("TempleBuddhist"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M21,9.02c0,1.09-0.89,1.98-1.98,1.98H4.98C3.89,11,3,10.11,3,9.02H1c0,1.86,1.28,3.4,3,3.84V22h6v-3c0-1.1,0.9-2,2-2 s2,0.9,2,2v3h6v-9.14c0.55-0.14,3-1.04,3-3.86L21,9.02z"/><path d="M6,8.86V10h12V8.86c0.55-0.14,3-1.04,3-3.86l-2,0.02C19,6.11,18.11,7,17.02,7H6.98C5.89,7,5,6.11,5,5.02H3 C3,6.87,4.28,8.42,6,8.86z"/>
        </SvgIcon>
    }
}
