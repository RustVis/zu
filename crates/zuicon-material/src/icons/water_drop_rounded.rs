// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(WaterDropRounded)]
pub fn water_drop_rounded(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("WaterDropRounded"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M12.66,2.58c-0.38-0.33-0.95-0.33-1.33,0C6.45,6.88,4,10.62,4,13.8c0,4.98,3.8,8.2,8,8.2s8-3.22,8-8.2 C20,10.62,17.55,6.88,12.66,2.58z M7.83,14c0.37,0,0.67,0.26,0.74,0.62c0.41,2.22,2.28,2.98,3.64,2.87 c0.43-0.02,0.79,0.32,0.79,0.75c0,0.4-0.32,0.73-0.72,0.75c-2.13,0.13-4.62-1.09-5.19-4.12C7.01,14.42,7.37,14,7.83,14z"/>
        </SvgIcon>
    }
}
