# Blog Generator

A small blog generator for this website.

## How to Use

```
cargo run <config.toml>
```

The config should contain paths necessary to build the path in particular `input_dir`, `output_dir`, and `templates_dir`.

Look at [`../generate-blog.sh`](../generate-blog.sh) and [`../ssg.toml`](../ssg.toml) for an example.

Because we use absolute paths from domain root (e.g. `<link rel="/style.css">`), it is necessary to test from a server instead of `file://`. I do the following:

```
cd ../docs && uv run python3 -m http.server
```

## Markdown format

Each .md file must come with the following frontmatter to be generated:

```yaml
---
title: Title
date_created: yyyy-mm-dd
tags: []

# optional
date_modified: yyyy-mm-dd
blurb: The summary that appears on the index page.
draft: false # will not generate if true
---
```

A variety of different Markdown features are supported, thanks to the `pushdown-cmark` library. I have extended this with jumpable footnotes and KaTeX also.

## Description

The generator hard-codes certain paths for the files to be generated. There are:

```
path/to/input/
  first-post/post.md
  second-post/
    post.md
    asset.png
  third-post/post.md
...

path/to/templates/
  base.html
  post.html
  post_index.html
  tag.html
  tag_index.html

path/to/output/
  posts/
    first-post/index.html
    second-post/
      index.html
      asset.png
    third-post/index.html
    ...
    index.html             <-- redirects to ../index.html
  tags/
    tag-1/index.html
    tag-2/index.html
    ...
    index.html
  index.html

```

Note that each post `posts/slug` may have only one .md file, as the output will write only one of them to `slug/index.html`.
