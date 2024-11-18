# Wiki-Corpus

Extract texts as JsonLine from Wikipedia dump (.bz2).

## Quick Start

1. install the `wiki-corpus` crate.

    ```bash
    cargo install wiki-corpus
    ```

2. prepare wikipedia dump.

    https://dumps.wikimedia.org/enwiki/latest/

    -> download `enwiki-latest-pages-articles-multistream.xml.bz2`

3. convert the bz2 file.

```
wiki-corpus --input <PATH/TO/enwiki-latest-pages-articles-multistream.xml.bz2>
```

## Releases

### v1.0.0
- First release.