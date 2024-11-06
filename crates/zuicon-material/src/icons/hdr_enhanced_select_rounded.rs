// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(HdrEnhancedSelectRounded)]
pub fn hdr_enhanced_select_rounded(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("HdrEnhancedSelectRounded"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M12,2C8.69,2,6,4.69,6,8s2.69,6,6,6s6-2.69,6-6S15.31,2,12,2z M14,9h-1v1c0,0.55-0.45,1-1,1h0c-0.55,0-1-0.45-1-1V9h-1 C9.45,9,9,8.55,9,8v0c0-0.55,0.45-1,1-1h1V6c0-0.55,0.45-1,1-1h0c0.55,0,1,0.45,1,1v1h1c0.55,0,1,0.45,1,1v0C15,8.55,14.55,9,14,9 z"/><path d="M10,16H7c-0.28,0-0.5,0.22-0.5,0.5v5C6.5,21.78,6.72,22,7,22h3c0.82,0,1.5-0.67,1.5-1.5v-3C11.5,16.67,10.82,16,10,16z M10,20.5H8v-3h2V20.5z"/><path d="M16.5,16H14c-0.55,0-1,0.45-1,1v4.31c0,0.38,0.31,0.69,0.69,0.69h0.11c0.38,0,0.69-0.31,0.69-0.69V20h1.1l0.72,1.59 c0.11,0.25,0.36,0.41,0.63,0.41h0c0.5,0,0.83-0.51,0.64-0.97L17.1,19.9c0.5-0.3,0.9-0.8,0.9-1.4v-1C18,16.67,17.33,16,16.5,16z M16.5,18.5h-2v-1h2V18.5z"/><path d="M3.5,18h-2v-1.25C1.5,16.34,1.16,16,0.75,16h0C0.34,16,0,16.34,0,16.75v4.5C0,21.66,0.34,22,0.75,22h0 c0.41,0,0.75-0.34,0.75-0.75V19.5h2v1.75C3.5,21.66,3.84,22,4.25,22h0C4.66,22,5,21.66,5,21.25v-4.5C5,16.34,4.66,16,4.25,16h0 c-0.41,0-0.75,0.34-0.75,0.75V18z"/><path d="M22,18.5v-1.25c0-0.41-0.34-0.75-0.75-0.75l0,0c-0.41,0-0.75,0.34-0.75,0.75v1.25h-1.25c-0.41,0-0.75,0.34-0.75,0.75l0,0 c0,0.41,0.34,0.75,0.75,0.75h1.25v1.25c0,0.41,0.34,0.75,0.75,0.75l0,0c0.41,0,0.75-0.34,0.75-0.75V20h1.25 c0.41,0,0.75-0.34,0.75-0.75l0,0c0-0.41-0.34-0.75-0.75-0.75H22z"/>
        </SvgIcon>
    }
}
