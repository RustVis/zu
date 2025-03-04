// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

use yew::{function_component, html, AttrValue, Html};

use crate::svg_icon::{Props, SvgIcon};

#[function_component(IntermediateCheckBox)]
pub fn intermediate_check_box(props: &Props) -> Html {
    let new_props = Props {
        icon: AttrValue::from("IntermediateCheckBox"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M19 3H5c-1.1 0-2 .9-2 2v14c0 1.1.9 2 2 2h14c1.1 0 2-.9 2-2V5c0-1.1-.9-2-2-2zm-2 10H7v-2h10v2z" />
        </SvgIcon>
    }
}
