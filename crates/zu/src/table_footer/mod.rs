// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

use yew::{AttrValue, Children, function_component, html, Html, Properties};

#[derive(Debug, Clone, PartialEq, Properties)]
pub struct Props {
    #[prop_or_default]
    pub children: Children,
    
    #[prop_or_default]
    pub classes: AttrValue,

    #[prop_or_default]
    pub component: AttrValue,
    
    #[prop_or_default]
    pub style: AttrValue,
}

#[function_component(TableFooter)]
pub fn table_footer(_props: &Props) -> Html {
    html! {
        <>
        </>
    }
}
