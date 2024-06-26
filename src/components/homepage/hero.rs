use crate::*;
use manganis::mg;

pub(crate) fn Hero() -> Element {
    rsx! {
        section { class: "w-full dark:bg-ideblack h-fit [@media(min-height:720px)]:min-h-[calc(100vh-6rem)] flex flex-col justify-between items-center py-16",
            div { class: "flex flex-wrap items-center pb-12 md:px-12 max-w-screen-2xl mx-auto text-center my-auto",
                div { class: "relative w-full mx-4 sm:mx-auto text-gray-600",
                    div { class: "text-[3em] md:text-[5em] font-semibold dark:text-white text-ghdarkmetal font-sans py-12 flex flex-col",
                        span { "Fullstack, crossplatform," }
                        span { "lightning fast, fully typed." }
                    }
                    // img { src: "https://ribir.org/landing-page/hero-banner.png" }
                    // h1 { class: "text-2xl md:text-[5em] font-semibold dark:text-white text-ghdarkmetal font-sans py-12 flex flex-col",
                    // h1 { class: "text-2xl md:text-[5em] font-semibold dark:text-white text-ghdarkmetal font-sans py-12 flex flex-col",
                    //     "Fullstack, crossplatform, lightning fast, fully typed."
                    // }
                    h3 { class: "text-[2em] dark:text-white font-extralight text-ghdarkmetal pt-4 max-w-screen-md mx-auto",
                        "Dioxus is a Rust library for building apps that run on desktop, web, mobile, and more."
                    }

                    img {
                        src: "/static/multiplatform-dark.svg",
                        class: "mx-auto max-h-48 lg:max-h-96 dark:hidden",
                        alt: "Dioxus Contributors"
                    }
                    img {
                        src: "/static/multiplatform-light.svg",
                        class: "mx-auto max-h-48 lg:max-h-96 hidden dark:block",
                        alt: "Dioxus Contributors"
                    }

                    div { class: "pt-12 text-white text-[1.2em] font-sans font-bold flex flex-col md:flex-row justify-center md:space-x-4 md:space-y-0 space-y-4",
                        Link {
                            to: Route::Docs {
                                child: BookRoute::GettingStartedIndex {},
                            },
                            class: "bg-dxorange m-0 p-4 px-8 rounded md:hover:-translate-y-2 transition-transform duration-300 w-full md:w-auto",
                            "Quickstart"
                        }
                        Link {
                            to: Route::Docs {
                                child: BookRoute::ReferenceIndex {},
                            },
                            class: "bg-dxblue m-0 p-4 px-8 rounded md:hover:-translate-y-2 transition-transform duration-300 w-full md:w-auto",
                            "Read the docs"
                        }
                    }

                    div { class: "max-w-screen-2xl mx-auto pt-36 hidden",
                        h1 { class: "text-md", "Trusted by top companies" }
                        div { class: "pt-4 flex flex-row flex-wrap justify-center invert dark:invert-0",
                            div { class: "h-12 w-40 p-2 m-4 flex justify-center items-center",
                                img { src: "static/futurewei_bw.png" }
                            }
                            div { class: "h-12 w-40 p-2 m-4 flex justify-center items-center",
                                img { src: "static/airbuslogo.svg" }
                            }
                            div { class: "h-12 w-40 p-2 m-4 flex justify-center items-center",
                                img { src: "static/ESA_logo.svg" }
                            }
                            div { class: "h-12 w-40 p-2 m-4 flex justify-center items-center",
                                img { src: "static/yclogo.svg" }
                            }
                            div { class: "h-12 w-40 p-2 m-4 flex justify-center items-center",
                                img { src: "static/satellite.webp" }
                            }
                        }
                    }
                }
            }
        }
    }
}

fn AnimatedIcon() -> Element {
    let dark = include_str!("../../../public/static/multiplatform-dark.svg");
    let light = include_str!("../../../public/static/multiplatform-light.svg");

    let a = 123;

    let b = 123;

    rsx! {
        div {
            div { class: "dark:hidden", dangerous_inner_html: "{dark}" }
            div { class: "hidden dark:block", dangerous_inner_html: "{light}" }
        }
    }
}
