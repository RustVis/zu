// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(CellWifiRounded)]
pub fn cell_wifi_rounded(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("CellWifiRounded"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M20.29,7.68L7.7,20.29C7.07,20.92,7.52,22,8.41,22H21c0.55,0,1-0.45,1-1V8.39C22,7.5,20.92,7.05,20.29,7.68z M20,20h-2 v-7.22l2-2V20z"/><path d="M9.61,10.68c-0.28,0.17-0.32,0.56-0.09,0.79l0.82,0.82c0.39,0.39,1.02,0.39,1.41,0l0.82-0.82 c0.23-0.23,0.18-0.62-0.09-0.79C11.61,10.14,10.49,10.14,9.61,10.68z"/><path d="M8.42,9.3c1.57-1.12,3.7-1.12,5.27,0c0.36,0.26,0.85,0.22,1.16-0.1c0.39-0.39,0.35-1.06-0.1-1.38 c-2.2-1.57-5.19-1.57-7.4,0C6.9,8.14,6.85,8.81,7.25,9.2C7.57,9.52,8.06,9.56,8.42,9.3z"/><path d="M16.26,6.69c0.34,0.28,0.83,0.28,1.14-0.03l0.12-0.12c0.35-0.35,0.31-0.92-0.08-1.24c-3.67-3.05-9.02-3.07-12.7-0.06 C4.31,5.59,4.27,6.23,4.66,6.61C4.98,6.94,5.5,6.98,5.85,6.69C8.86,4.21,13.25,4.21,16.26,6.69z"/>
        </SvgIcon>
    }
}
