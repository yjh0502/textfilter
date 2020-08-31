const filter = require('./pkg');
const fs = require('fs');

const start_read = Date.now();
fs.readFile('formatted.json', (err, content) => {
    const parsed = JSON.parse(content);
    console.log(`parsing took=${Date.now() - start_read}ms, words=${parsed.data.badWordList.length}`);

    const texts = [
        'fuck fuxk fuuuuuk foo fuck',
        'Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat. Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat cupidatat non proident, sunt in culpa qui officia deserunt mollit anim id est laborum',
    ];

    for(const text of texts) {
        const start = Date.now();
        const filtered = filter.filter(text, parsed.data.badWordList);
        console.log(`filtered=${filtered.result}, keywords=${filtered.keywords}, took=${(Date.now() - start)}ms`);
    }
});

