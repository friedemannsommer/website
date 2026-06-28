+++
title = "Redesigning my site"
description = "How I modernized this site using surprisingly widely available CSS."
date = 2026-06-27
updated = 2026-06-28

[extra]
is_post = true
show_meta = true
+++

For context, I'm a full-stack engineer who spends most of the time in tooling and back-end code,
so CSS is familiar but not my home turf.
This site had run on the same theme for years, and the redesign had two goals:
make it more **accessible**, and **modernize** the implementation.
The rule I set myself was to reach only for CSS that [Baseline][baseline] marks as widely
available today, and to use the [APCA][apca] contrast algorithm.

---

## Color from contrast

The old palette was two hand-picked hex values (a bright green for primary actions, a dark maroon
for hover) on a permanently dark background.
They were chosen somewhat at random, with no system behind them.

So this time, I built the new palette in [OKLCH][mdn-oklch], the cylindrical form of
[Björn Ottosson's][oklab-post] perceptually uniform [OKLab][mdn-oklab] color space.
Equal steps in lightness look equal across hues, so reasoning about complementary colors is easier.

The bigger shift was giving up on picking colors and then checking their contrast.
Instead I derived each color from a target contrast, the approach [Adobe's Leonardo][leonardo] popularized,
and measured it with [APCA][apca].
APCA is perceptually uniform and polarity-aware: it accounts for light text on a dark background reading
differently than the reverse, which WCAG 2's ratio doesn't.

So for each role I solved for the most vivid in-gamut color that hit the target APCA contrast.
The colors _fell_ out of the math instead of being nudged by hand, and since light and dark were solved
against the same targets, the two themes stay in step.

```css
/* light-mode tokens, derived from APCA contrast targets */
--color-base-100: oklch(0.98 0.005 95);
--color-base-content: oklch(0.22 0.115 315);
--color-primary: oklch(0.54 0.22 315);
```

With both themes solved in tandem, the last step was letting people choose between them: the site now
follows the [system preference][mdn-pcs] rather than forcing dark mode on everyone.

---

## Typepography, a contrast problem

With color settled, hierarchy was the next layer to rebuild.
It now comes from semantic HTML and a consistent type scale rather than one-off font sizes.
Headings sit on a modular scale, each rank fluid via [`clamp()`][mdn-clamp], and
spacing follows a small set of rhythm tokens.

Because the hierarchy lives in the markup and the scale, the visual order and the document outline
stay in sync, which means the structure a screen reader walks is the one a sighted reader sees.

The scale is also relevant for another reason: APCA ties the contrast a piece of text needs to its
size and [weight][apca-fonts].
Small, thin text needs far more contrast than any colored text can supply,
so the body settled at 18px and weight 600, large and heavy enough that the link, code,
and status colors clear their targets.
Because of this, contrast and typography turned out to be the same problem.

---

## One less font

The old theme bundled a webfont ([Open Sans][open-sans]).
For a site like this there was no real benefit: modern devices ship with perfectly good system fonts.
Moving to the [`system-ui`][mdn-system-font] stack dropped a network request and the layout shift that comes with it,
and there's one less thing to keep updated.

---

The new theme still needs some work, but I'll make changes as I go.

If you're interested in the details or found an issue, [let me know][mastodon].

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
