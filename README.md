# Rust learning

Just playing around with Rust, following the official book:

https://doc.rust-lang.org/book/title-page.html


## Troubleshooting

I run into a https error while installing a cargo package. I had to create a config file:

`c:\Users\username\.cargo\config`

```toml
[http]
check-revoke = false
```

## Practice apps

[ ] Todo app: command line based
[ ] Git commit analyzer: A cli tool that parses Conventional Commits
