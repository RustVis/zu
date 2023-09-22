// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License that can be
// found in the LICENSE file.

mod position;
mod variant;

use yew::{function_component, html, AttrValue, Classes, Html, Properties};

pub use position::Position;
pub use variant::Variant;

#[derive(Debug, Clone, PartialEq, Properties)]
pub struct Props {
    pub steps: i32,

    #[prop_or(0)]
    pub active_step: i32,

    #[prop_or_default]
    pub back_button: Option<Html>,

    #[prop_or_default]
    pub classes: Classes,

    #[prop_or_default]
    pub next_button: Option<Html>,

    #[prop_or_default]
    pub position: Position,

    #[prop_or_default]
    pub style: AttrValue,

    #[prop_or_default]
    pub variant: Variant,
}

#[function_component(MobileStepper)]
pub fn mobile_stepper(_props: &Props) -> Html {
    html! {
        <>
        </>
    }
}
