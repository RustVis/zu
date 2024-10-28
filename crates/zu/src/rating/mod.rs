// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

mod rating_item;
mod size;

use yew::{classes, function_component, html, AttrValue, Callback, Classes, Html, Properties};
use zu_util::prop::ToAttr;

use crate::styles::size::Size;

#[derive(Debug, Clone, PartialEq, Properties)]
pub struct Props {
    /// Override or extend the styles applied to the component.
    #[prop_or_default]
    pub classes: Classes,

    /// The default value.
    ///
    /// Use when the component is not controlled.
    #[prop_or_default]
    pub default_value: Option<f64>,

    /// If `true`, the component is disabled.
    #[prop_or(false)]
    pub disabled: bool,

    /// The icon to display when empty.
    ///
    /// Default is `<StarBorder font_size={FontSize::Inherit} />`
    #[prop_or_default]
    pub empty_icon: Option<Html>,

    /// The label read when the rating input is empty.
    #[prop_or_default]
    pub empty_label_text: Option<Html>,

    /// If `true`, only the selected icon will be highlighted.
    #[prop_or(false)]
    pub highlight_selected_only: bool,

    /// The icon to display.
    ///
    /// Default is `<Star font_size={FontSize::Inherit} />`
    #[prop_or_default]
    pub icon: Option<Html>,

    /// Maximum rating.
    #[prop_or(5.0)]
    pub max: f64,

    /// The name attribute of the radio `input` elements.
    ///
    /// This input `name` should be unique within the page.
    /// Being unique within a form is insufficient since the `name` is used to generated IDs.
    #[prop_or_default]
    pub name: AttrValue,

    /// Callback fired when the value changes.
    #[prop_or_default]
    pub on_change: Option<Callback<f64>>,

    /// Callback function that is fired when the hover state changes
    #[prop_or_default]
    pub on_change_active: Option<Callback<i32>>,

    #[prop_or_default]
    pub on_mouse_leave: Option<Callback<()>>,

    #[prop_or_default]
    pub on_mouse_move: Option<Callback<()>>,

    /// The minimum increment value change allowed.
    #[prop_or(1.0)]
    pub precision: f64,

    /// Removes all hover effects and pointer events.
    #[prop_or(false)]
    pub read_only: bool,

    /// The size of the component.
    #[prop_or(Size::Medium)]
    pub size: Size,

    #[prop_or_default]
    pub style: AttrValue,

    /// The rating value.
    #[prop_or_default]
    pub value: Option<f64>,
}

#[function_component(Rating)]
pub fn rating(props: &Props) -> Html {
    let focus_visible = false;

    let root_cls = classes!(
        "ZuRating-root",
        size::css_class(props.size),
        if props.disabled {
            "ZuRating-disabled"
        } else {
            ""
        },
        if focus_visible {
            "ZuRating-focusVisible"
        } else {
            ""
        },
        if props.read_only {
            "ZuRating-readOnly"
        } else {
            ""
        }
    );

    // let _label_cls = classes!("ZuRating-label", "ZuRating-pristine");
    // let _label_empty_value_cls = "ZuRating-labelEmptyValueActive";
    // let _icon_cls = "ZuRating-icon";
    // let _icon_empty_cls = "ZuRating-iconEmpty";
    // let _icon_filled_cls = "ZuRating-iconFilled";
    // let _icon_hover_cls = "ZuRating-iconHover";
    // let _icon_focus_cls = "ZuRating-iconFocus";
    // let _icon_active_cls = "ZuRating-iconActive";
    // let _decimal_cls = "ZuRating-decimal";
    // let _visually_hidden_cls = "ZuRating-visuallyHidden";
    let role = if props.read_only { Some("img") } else { None };

    html! {
        <span
            class={root_cls}
            role={role}
            style={props.style.to_attr()}>
        </span>
    }
}
