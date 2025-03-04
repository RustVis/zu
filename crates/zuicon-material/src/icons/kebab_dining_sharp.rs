// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(KebabDiningSharp)]
pub fn kebab_dining_sharp(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("KebabDiningSharp"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M7.75,8H11v5H7.75v1H8.5c1.38,0,2.5,1.12,2.5,2.5S9.88,19,8.5,19H7.75v4h-1.5v-4H5.5C4.12,19,3,17.88,3,16.5S4.12,14,5.5,14 h0.75v-1H3V8h3.25V7H5.5C4.12,7,3,5.88,3,4.5S4.12,2,5.5,2h0.75V1h1.5v1H8.5C9.88,2,11,3.12,11,4.5S9.88,7,8.5,7H7.75V8z M17.75,7 h0.75C19.88,7,21,5.88,21,4.5S19.88,2,18.5,2h-0.75V1h-1.5v1H15.5C14.12,2,13,3.12,13,4.5S14.12,7,15.5,7h0.75v1H13v5h3.25v1H15.5 c-1.38,0-2.5,1.12-2.5,2.5s1.12,2.5,2.5,2.5h0.75v4h1.5v-4h0.75c1.38,0,2.5-1.12,2.5-2.5S19.88,14,18.5,14h-0.75v-1H21V8h-3.25V7z"/>
        </SvgIcon>
    }
}
