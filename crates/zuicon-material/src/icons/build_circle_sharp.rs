// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(BuildCircleSharp)]
pub fn build_circle_sharp(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("BuildCircleSharp"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M12,2C6.48,2,2,6.48,2,12c0,5.52,4.48,10,10,10s10-4.48,10-10 C22,6.48,17.52,2,12,2z M15.14,17.25l-3.76-3.76c-1.22,0.43-2.64,0.17-3.62-0.81c-1.11-1.11-1.3-2.79-0.59-4.1l2.35,2.35 l1.41-1.41L8.58,7.17c1.32-0.71,2.99-0.52,4.1,0.59c0.98,0.98,1.24,2.4,0.81,3.62l3.76,3.76L15.14,17.25z" fill-rule="evenodd"/>
        </SvgIcon>
    }
}
