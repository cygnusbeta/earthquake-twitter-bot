# earthquake-twitter-bot

茨城大学理学部（水戸キャンパス）に設置されている地震計の地震観測情報をツイートする bot（非公式・非公認）

### ツイート内容について

- 震度は最大リアルタイム震度（RI）なので注意
  - Q. リアルタイム震度（RI）とは？ -> http://acrs.sci.ibaraki.ac.jp/kaisetsu.html

### プログラムについて

- http://acrs.sci.ibaraki.ac.jp/ を定期的にフェッチして情報が更新されていたらツイートするだけのプログラム
- GCE 無料枠（f1-micro）で安定して動かしたかったので、省メモリ・省 CPU な Rust 製
- ライセンス：MIT
- 製作者：[@cygnus_beta](https://twitter.com/cygnus_beta)
