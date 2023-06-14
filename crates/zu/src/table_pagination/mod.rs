// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

use yew::{function_component, html, AttrValue, Callback, Html, MouseEvent, Properties};

#[derive(Debug, Clone, PartialEq, Properties)]
pub struct Props {
    pub count: i32,

    pub on_page_change: Callback<(MouseEvent, i32)>,

    pub page: i32,

    /// The number of rows per page.
    ///
    /// Set -1 to display all the rows.
    pub rows_per_page: i32,
    // TODO(Shaohua): Add ActionsComponent.
    #[prop_or_default]
    pub classes: AttrValue,

    #[prop_or_default]
    pub component: AttrValue,

    // TODO(Shaohua): Add get_item_aria_label
    // TODO(Shaohua): Add labelDisplayedRows
    #[prop_or_default]
    pub label_rows_per_page: Option<Html>,
    // TODO(Shaohua); Add onRowsPerPageChange
    #[prop_or(false)]
    pub show_first_button: bool,

    #[prop_or(false)]
    pub show_last_button: bool,

    #[prop_or_default]
    pub style: AttrValue,
}

#[function_component(TablePagination)]
pub fn table_pagination(_props: &Props) -> Html {
    html! {
        <>
        </>
    }
}
