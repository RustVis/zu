// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(KebabDiningRounded)]
pub fn kebab_dining_rounded(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("KebabDiningRounded"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M7.75,13v1H8.5c1.38,0,2.5,1.12,2.5,2.5c0,1.38-1.12,2.5-2.5,2.5H7.75v3.25C7.75,22.66,7.41,23,7,23s-0.75-0.34-0.75-0.75 V19H5.5C4.12,19,3,17.88,3,16.5C3,15.12,4.12,14,5.5,14h0.75v-1H4c-0.55,0-1-0.45-1-1V9c0-0.55,0.45-1,1-1h2.25V7H5.5 C4.12,7,3,5.88,3,4.5C3,3.12,4.12,2,5.5,2h0.75V1.75C6.25,1.34,6.59,1,7,1s0.75,0.34,0.75,0.75V2H8.5C9.88,2,11,3.12,11,4.5 C11,5.88,9.88,7,8.5,7H7.75v1H10c0.55,0,1,0.45,1,1v3c0,0.55-0.45,1-1,1H7.75z M17.75,13v1h0.75c1.38,0,2.5,1.12,2.5,2.5 c0,1.38-1.12,2.5-2.5,2.5h-0.75v3.25c0,0.41-0.34,0.75-0.75,0.75s-0.75-0.34-0.75-0.75V19H15.5c-1.38,0-2.5-1.12-2.5-2.5 c0-1.38,1.12-2.5,2.5-2.5h0.75v-1H14c-0.55,0-1-0.45-1-1V9c0-0.55,0.45-1,1-1h2.25V7H15.5C14.12,7,13,5.88,13,4.5 C13,3.12,14.12,2,15.5,2h0.75V1.75C16.25,1.34,16.59,1,17,1s0.75,0.34,0.75,0.75V2h0.75C19.88,2,21,3.12,21,4.5 C21,5.88,19.88,7,18.5,7h-0.75v1H20c0.55,0,1,0.45,1,1v3c0,0.55-0.45,1-1,1H17.75z"/>
        </SvgIcon>
    }
}
