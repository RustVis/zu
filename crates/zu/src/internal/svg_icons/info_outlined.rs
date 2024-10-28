// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

use yew::{function_component, html, AttrValue, Html};

use crate::svg_icon::{Props, SvgIcon};

#[function_component(InfoOutlined)]
pub fn info_outlined(props: &Props) -> Html {
    let new_props = Props {
        icon: AttrValue::from("InfoOutlined"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M11,9H13V7H11M12,20C7.59,20 4,16.41 4,12C4,7.59 7.59,4 12,4C16.41,4 20,7.59 20, 12C20,16.41 16.41,20 12,20M12,2A10,10 0 0,0 2,12A10,10 0 0,0 12,22A10,10 0 0,0 22,12A10, 10 0 0,0 12,2M11,17H13V11H11V17Z" />
        </SvgIcon>
    }
}
