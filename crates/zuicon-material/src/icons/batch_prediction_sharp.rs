// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(BatchPredictionSharp)]
pub fn batch_prediction_sharp(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("BatchPredictionSharp"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M19,8H5v14h14V8z M13,20.5h-2V19h2V20.5z M13,18h-2c0-1.5-2.5-3-2.5-5c0-1.93,1.57-3.5,3.5-3.5c1.93,0,3.5,1.57,3.5,3.5 C15.5,15,13,16.5,13,18z M18,6.5H6V5h12V6.5z M17,3.5H7V2h10V3.5z"/>
        </SvgIcon>
    }
}
