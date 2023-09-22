// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License that can be
// found in the LICENSE file.

use yew::{function_component, html, AttrValue, Html};

use crate::svg_icon::{Props, SvgIcon};

#[function_component({NODE_NAME})]
pub fn {MODULE_NAME}(props: &Props) -> Html {
    let new_props = Props {
        icon: AttrValue::from("{ICON_NAME}"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            {PATH_DATA}
        </SvgIcon>
    }
}
