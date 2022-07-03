+++
title = "JavaScript cooperative async processing"
description = "Cooperative multitasking in JavaScript"
date = 2022-03-08
updated = 2022-03-13

[extra]
is_post = true
show_meta = true
+++

As someone who is _somewhat_ familiar with JavaScript I will try to explain how it's possible to execute _expensive_
tasks on a "single-threaded" runtime. As you might have already guessed this has pros and cons, and it's
**no silver bullet**. I refer to it as "cooperative processing", but it's also known as
"[cooperative multitasking][coop-multitask]". (This term is probably something most have heard from in context
of operating systems.)

> I acknowledge that it's possible to implement this via [async* Generators][async-generator], but we won't be
> using them since they require [Promise][promise] instances which **can** be relatively expensive.

The following examples are designed to be used within browsers, but can also be used with [Node.js][nodejs],
[Deno][deno] or any other [ES5][es5] compatible runtime with [Timers][whatwg-timers].

Given an example task to iterate through a large list of entries and apply some processing to them,
some might implement this:

```typescript
const entries = Array(10e8)
let result = 0

for (const entry of entries) {
    result += entry
}
```

Don't get me wrong, there is nothing wrong with this, if it's not blocking some other _higher_ priority task.
Which is nearly always the case if this or something similar is being run on the (browser) main thread. To work
around this, I would prefer the following implementation:

```typescript
const entries = Array(10e8)

function processEntries(
    entries: number[],
    onComplete: (sum: number) => void
): void {
    const length = entries.length
    let offset = 0
    let result = 0

    function _process(): void {
        // process up to fifty entries at once
        const limit = Math.min(offset + 50, length)

        for (let index = offset; index < limit; index++) {
            result += entries[index]
        }

        if (limit === length) {
            // we've reached the end
            onComplete(result)
        } else {
            // advance offset by batch size
            offset += 50
            // 24ms since we're targeting 60 FPS
            // 16ms * 1.5 (scripting multiplier) = 24ms
            setTimeout(_process, 24)
        }
    }

    _process()
}
```

The second example will be slower overall, but since execution is "paused" after fifty entries the main thread
can process user input or execute other tasks. Some might be thinking:
"Why would I write more code and have it execute slower?" That's a good question!

Let me _try_ to explain it by using a relatively modest monitoring dashboard as an example. Given that example
let's say there are fourteen different widgets that need to be updated every five seconds with data from the
last thirty minutes. A given data point represents one second, which means there are
`14 widgets * (30 minutes * 60 seconds)` entries which need to be processed. This will yield up to `25200` entries,
which is a relatively modest number for a modern PC, but updating all fourteen widgets in one task will lead to
unresponsiveness for a small amount of time which isn't a great user experience.

A possible solution would be, to process the data per widget per time budget (which was `24ms` in the previous
example). You are correct in thinking that this might lead to visual differences, but these should only be minor
and be resolved within the next one to three seconds (depending on the users PC). Overall this _should_ be a more
responsive user experience than updating them all at once. **But** since every application is at least slightly
different and might have different goals this isn't necessary.

The given example does not reflect most _real_ tasks, but I hope that it was understandable. In most _real_
applications these processing times simply add up by abstraction or other tasks running "simultaneously".
Some applications can use [Web Workers][web-worker] or [WebAssembly][wasm] to process tasks outside the main thread,
but this creates other hurdles which may not be worth for some.

All right, that's all. If I made a mistake or something isn't clear, try contacting me [@free_some_mem][twitter].

[coop-multitask]: https://en.wikipedia.org/wiki/Cooperative_multitasking "Cooperative multitasking - Wikipedia"
[promise]: https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Promise "Promise - MDN"
[async-generator]: https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Symbol/asyncIterator "Async Generator - MDN"
[nodejs]: https://nodejs.org/en/
[deno]: https://deno.land/
[es5]: https://262.ecma-international.org/5.1/ "ECMAScript 5.1"
[whatwg-timers]: https://html.spec.whatwg.org/multipage/timers-and-user-prompts.html#dom-settimeout-dev "HTML Timers"
[web-worker]: https://developer.mozilla.org/en-US/docs/Web/API/Web_Workers_API/Using_web_workers "Web Workers - MDN"
[wasm]: https://developer.mozilla.org/en-US/docs/WebAssembly "WebAssembly - MDN"
[twitter]: https://twitter.com/free_some_mem "free_some_mem on Twitter"
