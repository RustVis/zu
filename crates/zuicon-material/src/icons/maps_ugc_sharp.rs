// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(MapsUgcSharp)]
pub fn maps_ugc_sharp(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("MapsUgcSharp"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M12,4c4.41,0,8,3.59,8,8s-3.59,8-8,8c-1.18,0-2.34-0.26-3.43-0.78c-0.27-0.13-0.56-0.19-0.86-0.19 c-0.19,0-0.38,0.03-0.56,0.08l-3.2,0.94l0.94-3.2c0.14-0.47,0.1-0.98-0.11-1.42C4.26,14.34,4,13.18,4,12C4,7.59,7.59,4,12,4 M12,2 C6.48,2,2,6.48,2,12c0,1.54,0.36,2.98,0.97,4.29L1,23l6.71-1.97C9.02,21.64,10.46,22,12,22c5.52,0,10-4.48,10-10 C22,6.48,17.52,2,12,2L12,2z"/>
        </SvgIcon>
    }
}
