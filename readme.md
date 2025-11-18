# NewsAPI reader

Basic news feed application using [NewsAPI.org](https://newsapi.org).

## Features

- View the daily top headlines
- Search for articles
- Easily filter news by source

## Quick start

1. Set up cargo and the rust toolchain with [Rustup](https://rustup.rs).
2. Clone the repository
3. Run:
```bash
cargo run --release
```

## Setup

This is a rust project, cargo is used as the build tool. 

On Linux, Iced (the gui toolkit) requires system wayland or x11 libraries depending on your environment. 

There shouldn't be any extra setup required on windows, I made sure to use cross platform solutions like using native rustls with reqwest instead of openssl. Despite this, Windows support is untested, because I do not own a machine running Windows, same goes for MacOS.

There is a [nix](https://nixos.org) flake, that provides a package and a developement shell, that bundle all of the dependencies. You can easily run the application through `nix run`, if you have nix installed.
`nix develop` will drop you into a developement shell with all the dependencies.

## Usage

Paste your NewsAPI token in the input box. Alternatively set the `NEWS_API_TOKEN` environment variable, which is automatically read on startup.

![Token input page](/readme/token.png)

The main page automatically loads the top headlines of the day. 

![Main page](/readme/main.png)

You can search for articles using the search box at the top of the page.

The advanced search functionality supported by NewsAPI works here.
- Surround phrases with quotes (") for exact match.
- Prepend words or phrases that must appear with a + symbol. Eg: +bitcoin
- Prepend words that must not appear with a - symbol. Eg: -bitcoin
- Alternatively you can use the AND / OR / NOT keywords, and optionally group these with parenthesis. Eg: crypto AND (ethereum OR litecoin) NOT bitcoin.

![Source page](/readme/source.png)

The sources button on the top right opens the sources menu, where you can toggle sources to filter by. Right clicking the sources button resets your source filters.

You can filter the sources using the text input box at the top of the source input menu.

![Source page](/readme/source_filtering.png)


Clicking on any of the article cards opens a more detailed view of the article. You can click the button at the bottom to open the full article.

![Detailed article view](/readme/article.png)

## About

This project was made to fulfill a preliminary task for a job internship application, but I also used this as a learning opportunity. Using reqwest for a rest api was already a familiar task, but I hadn't really made a proper GUI before.

I picked [Iced](https://iced.rs) as the GUI library, because it was the only Rust GUI library I had heard about before, and I wanted to learn one.
