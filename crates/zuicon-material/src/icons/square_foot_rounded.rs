// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License that can be
// found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(SquareFootRounded)]
pub fn square_foot_rounded(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("SquareFootRounded"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M17.66,17.66l-0.71,0.71c-0.2,0.2-0.51,0.2-0.71,0l0,0c-0.2-0.2-0.2-0.51,0-0.71l0.71-0.71l-1.94-1.94l-0.71,0.71 c-0.2,0.2-0.51,0.2-0.71,0l0,0c-0.2-0.2-0.2-0.51,0-0.71l0.71-0.71l-1.94-1.94l-0.71,0.71c-0.2,0.2-0.51,0.2-0.71,0l0,0 c-0.2-0.2-0.2-0.51,0-0.71l0.71-0.71L9.7,9.7l-0.71,0.71c-0.2,0.2-0.51,0.2-0.71,0l0,0c-0.2-0.2-0.2-0.51,0-0.71l0.71-0.71 L7.05,7.05L6.34,7.76c-0.2,0.2-0.51,0.2-0.71,0l0,0c-0.2-0.2-0.2-0.51,0-0.71l0.71-0.71L4.85,4.85C4.54,4.54,4,4.76,4,5.21V18 c0,1.1,0.9,2,2,2h12.79c0.45,0,0.67-0.54,0.35-0.85L17.66,17.66z M7,16v-4.76L12.76,17H8C7.45,17,7,16.55,7,16z"/>
        </SvgIcon>
    }
}
