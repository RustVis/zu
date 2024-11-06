// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto generated, do not edit.

use yew::{function_component, html, Html};
use zu::svg_icon::{Props, SvgIcon};

#[function_component(ViewKanbanSharp)]
pub fn view_kanban_sharp(props: &Props) -> Html {
    let new_props = Props{
        icon: From::from("ViewKanbanSharp"),
        ..props.clone()
    };

    html! {
        <SvgIcon ..new_props>
            <path d="M21,3H3v18h18V3z M9,17H7V7h2V17z M13,12h-2V7h2V12z M17,15h-2V7h2V15z"/>
        </SvgIcon>
    }
}
