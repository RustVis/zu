// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License that can be
// found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(GasMeterSharp)]
pub fn gas_meter_sharp(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("GasMeterSharp"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M20,4h-5V2h-2v2h-2V2H9v2H4v18h16V4z M12,18c-1.38,0-2.5-1.1-2.5-2.46c0-1.09,0.43-1.39,2.5-3.79 c2.05,2.38,2.5,2.7,2.5,3.79C14.5,16.9,13.38,18,12,18z M16,10H8V8h8V10z"/>
        </SvgIcon>
    }
}
