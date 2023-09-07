sponge-rs
=========

`sponge` written with Rust.

- The original sponge is https://joeyh.name/code/moreutils .
- The golang version of mine is https://github.com/hymkor/sponge .

```
cat -n HOGEHOGE > HOGEHOGE.sponge
mv HOGEHOGE.sponge HOGEHOGE
```

equals

```
cat -n HOGEHOGE | sponge HOGEHOGE
```
