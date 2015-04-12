# motors - A super small template library, inspired by Mote

Inspired and based on [mote](https://github.com/soveran/mote).

## Documentation

[Online documentation][doc].

## Basic operation

```rust
use motors;
let tmpl = motors::Template::parse("Hello {{user}}!").unwrap();
let mut ctx = HashMap::new();

ctx.insert("user", "Ada");
assert_eq!("Hello Ada!", tmpl.render(&ctx));
```

## Contribute

If you find bugs or want to help otherwise, please [open an issue][issues].

## License

MIT. See [LICENSE](LICENSE).

[doc]: http://badboy.github.io/motors/doc/motors/
[issues]: https://github.com/badboy/motors/issues
