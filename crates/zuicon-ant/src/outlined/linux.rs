// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto Generated! DO NOT EDIT!

use yew::prelude::{html, Component, Context, Html, Properties};

pub struct Linux {}

#[derive(Properties, Debug, Clone, PartialEq, Eq)]
pub struct Props {
    #[prop_or_default]
    pub class: Option<&'static str>,

    #[prop_or_default]
    pub width: Option<&'static str>,

    #[prop_or_default]
    pub height: Option<&'static str>,

    #[prop_or_default]
    pub color: Option<&'static str>,

    #[prop_or_default]
    pub fill: Option<&'static str>,

    #[prop_or_default]
    pub spin: bool,

    #[prop_or_default]
    pub rotate: i16,
}

impl Component for Linux {
    type Properties = Props;
    type Message = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {}
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let props = ctx.props();
        // TODO(Shaohua): Do not generate style attribute if it is empty.
        let mut style = String::new();
        if props.rotate != 0 {
            style += &format!("transform: rotate({}deg);", props.rotate);
        }
        html! {
            <svg
                xmlns={ "http://www.w3.org/2000/svg" }
                class={ props.class.unwrap_or("") }
                width={ props.width.unwrap_or("16") }
                height={ props.height.unwrap_or("16") }
                focusable={ "false" }
                data-icon={ "linux" }
                viewBox={ "0 0 16 16" }
                fill={ props.fill.unwrap_or("currentColor") }
                style={ style }
            >
            <path d="M387.86 0c-5.786 0-11.759.299-17.88.784-157.798 12.431-115.95 179.448-118.34 235.108-2.874 40.803-11.198 72.945-39.234 112.776-33.037 39.235-79.402 102.66-101.39 168.772-10.378 31.06-15.305 62.865-10.714 92.916a15.828 15.828 0 0 0-4.143 5.04c-9.706 10.004-16.762 22.436-24.713 31.32-7.429 7.43-18.106 9.968-29.753 14.933C30.01 666.726 17.13 671.69 9.44 687.07c-3.36 7.018-5.077 14.67-4.928 22.435 0 7.43 1.008 14.97 2.053 20.01 2.165 14.895 4.33 27.214 1.456 36.21-9.258 25.385-10.415 42.781-3.92 55.436 6.496 12.469 19.972 17.509 35.054 22.436 30.275 7.466 71.301 5.04 103.592 22.361 34.569 17.434 69.66 25.05 97.657 17.546a66.01 66.01 0 0 0 45.096-35.278c21.913-.112 45.917-10.042 84.367-12.468 26.094-2.165 58.759 9.967 96.239 7.429.933 5.04 2.351 7.428 4.255 12.468l.112.112c14.597 29.043 41.55 42.258 70.331 39.981 28.782-2.24 59.43-20.01 84.255-48.754 23.556-28.558 62.828-40.466 88.773-56.108 12.99-7.429 23.48-17.508 24.227-31.843.859-14.932-7.428-30.312-26.654-51.404v-3.621l-.112-.112c-6.346-7.466-9.332-19.972-12.617-34.568-3.174-14.97-6.795-29.342-18.367-39.048h-.112c-2.203-2.016-4.592-2.501-7.018-5.04a13.327 13.327 0 0 0-7.093-2.389c16.09-47.709 9.855-95.193-6.458-137.9-19.898-52.636-54.69-98.478-81.195-130.022-29.715-37.517-58.833-73.056-58.273-125.767C521.168 148.837 529.008.224 387.86 0m19.748 127.11h.486c7.951 0 14.783 2.315 21.8 7.392 7.131 5.04 12.32 12.394 16.389 19.898 3.92 9.668 5.898 17.134 6.197 27.027 0-.747.224-1.493.224-2.203v3.883a3.21 3.21 0 0 1-.15-.784l-.149-.896a67.456 67.456 0 0 1-5.6 26.355 35.576 35.576 0 0 1-7.95 12.506 26.505 26.505 0 0 0-3.286-1.568c-3.92-1.68-7.429-2.389-10.64-4.965a48.978 48.978 0 0 0-8.175-2.463c1.83-2.203 5.413-4.965 6.795-7.392 1.978-4.778 3.06-9.855 3.285-15.007v-.71a45.17 45.17 0 0 0-2.277-14.931c-1.68-5.04-3.77-7.504-6.832-12.469-3.136-2.464-6.234-4.928-9.967-4.928h-.598c-3.471 0-6.57 1.12-9.78 4.928a29.865 29.865 0 0 0-7.653 12.469 44.05 44.05 0 0 0-3.36 14.932v.71c.075 3.322.299 6.681.747 9.966-7.205-2.5-16.351-5.04-22.66-7.54-.375-2.46-.6-4.942-.672-7.43v-.746a66.15 66.15 0 0 1 5.6-28.707 40.455 40.455 0 0 1 16.052-19.897 36.77 36.77 0 0 1 22.174-7.43m-110.573 2.203h1.344c5.3 0 10.08 1.792 14.895 5.04 5.45 4.816 9.855 10.751 12.842 17.359 3.36 7.429 5.263 14.97 5.711 24.9v.149c.261 5.002.224 7.503-.074 9.93v2.986c-1.12.261-2.091.672-3.099.896-5.674 2.053-10.229 5.04-14.67 7.466.447-3.322.484-6.682.111-9.967v-.56c-.448-4.965-1.456-7.429-3.061-12.431a22.884 22.884 0 0 0-6.197-9.968 9.258 9.258 0 0 0-6.831-2.389h-.784c-2.65.224-4.853 1.53-6.944 4.928a20.607 20.607 0 0 0-4.48 10.08 35.24 35.24 0 0 0-.858 12.356v.522c.448 5.04 1.381 7.504 3.024 12.469 1.68 5.002 3.62 7.466 6.16 10.004.41.336.783.672 1.268.896-2.613 2.128-4.367 2.613-6.57 5.077a11.386 11.386 0 0 1-4.89 2.539 97.844 97.844 0 0 1-10.266-15.007 66.15 66.15 0 0 1-5.786-24.9 65.665 65.665 0 0 1 2.986-24.937 53.383 53.383 0 0 1 10.565-19.971c4.778-4.965 9.706-7.467 15.604-7.467M348.215 193c12.357 0 27.326 2.427 45.357 14.895 10.938 7.467 19.524 10.042 39.31 17.471h.111c9.52 5.077 15.12 9.93 17.844 14.895v-4.89a21.316 21.316 0 0 1 .598 17.545c-4.592 11.61-19.263 24.041-39.72 31.47v.075c-10.005 5.04-18.703 12.43-28.931 17.358-10.304 5.04-21.95 10.9-37.78 9.968a42.52 42.52 0 0 1-16.723-2.502 133.121 133.121 0 0 1-12.02-7.391c-7.28-5.04-13.552-12.394-22.847-17.359v-.186h-.187c-14.932-9.184-22.995-19.114-25.609-26.542-2.575-10.005-.186-17.509 7.205-22.399 8.362-5.04 14.186-10.116 18.031-12.543 3.882-2.762 5.338-3.808 6.57-4.89h.075v-.112c6.309-7.541 16.276-17.508 31.32-22.436 5.19-1.344 10.975-2.427 17.396-2.427m104.489 80c13.402 52.898 44.685 129.724 64.806 166.98 10.676 19.935 31.918 61.932 41.138 112.888 5.824-.187 12.282.672 19.15 2.39 24.116-62.38-20.382-129.426-40.652-148.054-8.25-7.504-8.66-12.506-4.592-12.506 21.988 19.935 50.956 58.684 61.446 102.92 4.816 19.973 5.936 41.214.784 62.343 2.501 1.045 5.04 2.277 7.653 2.501 38.525 19.935 52.748 35.016 45.917 57.377v-1.605c-2.277-.112-4.48 0-6.757 0h-.56c5.637-17.433-6.794-30.798-39.757-45.693-34.158-14.932-61.446-12.543-66.113 17.359-.261 1.605-.448 2.464-.634 5.04-2.539.858-5.19 1.978-7.802 2.389-16.053 10.004-24.713 24.974-29.604 44.311-4.853 19.898-6.346 43.155-7.652 69.771v.112c-.784 12.469-6.384 31.283-11.909 50.434-55.996 40.018-133.644 57.415-199.682 12.468a98.74 98.74 0 0 0-15.007-19.897 54.13 54.13 0 0 0-10.265-12.468c6.794 0 12.617-1.083 17.358-2.501a22.958 22.958 0 0 0 11.722-12.469c4.032-9.967 0-26.02-12.879-43.415-12.879-17.434-34.755-37.144-66.747-56.78-23.518-14.895-36.808-32.478-42.93-52.114-6.16-19.934-5.339-40.504-.56-61.409 9.146-39.944 32.59-78.767 47.559-103.144 3.994-2.427 1.381 5.04-15.231 36.36-14.783 28.035-42.594 93.214-4.554 143.872a303.274 303.274 0 0 1 24.153-107.363c21.054-47.709 65.067-130.807 68.539-196.658 1.791 1.344 8.1 5.04 10.788 7.54 8.138 4.966 14.186 12.432 22.025 17.36 7.877 7.503 17.807 12.505 32.702 12.505 1.456.112 2.8.224 4.144.224 15.343 0 27.214-5.002 37.181-10.004 10.826-5.002 19.45-12.469 27.625-14.932h.186c17.434-5.04 31.209-15.007 39.01-26.132m81.605 334.408c1.38 22.436 12.804 46.477 32.925 51.404 21.95 5.003 53.532-12.43 66.86-28.558l7.876-.336c11.76-.298 21.54.374 31.62 9.968l.111.112c7.765 7.429 11.386 19.822 14.597 32.701 3.173 14.97 5.749 29.118 15.268 39.795 18.143 19.673 24.078 33.821 23.742 42.557l.112-.224v.672l-.112-.448c-.56 9.78-6.906 14.783-18.59 22.212-23.519 14.97-65.18 26.579-91.722 58.609-23.07 27.512-51.18 42.52-76.005 44.46-24.788 1.98-46.178-7.466-58.759-33.522l-.186-.112c-7.84-14.97-4.48-38.264 2.09-63.09 6.57-24.936 15.978-50.209 17.284-70.853 1.382-26.654 2.837-49.836 7.28-67.718 4.48-17.358 11.498-29.752 23.929-36.733l1.68-.821zm-403.731 1.83h.373c1.978 0 3.92.186 5.86.522 14.037 2.053 26.356 12.431 38.19 28.073l33.971 62.118.112.112c9.071 19.897 28.147 39.72 44.386 61.147 16.202 22.324 28.745 42.221 27.214 58.61v.224c-2.128 27.774-17.881 42.855-41.997 48.305-24.078 5.04-56.742.075-89.407-17.321-36.136-20.01-79.066-17.508-106.653-22.473-13.775-2.464-22.81-7.504-26.99-14.97-4.144-7.428-4.219-22.435 4.591-45.916v-.112l.075-.112c4.368-12.469 1.12-28.11-1.008-41.773-2.053-14.97-3.099-26.468 1.605-35.091 5.973-12.469 14.783-14.895 25.721-19.897 11.013-5.04 23.929-7.541 34.195-17.509h.075v-.111c9.556-10.005 16.612-22.436 24.936-31.284 7.093-7.503 14.186-12.543 24.75-12.543m267.25-338.74c-16.24 7.504-35.278 19.973-55.548 19.973-20.233 0-36.211-9.967-47.746-17.396-5.786-5.003-10.453-10.005-13.962-12.506-6.122-5.002-5.375-12.468-2.762-12.468 4.069.597 4.815 5.04 7.429 7.503 3.583 2.464 8.026 7.429 13.476 12.431 10.863 7.466 25.385 17.434 43.527 17.434 18.106 0 39.31-9.968 52.189-17.397 7.28-5.04 16.612-12.468 24.19-17.433 5.824-5.114 5.562-10.005 10.415-10.005 4.816.598 1.27 5.003-5.487 12.432a302.304 302.304 0 0 1-25.759 17.47v-.037zm-40.392-59.13v-.822c-.224-.71.485-1.568 1.083-1.867 2.762-1.605 6.72-1.008 9.706.15 2.351 0 5.972 2.5 5.6 5.04-.225 1.829-3.174 2.463-5.04 2.463-2.054 0-3.435-1.605-5.264-2.538-1.941-.672-5.45-.299-6.085-2.427m-20.57 0c-.746 2.164-4.218 1.828-6.196 2.463-1.755.933-3.21 2.538-5.189 2.538-1.904 0-4.89-.709-5.114-2.538-.336-2.464 3.285-4.965 5.6-4.965 3.023-1.157 6.868-1.755 9.668-.187.71.336 1.344 1.12 1.12 1.867v.784h.112z" transform="translate(142.938 64)"/>
            </svg>
        }
    }
}
