Tailwind
You can style your Dioxus application with whatever CSS framework you choose, or just write vanilla CSS.

One popular option for styling your Dioxus application is Tailwind. Tailwind allows you to style your elements with CSS utility classes. This guide will show you how to setup Tailwind CSS with your Dioxus application.

Setup
Install the Dioxus CLI:

cargo install dioxus-cli
Install NPM: https://docs.npmjs.com/downloading-and-installing-node-js-and-npm

Install the Tailwind CSS CLI: https://tailwindcss.com/docs/installation/tailwind-cli

Create a input.css file in the root of your project with the following content:


@import "tailwindcss";
@source "./src/**/*.{rs,html,css}";
Create a link to the tailwind.css file using manganis somewhere in your rust code:
src/tailwind.rs

use dioxus::prelude::*;

#[component]
fn app() -> Element {
    rsx! {
        // The Stylesheet component inserts a style link into the head of the document
        document::Stylesheet {
            // Urls are relative to your Cargo.toml file
            href: asset!("/assets/tailwind.css")
        }
    }
}
Bonus Steps
Install the Tailwind CSS VSCode extension
Go to the settings for the extension and find the experimental regex support section. Edit the setting.json file to look like this:

"tailwindCSS.experimental.classRegex": ["class: \"(.*)\""],
"tailwindCSS.includeLanguages": {
    "rust": "html"
},
Development
Run the following command in the root of the project to start the Tailwind CSS compiler:

npx @tailwindcss/cli -i ./input.css -o ./assets/tailwind.css --watch
Web
Run the following command in the root of the project to start the Dioxus dev server:

dx serve
Open the browser to http://localhost:8080.
Desktop
Launch the Dioxus desktop app:

dx serve --platform desktop
