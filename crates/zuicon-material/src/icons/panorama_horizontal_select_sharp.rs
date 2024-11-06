// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(PanoramaHorizontalSelectSharp)]
pub fn panorama_horizontal_select_sharp(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("PanoramaHorizontalSelectSharp"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M12,5.5c-5.25,0-9.01-1.54-10-1.92L2,20.4c2.16-0.76,5.21-1.9,10-1.9c4.78,0,7.91,1.17,10,1.9L22,3.6 C19.91,4.33,16.77,5.5,12,5.5z"/>
        </SvgIcon>
    }
}
