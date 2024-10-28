// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

use yew::{function_component, html, AttrValue, Classes, Html, Properties};

const ROLE_DEFAULT: &str = "alert";

#[derive(Debug, Clone, PartialEq, Properties)]
pub struct Props {
    #[prop_or_default]
    pub action: Option<Html>,

    #[prop_or_default]
    pub classes: Classes,

    #[prop_or_default]
    pub message: Option<Html>,

    #[prop_or_default]
    pub role: AttrValue,

    #[prop_or_default]
    pub style: AttrValue,
}

#[function_component(SnackbarContent)]
pub fn snackbar_content(_props: &Props) -> Html {
    html! {
        <>
        </>
    }
}
