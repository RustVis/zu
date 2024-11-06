// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(TranscribeOutlined)]
pub fn transcribe_outlined(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("TranscribeOutlined"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M9,13c2.21,0,4-1.79,4-4c0-2.21-1.79-4-4-4S5,6.79,5,9C5,11.21,6.79,13,9,13z M9,7c1.1,0,2,0.9,2,2c0,1.1-0.9,2-2,2 s-2-0.9-2-2C7,7.9,7.9,7,9,7z M15.39,15.56C13.71,14.7,11.53,14,9,14c-2.53,0-4.71,0.7-6.39,1.56C1.61,16.07,1,17.1,1,18.22V21h16 v-2.78C17,17.1,16.39,16.07,15.39,15.56z M15,19H3v-0.78c0-0.38,0.2-0.72,0.52-0.88C4.71,16.73,6.63,16,9,16 c2.37,0,4.29,0.73,5.48,1.34C14.8,17.5,15,17.84,15,18.22V19z M17.93,16l1.63-1.63c-2.77-3.02-2.77-7.56,0-10.74L17.93,2 C14.03,5.89,14.02,11.95,17.93,16z M22.92,10.95c-0.84-1.18-0.84-2.71,0-3.89l-1.68-1.69c-2.02,2.02-2.02,5.07,0,7.27L22.92,10.95z"/>
        </SvgIcon>
    }
}
