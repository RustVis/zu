// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License that can be
// found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(Soap)]
pub fn soap(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("Soap"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M9.12,5l-7.18,6.79C1.34,12.35,1,13.14,1,13.97V20c0,1.66,1.34,3,3,3h6.25H12h5.75c0.69,0,1.25-0.56,1.25-1.25 s-0.56-1.25-1.25-1.25H12v-1h7.75c0.69,0,1.25-0.56,1.25-1.25S20.44,17,19.75,17H12v-1h8.75c0.69,0,1.25-0.56,1.25-1.25 s-0.56-1.25-1.25-1.25H12v-1h6.75c0.69,0,1.25-0.56,1.25-1.25S19.44,10,18.75,10H8.86c0.64-1.11,1.48-2.58,1.49-2.61 c0.09-0.16,0.14-0.33,0.14-0.53c0-0.26-0.09-0.5-0.26-0.7C10.22,6.12,9.12,5,9.12,5L9.12,5z M14,6.25c0.41,0,0.75,0.34,0.75,0.75 S14.41,7.75,14,7.75S13.25,7.41,13.25,7S13.59,6.25,14,6.25 M14,4.75c-1.24,0-2.25,1.01-2.25,2.25S12.76,9.25,14,9.25 S16.25,8.24,16.25,7S15.24,4.75,14,4.75L14,4.75z M19.75,5.5c0.28,0,0.5,0.22,0.5,0.5s-0.22,0.5-0.5,0.5s-0.5-0.22-0.5-0.5 S19.47,5.5,19.75,5.5 M19.75,4c-1.1,0-2,0.9-2,2s0.9,2,2,2s2-0.9,2-2S20.85,4,19.75,4L19.75,4z M16.5,1C15.67,1,15,1.67,15,2.5 S15.67,4,16.5,4C17.33,4,18,3.33,18,2.5S17.33,1,16.5,1z"/>
        </SvgIcon>
    }
}
