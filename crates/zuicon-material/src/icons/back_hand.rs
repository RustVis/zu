// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(BackHand)]
pub fn back_hand(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("BackHand"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M13,24c-3.26,0-6.19-1.99-7.4-5.02l-3.03-7.61C2.26,10.58,3,9.79,3.81,10.05l0.79,0.26c0.56,0.18,1.02,0.61,1.24,1.16 L7.25,15H8V3.25C8,2.56,8.56,2,9.25,2s1.25,0.56,1.25,1.25V12h1V1.25C11.5,0.56,12.06,0,12.75,0S14,0.56,14,1.25V12h1V2.75 c0-0.69,0.56-1.25,1.25-1.25c0.69,0,1.25,0.56,1.25,1.25V12h1V5.75c0-0.69,0.56-1.25,1.25-1.25S21,5.06,21,5.75V16 C21,20.42,17.42,24,13,24z"/>
        </SvgIcon>
    }
}
