## General
Utils
- [Calender](https://competitiveprogramming.info/calendar)
- [AtCoder Problems](https://kenkoooo.com/atcoder/#/table/)

## Workflows
Utils
- [cargo-compete](https://github.com/qryxip/cargo-compete)

Setup
```sh
rustup update
cargo install cargo-compete
alias procon='cargo compete'
git clone https://github.com/fuyutarow/procon
cd procon
```

### AtCoder
https://atcoder.jp/
```sh
cd atcoder
procon n abc095
cd $_
procon o --bin a
procon t a
procon s a
```

### Codeforces
https://codeforces.com/
```sh
cd codeforces
procon n 715 
cd $_
procon o --bin a
procon t a
procon s a
```

### yukicoder
https://yukicoder.me/
```sh
cd yukicoder
procon n 305 
cd $_
procon t a
```

### Aizu Online Judge
https://judge.u-aizu.ac.jp/onlinejudge/
```sh
cd aoj/problems
procon a ITP1_7_B
procon o ITP1_7_B
procon t ITP1_7_B
```

## snippet
Utils
- [cargo-snippet](https://github.com/hatoo/cargo-snippet)

Setup
```sh
cargo install cargo-snippet --features="binaries"
```

Set the snippets to vscode
```sh
cd snippets
cargo snippet -t vscode > $(wslpath "$(wslvar USERPROFILE)")/AppData/Roaming/Code/User/snippets/rust.json 
cargo snippet -t vscode > $HOME/Library/Application Support/Code/User/snippets/rust.json
```