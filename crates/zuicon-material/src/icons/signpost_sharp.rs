// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(SignpostSharp)]
pub fn signpost_sharp(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("SignpostSharp"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M12,2c-0.55,0-1,0.45-1,1v1H5.5C4.67,4,4,4.67,4,5.5v3C4,9.33,4.67,10,5.5,10H11v2H6.62c-0.4,0-0.78,0.16-1.06,0.44 l-1.5,1.5c-0.59,0.59-0.59,1.54,0,2.12l1.5,1.5C5.84,17.84,6.22,18,6.62,18H11v3c0,0.55,0.45,1,1,1c0.55,0,1-0.45,1-1v-3h5.5 c0.83,0,1.5-0.67,1.5-1.5v-3c0-0.83-0.67-1.5-1.5-1.5H13v-2h4.38c0.4,0,0.78-0.16,1.06-0.44l1.5-1.5c0.59-0.59,0.59-1.54,0-2.12 l-1.5-1.5C18.16,4.16,17.78,4,17.38,4H13V3C13,2.45,12.55,2,12,2z"/>
        </SvgIcon>
    }
}
