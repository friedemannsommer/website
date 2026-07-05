+++
title = "Redesigning my site"
description = "How I replaced hand-picked hex colors with a contrast-first palette."
date = 2026-06-27
updated = 2026-07-05

[extra]
is_post = true
show_meta = true
+++

I spend most of my time in tooling and back-end code. CSS is _somewhat_ familiar to me, but not quite
home turf. Even so, one workflow is muscle memory by now: pick a hex color, drop it into a contrast
checker, and nudge it until it clears `4.5:1`. That habit is where this redesign began. Gently talking
myself out of it is what I'd like to walk through.

Before touching anything, I set myself one rule: reach only for CSS that [Baseline][baseline] marks as
**widely available**, so I could lean on it without fallbacks. Everything below should clear that bar.

Rule in hand, I started with color, since that's where the old design was showing its age. The palette
was two hand-picked hex values on a permanently dark background: a bright green and a dark maroon.
Neither came from a system. Hand-picking is fine until you want a second theme. With no relationship
between the colors, a light mode means starting from scratch and hoping the two feel consistent.

The relationship problem is only half of it. That `4.5:1` comes from [WCAG 2][wcag-contrast], and its
formula is symmetric, so it reads text and background as interchangeable. In practice it overstates
contrast for dark colors, and I wouldn't trust it for a dark mode. So I went looking for a measure
that knew the difference.

That measure is [APCA][apca]. Unlike WCAG 2, it's **polarity-aware**, so it accounts for text and
background in their real roles. It reports contrast as a single `Lc` value. For body text, the
guidelines put a comfortable level around `Lc 90`.

APCA settles the contrast, and [OKLCH][mdn-oklch] handles the colors that meet it. OKLCH builds on
[Björn Ottosson's][oklab-post] [OKLab][mdn-oklab], a color space designed for perceptual uniformity.
Equal lightness steps look equal _across hues_, so I can solve each role for its target without
guesswork.

> You don't really need OKLCH for any of this. The contrast math works in any color space. Readable
> numbers just make the whole palette much easier to reason about.

With both a perceptual color space and a perceptual contrast measure, I could finally turn the
workflow around. Rather than pick a color and then check it, I set a target `Lc` for each role and
solved for the most vivid in-gamut color that hit it. That's the same contrast-first approach
[Adobe's Leonardo][leonardo] uses for WCAG ratios. The colors fell out of the math instead of being
hand-tuned. And because both themes were solved against the same targets, the light and dark palettes
stay in step on their own.

```css
/* light-mode tokens, each solved for a target APCA contrast */
--color-base-100: oklch(0.98 0.005 95);
--color-base-content: oklch(0.22 0.115 315);
--color-primary: oklch(0.54 0.22 315);
```

Choosing between the two was the last color decision. The site now follows your
[system preference][mdn-pcs] instead of forcing dark mode on everyone.

Typography then brought `Lc` straight back. The contrast a piece of text needs depends on its size
and [weight][apca-fonts]. Small, thin text asks for far more contrast than any colored text can
comfortably supply. So contrast and typography turned out to be two sides of one problem. That settled
the body at `18px` and weight `600`, heavy enough for the link, code, and status colors to clear their
targets.

Hierarchy is the other half. It comes from semantic HTML and a type scale rather than one-off font
sizes. Each heading rank sits a fixed ratio above the last, and [`clamp()`][mdn-clamp] makes them
fluid. Because the hierarchy lives in the markup, screen-reader users move through the page in the
same order sighted readers scan it.

Last of all, I dropped the bundled webfont ([Open Sans][open-sans]). For a small site like this, the
[`system-ui`][mdn-system-font] stack does the job just as nicely. It skips the usual webfont tax, and
it's one less thing to keep updated.

And that's about it. The theme still needs some work, but I'll keep tweaking it as I go. If something's
off or unclear, or you spotted an issue, I'd love to hear from you [@frisom][mastodon].

---

_Update, 2026-07-05:_ I rewrote this post not long after publishing it. The original split each topic
under its own heading. That felt tidy while I was deep in the redesign. Reading it back, though, I
found the headings chopped the piece up and made it harder to follow. It now runs as continuous prose,
with each section easing into the next.

[apca-fonts]: https://git.apcacontrast.com/documentation/APCA_in_a_Nutshell.html "APCA – font size and weight lookup tables"
[apca]: https://apcacontrast.com/ "APCA – Accessible Perceptual Contrast Algorithm"
[baseline]: https://web.dev/baseline/ "Baseline – web.dev"
[leonardo]: https://leonardocolor.io/ "Leonardo – contrast-based color generation (Adobe)"
[mastodon]: https://social.famsom.net/@friedemann "frisom on Mastodon"
[mdn-clamp]: https://developer.mozilla.org/en-US/docs/Web/CSS/clamp "clamp() – MDN"
[mdn-oklab]: https://developer.mozilla.org/en-US/docs/Web/CSS/color_value/oklab "oklab() – MDN"
[mdn-oklch]: https://developer.mozilla.org/en-US/docs/Web/CSS/color_value/oklch "oklch() – MDN"
[mdn-pcs]: https://developer.mozilla.org/en-US/docs/Web/CSS/@media/prefers-color-scheme "prefers-color-scheme – MDN"
[mdn-system-font]: https://developer.mozilla.org/en-US/docs/Web/CSS/font-family "font-family (system-ui) – MDN"
[oklab-post]: https://bottosson.github.io/posts/oklab/ "A perceptual color space for image processing – Björn Ottosson"
[open-sans]: https://fonts.google.com/specimen/Open+Sans "Open Sans - a humanist sans serif typeface"
[wcag-contrast]: https://www.w3.org/TR/WCAG21/#contrast-minimum "WCAG 2.1 – Contrast (Minimum)"
