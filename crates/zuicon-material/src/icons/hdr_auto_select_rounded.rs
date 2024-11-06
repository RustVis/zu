// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(HdrAutoSelectRounded)]
pub fn hdr_auto_select_rounded(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("HdrAutoSelectRounded"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M0,0h24v24H0V0z" fill="none"/><path d="M10,16H7.25c-0.41,0-0.75,0.34-0.75,0.75v4.5C6.5,21.66,6.84,22,7.25,22H10c0.83,0,1.5-0.67,1.5-1.5v-3 C11.5,16.67,10.83,16,10,16z M10,20.5H8v-3h2V20.5z"/><path d="M4.25,16c-0.41,0-0.75,0.34-0.75,0.75V18h-2v-1.25C1.5,16.34,1.16,16,0.75,16S0,16.34,0,16.75v4.5 C0,21.66,0.34,22,0.75,22s0.75-0.34,0.75-0.75V19.5h2v1.75C3.5,21.66,3.84,22,4.25,22S5,21.66,5,21.25v-4.5 C5,16.34,4.66,16,4.25,16z"/><path d="M23.25,18.5H22v-1.25c0-0.41-0.34-0.75-0.75-0.75s-0.75,0.34-0.75,0.75v1.25h-1.25c-0.41,0-0.75,0.34-0.75,0.75 S18.84,20,19.25,20h1.25v1.25c0,0.41,0.34,0.75,0.75,0.75S22,21.66,22,21.25V20h1.25c0.41,0,0.75-0.34,0.75-0.75 S23.66,18.5,23.25,18.5z"/><path d="M16.5,16h-2.75C13.34,16,13,16.34,13,16.75v4.56c0,0.38,0.31,0.69,0.69,0.69h0.11c0.38,0,0.69-0.31,0.69-0.69V20h1.1 l0.72,1.59c0.11,0.25,0.36,0.41,0.63,0.41c0.5,0,0.83-0.51,0.64-0.97L17.1,19.9c0.5-0.3,0.9-0.8,0.9-1.4v-1 C18,16.67,17.33,16,16.5,16z M16.5,18.5h-2v-1h2V18.5z"/><path d="M12,2C8.69,2,6,4.69,6,8s2.69,6,6,6s6-2.69,6-6S15.31,2,12,2z M14.44,11c-0.24,0-0.45-0.15-0.53-0.38l-0.49-1.41h-2.83 l-0.5,1.41C10.01,10.85,9.8,11,9.56,11c-0.39,0-0.67-0.39-0.53-0.76l2.12-5.65C11.29,4.23,11.62,4,12,4s0.71,0.23,0.85,0.59 l2.12,5.65C15.11,10.61,14.84,11,14.44,11z"/>
        </SvgIcon>
    }
}
