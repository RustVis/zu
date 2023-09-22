// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License that can be
// found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(HolidayVillageOutlined)]
pub fn holiday_village_outlined(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("HolidayVillageOutlined"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M8,4l-6,6v10h12V10L8,4z M12,18H9v-3H7v3H4v-7.17l4-4l4,4V18z M9,13H7v-2h2V13z M18,20V8.35L13.65,4h-2.83L16,9.18V20H18z M22,20V6.69L19.31,4h-2.83L20,7.52V20H22z"/>
        </SvgIcon>
    }
}
