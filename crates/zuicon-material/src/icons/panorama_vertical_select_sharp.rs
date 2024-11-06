// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(PanoramaVerticalSelectSharp)]
pub fn panorama_vertical_select_sharp(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("PanoramaVerticalSelectSharp"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M18.49,11.99c0-5.25,1.54-9.01,1.92-10l-16.82,0c0.76,2.16,1.9,5.21,1.9,10c0,4.78-1.17,7.91-1.9,10l16.81,0 C19.66,19.91,18.49,16.76,18.49,11.99z"/>
        </SvgIcon>
    }
}
