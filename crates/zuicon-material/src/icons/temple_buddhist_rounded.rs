// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(TempleBuddhistRounded)]
pub fn temple_buddhist_rounded(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("TempleBuddhistRounded"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M21.85,9.01c-0.41,0-0.82,0.24-0.95,0.63C20.64,10.43,19.89,11,19.02,11H4.98c-0.87,0-1.62-0.57-1.88-1.36 C2.97,9.25,2.57,9.02,2.16,9.02h0C1.5,9.02,1,9.66,1.21,10.28c0.43,1.27,1.48,2.24,2.79,2.58V20c0,1.1,0.9,2,2,2h4l0-2.89 c0-1,0.68-1.92,1.66-2.08C12.92,16.82,14,17.79,14,19v3h4c1.1,0,2-0.9,2-2v-7.14c0.46-0.12,2.22-0.76,2.81-2.58 C23.01,9.65,22.51,9,21.85,9.01L21.85,9.01z"/><path d="M6,8.86V10h12V8.86c0.46-0.12,2.22-0.76,2.81-2.58c0.2-0.63-0.3-1.27-0.96-1.27l0,0c-0.41,0-0.82,0.24-0.95,0.63 C18.64,6.43,17.89,7,17.02,7H6.98C6.11,7,5.36,6.43,5.1,5.64C4.97,5.25,4.57,5.02,4.16,5.02h0C3.5,5.02,3,5.66,3.21,6.28 C3.64,7.55,4.69,8.53,6,8.86z"/><path d="M11.2,2.07L8.25,6h7.5L12.8,2.07C12.4,1.53,11.6,1.53,11.2,2.07z"/>
        </SvgIcon>
    }
}
