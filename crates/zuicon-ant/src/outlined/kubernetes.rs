// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by Lesser General Public License
// that can be found in the LICENSE file.

// Auto Generated! DO NOT EDIT!

use yew::prelude::{html, Component, Context, Html, Properties};

pub struct Kubernetes {}

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

impl Component for Kubernetes {
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
                data-icon={ "kubernetes" }
                viewBox={ "0 0 16 16" }
                fill={ props.fill.unwrap_or("currentColor") }
                style={ style }
            >
            <path d="M399.987 0a61.547 61.547 0 0 0-26.801 6.135L101.882 137.131c-16.876 8.144-29.148 23.57-33.313 41.849L1.53 473.498A61.767 61.767 0 0 0 13.39 525.56l187.815 236.15c11.69 14.704 29.484 23.271 48.256 23.271h301.052c18.772 0 36.566-8.567 48.256-23.272l187.815-236.115v-.034c11.633-14.637 16.03-33.792 11.893-52.028v-.034L731.405 178.98v-.035c-4.164-18.25-16.429-33.654-33.313-41.814L426.822 6.135a17.426 17.426 0 0 0-.034-.034A61.764 61.764 0 0 0 399.987 0m0 35.097c3.942 0 7.875.876 11.55 2.639L682.84 168.732a26.538 26.538 0 0 1 14.36 18.028l67.037 294.518a26.564 26.564 0 0 1-5.106 22.45L571.317 739.875c-5.05 6.353-12.692 10.008-20.804 10.008H249.461c-8.112 0-15.72-3.655-20.77-10.008L40.876 503.727c-5.035-6.339-6.929-14.575-5.14-22.45l67.071-294.483c1.801-7.903 7.057-14.52 14.326-18.028v-.034L388.437 37.736a26.521 26.521 0 0 1 11.55-2.64m-.137 73.826c-2.476 0-4.99.5-7.403 1.508-9.651 4.212-14.22 15.437-10.008 25.09 4.036 9.475 5.425 18.936 6.478 28.412.35 4.914.552 9.657.377 14.395.526 4.739-1.94 9.486-5.45 14.224-3.685 4.738-4.031 9.486-4.558 14.224-48.155 4.727-91.506 25.83-124.65 57.546l-.309-.171c-4.036-2.632-7.875-5.274-14.017-5.45-5.79-.35-11.06-1.391-14.395-4.9-3.685-2.809-7.359-5.955-10.693-9.29-6.843-6.667-13.358-13.872-18.096-22.997-2.281-4.387-6.495-7.915-11.584-9.494-10.178-2.808-20.7 2.983-23.683 13.161-2.983 10.003 2.983 20.705 13.161 23.512 9.827 2.984 18.078 7.897 26.15 13.162a127.38 127.38 0 0 1 11.242 8.602c4.036 2.633 6.132 7.546 7.711 13.162 1.158 5.625 4.386 8.879 7.54 12.03-24.02 34.206-38.248 75.777-38.248 120.645 0 7.422.412 14.743 1.165 21.97-3.937 1.378-7.833 2.873-11.173 6.82-3.86 4.387-8.08 7.875-12.818 8.226a94.03 94.03 0 0 1-14.018 2.64c-9.475 1.228-19.13 1.934-29.131-.172-4.914-.878-10.35.166-14.738 3.324-8.598 5.967-10.524 17.896-4.558 26.494 6.142 8.6 18.066 10.735 26.664 4.593 8.423-5.966 17.368-9.321 26.493-12.304 4.563-1.404 9.143-2.619 13.88-3.496 4.563-1.58 9.64-.19 15.08 2.09 4.52 2.333 8.523 2.148 12.476 1.748 15.446 50.08 49.22 92.035 93.325 118.52-1.507 4.217-2.92 8.611-1.577 14.156 1.053 5.79 1.223 11.25-1.234 15.286-1.93 4.387-4.025 8.573-6.306 12.784-4.913 8.073-10.17 16.15-17.89 23.17-3.685 3.334-6.165 8.064-6.34 13.504-.352 10.529 7.91 19.322 18.438 19.673 10.529.351 19.493-7.91 19.844-18.44.176-10.353 2.976-19.459 6.135-28.584 1.58-4.387 3.515-8.778 5.62-12.99 1.58-4.563 5.776-7.92 10.865-10.728 5.07-2.616 7.35-6.316 9.63-10.214 22.15 7.94 45.898 12.51 70.74 12.51 25.266 0 49.402-4.72 71.87-12.92 2.377 4.055 4.819 7.912 9.905 10.624 5.089 2.983 9.285 6.165 10.864 10.728 2.106 4.387 3.87 8.774 5.45 13.161 3.158 9.125 5.784 18.231 6.135 28.585 0 5.089 2.098 10.024 6.134 13.71 7.897 7.019 19.847 6.487 27.042-1.234 7.019-7.897 6.487-19.848-1.234-27.043-7.721-6.844-12.981-15.093-17.72-23.34-2.28-4.036-4.375-8.402-6.305-12.613-2.457-4.212-2.291-9.496-1.063-15.287 1.404-5.966-.173-10.346-1.576-14.909l-.138-.445c43.755-26.75 77.084-68.833 92.194-118.897l.583.034c4.913.35 9.644.858 14.909-2.125 5.264-2.457 10.552-3.872 15.114-2.468 4.562.702 9.284 1.754 13.846 2.982 9.125 2.632 18.272 5.794 26.87 11.585 4.212 2.632 9.473 3.867 14.737 2.639 10.354-2.282 16.848-12.443 14.566-22.621-2.28-10.354-12.472-16.848-22.825-14.567-10.178 2.282-19.652 1.906-29.304 1.028-4.737-.526-9.314-1.209-14.051-2.262-4.738-.35-8.92-3.5-12.956-7.711-4.036-4.738-8.598-5.965-13.16-7.37l-.309-.102c.61-6.51.994-13.077.994-19.742 0-43.507-13.282-83.992-35.986-117.595 3.328-3.502 6.69-6.828 7.917-12.784 1.579-5.615 3.675-10.529 7.711-13.161 3.51-3.159 7.381-5.967 11.242-8.774 7.896-5.265 16.152-10.354 25.978-13.162 4.914-1.404 9.094-4.751 11.55-9.665 4.914-9.3 1.25-20.865-8.225-25.603-9.3-4.913-20.898-1.25-25.636 8.226-4.738 9.125-11.223 16.33-17.89 22.998-3.51 3.334-7.009 6.51-10.694 9.494-3.334 3.51-8.604 4.55-14.394 4.901-6.142.176-10.016 2.989-14.052 5.621-33.462-33.151-77.946-55.16-127.392-60.014-.527-4.738-.874-9.486-4.559-14.224-3.51-4.738-5.975-9.485-5.449-14.224-.175-4.738.026-9.481.377-14.395 1.053-9.476 2.442-18.937 6.478-28.413 1.93-4.563 2.105-9.992 0-15.08-3.159-7.24-10.258-11.533-17.685-11.517m-25.156 133.91-.857 6.751c-2.457 18.953-4.209 38.08-5.964 57.033a875.999 875.999 0 0 0-2.639 30.195c-8.598-6.142-17.196-12.648-26.321-18.44-15.793-10.704-31.594-21.422-47.914-31.6l-5.518-3.427c24.68-21.267 55.354-35.638 89.213-40.513m50.586 0c35.246 5.074 66.986 20.51 92.16 43.22l-5.86 3.7c-16.145 10.354-31.744 21.068-47.537 31.773-6.317 4.212-12.473 8.603-18.439 12.99-4.738 3.334-11.233.341-11.584-5.45-.527-7.546-1.218-14.904-1.92-22.45-1.754-18.952-3.506-38.08-5.963-57.032zm-173.009 78.83 5.587 5.827c13.336 13.863 26.857 27.205 40.544 40.717 5.791 5.791 11.582 11.252 17.548 16.692 4.211 3.86 2.63 10.69-2.81 12.27-8.599 2.632-17.21 5.075-25.808 7.883-18.074 5.967-36.317 11.604-54.39 18.097l-7.952 2.776c-.174-3.21-.48-6.376-.48-9.631 0-34.926 10.267-67.33 27.761-94.63m297.522 3.462c16.177 26.577 25.67 57.694 25.67 91.169 0 2.931-.304 5.776-.445 8.671l-6.237-1.988c-18.25-5.966-36.48-11.084-54.906-16.348-11.932-3.51-23.882-6.648-35.815-9.631 8.95-8.599 18.272-17.035 26.87-25.809 13.512-13.512 27.007-27.022 40.168-41.06zm-160.67 56.072h21.386c3.334 0 6.505 1.588 8.26 4.045l13.674 17.205c1.93 2.282 2.652 5.586 2.125 8.569l-4.935 21.25c-.527 3.334-2.813 5.96-5.62 7.54l-19.64 9.117a9.363 9.363 0 0 1-9.116 0l-19.672-9.117c-2.808-1.58-5.27-4.206-5.621-7.54l-4.901-21.25c-.526-2.983.195-6.287 2.125-8.569l13.675-17.205c1.754-2.457 4.925-4.045 8.26-4.045m-63.371 83.698c5.44-.878 9.851 4.576 7.746 9.665-3.334 8.774-6.51 17.377-9.494 26.151-6.493 18.075-12.815 35.972-18.781 54.222l-2.4 7.54c-31.23-20.767-55.218-51.326-67.997-87.296l9.322-.788c19.127-1.755 37.903-4.06 57.03-6.341 8.248-.877 16.327-2.1 24.574-3.153m151.623 2.468c8.247.877 16.326 1.765 24.574 2.467 19.127 1.755 38.074 3.502 57.2 4.73l6.101.343c-12.548 35.512-36.008 65.782-66.592 86.576l-1.987-6.375c-5.791-18.25-12.092-36.322-18.234-54.221-2.807-8.073-5.619-15.958-8.602-23.855-1.93-5.09 2.276-10.367 7.54-9.665m-76.086 34.616c2.5 0 5.005 1.26 6.41 3.805 3.86 7.37 7.92 14.57 12.132 21.764 9.476 16.495 18.92 33.176 29.098 49.32l4.147 6.718a176.026 176.026 0 0 1-53.09 8.192 176.144 176.144 0 0 1-51.58-7.712l4.387-7.026c10.178-16.145 19.824-32.654 29.475-49.15a451.58 451.58 0 0 0 12.647-22.106c1.403-2.545 3.874-3.805 6.374-3.805" transform="translate(112 111)"/>
            </svg>
        }
    }
}
