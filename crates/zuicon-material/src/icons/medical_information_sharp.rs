// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(MedicalInformationSharp)]
pub fn medical_information_sharp(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("MedicalInformationSharp"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M22,7h-7V2H9v5H2v15h20V7z M11,4h2v5h-2V4z M11,16H9v2H7v-2H5v-2h2v-2h2v2h2V16z M13,14.5V13h6v1.5H13z M13,17.5V16h4v1.5 H13z"/>
        </SvgIcon>
    }
}
