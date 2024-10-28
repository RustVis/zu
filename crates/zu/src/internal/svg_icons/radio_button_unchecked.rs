// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

use yew::{function_component, html, AttrValue, Html};

use crate::svg_icon::{Props, SvgIcon};

#[function_component(RadioButtonUnchecked)]
pub fn radio_button_unchecked(props: &Props) -> Html {
    let new_props = Props {
        icon: AttrValue::from("RadioButtonUnchecked"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M12 2C6.48 2 2 6.48 2 12s4.48 10 10 10 10-4.48 10-10S17.52 2 12 2zm0 18c-4.42 0-8-3.58-8-8s3.58-8 8-8 8 3.58 8 8-3.58 8-8 8z" />
        </SvgIcon>
    }
}
