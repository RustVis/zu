// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(PhoneEnabledSharp)]
pub fn phone_enabled_sharp(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("PhoneEnabledSharp"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="m3 15.46 5.27-.61 2.52 2.52c2.83-1.44 5.15-3.75 6.59-6.59l-2.53-2.53.61-5.25h5.51C21.55 13.18 13.18 21.55 3 20.97v-5.51z"/>
        </SvgIcon>
    }
}
