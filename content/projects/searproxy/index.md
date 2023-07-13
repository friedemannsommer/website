+++
title = "SearProxy"
description = "A SearX / SearXNG compatible web content sanitizer proxy"
date = 2022-02-12
updated = 2022-10-20
+++

A [SearX][searx] / [SearXNG][searxng] compatible web content sanitizer proxy, which is heavily inspired by
[Morty][morty].

I've created this project since I wanted a stricter sanitation proxy which still allows some modern features such
as: `<video>`, `<audio>`, `<picture>`, and `<source>`. To prevent leakage through unknown or unexpected resource links,
this project also uses a [Content Security Policy][csp] which allows
only itself. Like [Morty][morty] it also supports an HTTP or SOCKS5 proxy to tunnel the outgoing traffic. But
unlike [Morty][morty] it requires a HMAC secret to validate the given URL and does not allow direct URL opening.

This project currently has the following features:

* HTML sanitization (`<applet>`, `<canvas>`, `<embed>`)
* Resource reference rewrite
* JavaScript blocking (`<script>`, `on*="code"`)
* No cookies, caching, or referrers
* HTML `<form>` with `GET` or `POST`
* HTML `<img>` async decoding and optionally "lazy" loading
* HTML `<iframe>`, `<video>`, `<audio>`

To use it for [SearX][searx] or [SearXNG][searxng] define a `result_proxy` section within the `settings.yml`. Inside
this section define a `url` with the public base URL to this service and a `key` which is the [HMAC][hmac] secret that's
used to validate the given URL. This project can also be used as image proxy if `server.image_proxy` is set to `true`.
(See [SearX settings.yml][searx_image_proxy], [SearXNG settings.yml][searxng_image_proxy])

```yaml
result_proxy:
  url: https://proxy.example.com/
  key: !!binary "hmac_secret"

server:
  image_proxy: true
```

Alternatively, see the documentation for the [SearX result proxy][searx_morty]. (SearXNG has removed their documentation
for result proxies, but still support them just like SearX.)

The source code for this project can be found on GitHub at [friedemannsommer/searproxy][searproxy].

[csp]: https://developer.mozilla.org/en-US/docs/Web/HTTP/CSP "Content Security Policy - MDN"
[hmac]: https://en.wikipedia.org/wiki/HMAC "HMAC - Wikipedia"
[morty]: https://github.com/asciimoo/morty "Morty - GitHub"
[searproxy]: https://github.com/friedemannsommer/searproxy "SearProxy - GitHub"
[searx]: https://github.com/searx/searx "SearX - GitHub"
[searx_morty]: https://searx.github.io/searx/admin/morty.html "SearX result proxy documentation"
[searx_image_proxy]: https://searx.github.io/searx/admin/settings.html#server "SearX settings.yml documentation"
[searxng]: https://github.com/searxng/searxng "SearXNG - GitHub"
[searxng_image_proxy]: https://docs.searxng.org/admin/settings/settings_server.html "SearXNG settings.yml documentation"
