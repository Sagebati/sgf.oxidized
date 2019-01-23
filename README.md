SGF .oxidized
============
use pest to parse the SGF

*Simple SGF (SMART GAME FORMAT) parser for rust*


It's the 0.0.1 the API can change.

Ex:

```rust
    #[test]
        fn from_file_test() {
            let sgf = Sgf::from_file("test.sgf").unwrap();
            assert_eq!(sgf.collection.first().unwrap().root["FF"].values.first().unwrap(), "4")
        }
```

The file begin with : 
```
;FF[4]GM[1]SZ[19]ST[2]CA[UTF-8]SO[gokifu.com]AP[SGFC:1.16]
```

