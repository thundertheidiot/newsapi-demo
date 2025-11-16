# NewsAPI reader

Basic news feed application using [NewsAPI.org](https://newsapi.org).

## Features

- View the daily top headlines
- Search for articles
- Easily filter news by source

## Setup

This is a rust project, cargo is required for building. [Rustup](https://rustup.rs/) is the recommended way to install the rust components.

On Linux you need Wayland or X11 libraries installed, depending on your environment. Native Rustls is used instead of OpenSSL. Windows compatibility is untested.

After acquiring cargo, simply:
```bash
cargo run --release
```

There is also a [nix](https://nixos.org) flake that bundles all of the dependencies, you can `nix run` if you have nix installed.

## Usage

Input your NewsAPI token in the input box. Alternatively set the `NEWS_API_TOKEN` environment variable, which is automatically read on startup.

![Token input page](/readme/token.png)

The next page automatically loads the top headlines of the day. 

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

![Detailed article view](/readme/article.png)

Clicking on any of the article cards opens a more detailed view of the article. You can click the button at the bottom to open the full article.

Right clicking on any of the sub menus will close the menu.

## About the project

I picked [Iced](https://iced.rs) as the GUI library, because it was the only rust gui library I had heard about before, and I wanted to learn one.
