// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

mod blur_option;

use yew::{function_component, html, AttrValue, Callback, Html, Properties};

use crate::styles::size::Size;
pub use blur_option::BlurOption;

#[derive(Debug, Clone, PartialEq, Properties)]
pub struct Props {
    pub options: Vec<String>,

    pub render_input: Callback<()>,

    #[prop_or(false)]
    pub auto_complete: bool,

    #[prop_or(false)]
    pub auto_highlight: bool,

    #[prop_or(false)]
    pub auto_select: bool,

    #[prop_or_default]
    pub blur_on_select: BlurOption,

    #[prop_or_default]
    pub classes: AttrValue,

    #[prop_or_default]
    pub clear_icon: Option<Html>,

    #[prop_or(false)]
    pub clear_on_blur: bool,

    #[prop_or(false)]
    pub clear_on_escape: bool,

    #[prop_or_default]
    pub clear_text: AttrValue,

    #[prop_or_default]
    pub close_text: AttrValue,

    #[prop_or_default]
    pub default_value: AttrValue,

    #[prop_or(false)]
    pub disable_clearable: bool,

    #[prop_or(false)]
    pub disable_close_on_select: bool,

    #[prop_or(false)]
    pub disabled: bool,

    #[prop_or(false)]
    pub disabled_items_focusable: bool,

    #[prop_or(false)]
    pub disable_list_wrap: bool,

    #[prop_or(false)]
    pub disable_portal: bool,

    #[prop_or(false)]
    pub filter_selected_options: bool,

    #[prop_or(false)]
    pub free_solo: bool,

    #[prop_or(false)]
    pub full_width: bool,

    pub get_limit_tags_text: Option<Callback<i32, Html>>,

    #[prop_or(false)]
    pub handle_home_end_keys: bool,

    #[prop_or_default]
    pub id: AttrValue,

    #[prop_or(false)]
    pub include_input_in_list: bool,

    #[prop_or_default]
    pub input_value: AttrValue,

    #[prop_or(-1)]
    pub limit_tags: i32,

    #[prop_or_default]
    pub listbox_component: AttrValue,

    #[prop_or(false)]
    pub loading: bool,

    #[prop_or_default]
    pub loading_text: AttrValue,

    #[prop_or(false)]
    pub multiple: bool,

    #[prop_or_default]
    pub no_options_text: Option<Html>,

    #[prop_or_default]
    pub on_change: Option<Callback<()>>,

    #[prop_or_default]
    pub on_close: Option<Callback<()>>,

    #[prop_or_default]
    pub on_highlight_change: Option<Callback<()>>,

    #[prop_or_default]
    pub on_input_change: Option<Callback<()>>,

    #[prop_or_default]
    pub on_open: Option<Callback<()>>,

    #[prop_or(false)]
    pub open: bool,

    #[prop_or(false)]
    pub open_on_focus: bool,

    #[prop_or_default]
    pub open_text: AttrValue,

    #[prop_or_default]
    pub paper_component: Option<Html>,

    #[prop_or_default]
    pub popper_component: Option<Html>,

    #[prop_or_default]
    pub popup_icon: Option<Html>,

    #[prop_or(false)]
    pub read_only: bool,

    #[prop_or(false)]
    pub select_on_focus: bool,

    #[prop_or_default]
    pub size: Size,

    #[prop_or_default]
    pub style: AttrValue,

    #[prop_or_default]
    pub value: AttrValue,
}

#[function_component(Autocomplete)]
pub fn autocomplete(_props: &Props) -> Html {
    html! {
        <>
        </>
    }
}
