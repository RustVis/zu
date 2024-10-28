// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

mod position;

use yew::{function_component, html, AttrValue, Classes, Html, Properties};

use crate::styles::position::HorizontalPosition;
pub use position::Position;

#[derive(Debug, Clone, PartialEq, Properties)]
pub struct Props {
    #[prop_or_default]
    pub action_icon: Option<Html>,

    #[prop_or(HorizontalPosition::Right)]
    pub action_position: HorizontalPosition,

    #[prop_or_default]
    pub classes: Classes,

    #[prop_or_default]
    pub position: Position,

    #[prop_or_default]
    pub subtitle: Option<Html>,

    #[prop_or_default]
    pub style: AttrValue,

    #[prop_or_default]
    pub title: Option<Html>,
}

#[function_component(ImageListItemBar)]
pub fn image_list_item_bar(_props: &Props) -> Html {
    html! {
        <>
        </>
    }
}
