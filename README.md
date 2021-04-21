
```sh
cargo install cargo-compete
alias procon='cargo compete'
```

```sh
cd atcoder
procon n 
```

```sh
cd codeforces
procon n  
```

```sh
cd yukicoder
procon n 305 
cd $_
procon t a
```

```sh
cd aoj/problems
procon a ITP1_7_B
procon o ITP1_7_B
procon t ITP1_7_B
```

### snippet
```sh
cargo install cargo-snippet --features="binaries"
```

```sh
cd snippets
cargo snippet -t vscode > $(wslpath "$(wslvar USERPROFILE)")/AppData/Roaming/Code/User/snippets/rust.json 
cargo snippet -t vscode > $HOME/Library/Application Support/Code/User/snippets/rust.json
```