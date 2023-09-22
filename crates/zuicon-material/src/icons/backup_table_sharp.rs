// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License that can be
// found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(BackupTableSharp)]
pub fn backup_table_sharp(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("BackupTableSharp"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M18,2H2v16h16V2z M9,16H4v-5h5V16z M16,16h-5v-5h5V16z M16,9H4V4h12V9z"/>
        </SvgIcon>
    }
}
