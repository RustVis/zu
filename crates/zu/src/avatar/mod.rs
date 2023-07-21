// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Apache-2.0 License that can be found
// in the LICENSE file.

mod person;
mod variant;

use yew::{classes, function_component, html, AttrValue, Children, Classes, Html, Properties};
use yew_hooks::{use_async_with_options, UseAsyncOptions};
use zu_util::image_future::ImageFuture;
use zu_util::{name, prop::ToAttr};

use crate::styles::shape_variant::ShapeVariant;
pub use person::Person;

#[derive(Debug, Clone, PartialEq, Properties)]
pub struct Props {
    /// Used in combination with src or srcSet to provide an alt attribute for the rendered img element.
    #[prop_or_default]
    pub alt: AttrValue,

    #[prop_or_default]
    pub aria_label: AttrValue,

    #[prop_or_default]
    pub children: Children,

    #[prop_or_default]
    pub classes: Classes,

    #[prop_or(false)]
    pub color_default: bool,

    #[prop_or_default]
    pub component: AttrValue,

    #[prop_or_default]
    pub img_prop_cross_origin: AttrValue,

    #[prop_or_default]
    pub img_prop_referencer_policy: AttrValue,

    /// The sizes attribute for the img element.
    #[prop_or_default]
    pub sizes: AttrValue,

    /// The src attribute for the img element.
    #[prop_or_default]
    pub src: AttrValue,

    /// The srcSet attribute for the img element.
    #[prop_or_default]
    pub src_set: AttrValue,

    #[prop_or_default]
    pub style: AttrValue,

    #[prop_or_default]
    pub variant: ShapeVariant,
}

/// Fallback order:
/// - the provided children
/// - the first letter of the alt text
/// - a generic avatar icon
#[function_component(Avatar)]
pub fn avatar(props: &Props) -> Html {
    let has_image = !props.src.is_empty() || !props.src_set.is_empty();
    // Load image first.
    let use_loaded = {
        let image_src = props.src.to_string();
        let image_src_set = props.src_set.to_string();
        use_async_with_options(
            async move { ImageFuture::new(&image_src, Some(&image_src_set)).await },
            UseAsyncOptions::enable_auto(),
        )
    };
    let has_image_no_failing = has_image && use_loaded.data.is_some();

    let component = if props.component.is_empty() {
        "div"
    } else {
        props.component.as_str()
    };
    let root_cls = classes!(
        "ZuAvatar-root",
        variant::css_class(props.variant),
        if has_image_no_failing {
            ""
        } else {
            "ZuAvatar-colorDefault"
        },
        props.classes.clone(),
    );
    // TODO(Shaohua): Setup text color based on current theme.
    let style = [
        if props.alt.is_empty() {
            String::new()
        } else {
            format!("background-color: {};", name::to_color(props.alt.as_str()))
        }
        .as_str(),
        // Put custom style after auto-generated bg-color to make sure it is override.
        props.style.as_str(),
    ]
    .join(";");

    let abbr_name: String = if props.alt.is_empty() {
        String::new()
    } else {
        name::abbreviate_first(props.alt.as_str())
    };

    let children = if !props.children.is_empty() {
        html! {for props.children.iter()}
    } else if has_image_no_failing {
        html! {<img class="ZuAvatar-img"
        aria-label={props.aria_label.to_attr()}
        src={props.src.to_attr()}
        src-set={props.src_set.to_attr()}
        alt={props.alt.to_attr()}
        sizes={props.sizes.to_attr()} />}
    } else if !abbr_name.is_empty() {
        html! {abbr_name}
    } else {
        html! {<Person classes="ZuAvatar-fallback" />}
    };

    html! {
        <@{component.to_owned()} class={root_cls} style={style}>
            {children}
        </@>
    }
}
