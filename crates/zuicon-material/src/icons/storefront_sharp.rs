// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License that can be
// found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(StorefrontSharp)]
pub fn storefront_sharp(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("StorefrontSharp"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M21.9,8.89L20.49,3h-3.75h-2.01H13h-2H9.28H7.26H3.51L2.1,8.89c-0.24,1.02-0.02,2.06,0.62,2.88 C2.8,11.88,2.91,11.96,3,12.06V21h18v-8.94c0.09-0.09,0.2-0.18,0.28-0.28C21.92,10.96,22.15,9.91,21.9,8.89z M7.02,5L6.44,9.86 C6.36,10.51,5.84,11,5.23,11c-0.49,0-0.8-0.29-0.93-0.47c-0.26-0.33-0.35-0.76-0.25-1.17L5.09,5H7.02z M18.91,5l1.05,4.36 c0.1,0.42,0.01,0.84-0.25,1.17C19.57,10.71,19.27,11,18.77,11c-0.61,0-1.14-0.49-1.21-1.14L16.98,5H18.91z M15.51,9.52 c0.05,0.39-0.07,0.78-0.33,1.07C14.95,10.85,14.63,11,14.22,11C13.55,11,13,10.41,13,9.69V5h1.96L15.51,9.52z M11,9.69 C11,10.41,10.45,11,9.71,11c-0.34,0-0.65-0.15-0.89-0.41C8.57,10.3,8.45,9.91,8.49,9.52L9.04,5H11V9.69z M5,19v-6.03 C5.08,12.98,5.15,13,5.23,13c0.87,0,1.66-0.36,2.24-0.95c0.6,0.6,1.4,0.95,2.31,0.95c0.87,0,1.65-0.36,2.23-0.93 c0.59,0.57,1.39,0.93,2.29,0.93c0.84,0,1.64-0.35,2.24-0.95c0.58,0.59,1.37,0.95,2.24,0.95c0.08,0,0.15-0.02,0.23-0.03V19H5z"/>
        </SvgIcon>
    }
}
