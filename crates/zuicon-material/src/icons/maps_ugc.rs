// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(MapsUgc)]
pub fn maps_ugc(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("MapsUgc"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M12,2C6.48,2,2,6.48,2,12c0,1.54,0.36,2.98,0.97,4.29L1,23l6.71-1.97 C9.02,21.64,10.46,22,12,22c5.52,0,10-4.48,10-10C22,6.48,17.52,2,12,2z M16,13h-3v3h-2v-3H8v-2h3V8h2v3h3V13z" fill-rule="evenodd"/>
        </SvgIcon>
    }
}
