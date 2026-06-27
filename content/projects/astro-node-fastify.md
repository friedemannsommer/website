+++
title = "astro-node-fastify"
description = "Fast, opinionated Node.js adapter for Astro SSR using Fastify with built-in compression and caching"
date = 2024-06-07
+++

A [Node.js](https://nodejs.org/) adapter for [Astro](https://astro.build/) SSR built on top of [Fastify](https://fastify.dev/).
It compresses asset and SSR responses by default and supports configurable cache headers for static assets.

Key features:

* Build-time or runtime compression (`gzip`, `brotli`)
* Configurable `Cache-Control` headers for static assets
* Default response headers for SSR and asset routes

Available on npm as [`astro-node-fastify`](https://www.npmjs.com/package/astro-node-fastify).

The configuration reference is available at [friedemannsommer.github.io/astro-node-fastify][docs].

Source code is hosted on GitHub at [friedemannsommer/astro-node-fastify][repo].

[docs]: https://friedemannsommer.github.io/astro-node-fastify/ "astro-node-fastify documentation"
[repo]: https://github.com/friedemannsommer/astro-node-fastify "astro-node-fastify - GitHub"
