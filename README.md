## Textfilter

Substitute keywords from a text.

## Usage

```js
const filter = require('@jihyun.yu/textfilter');
const assert = require('assert').strict;

const text = 'hello world foo bar baz';
const keywords = ['foo', 'bar'];
const filtered = filter.filter(text, keywords);

assert.equal('hello world *** *** baz', filtered.result);
```
