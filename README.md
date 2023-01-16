# kisaragi-booth-utility
KisaragiEffectiveが開発した[BOOTH][BOOTH-top]™<sup>†1</sup>の利用を補助するための実行ファイル群です。

## はじめに
* このプロジェクトはピクシブ株式会社と何ら関係を持つものではなく、ピクシブ株式会社が主導及び開発するものではありません。

## このプロジェクトの目的
* [BOOTH][BOOTH-top]における継続的デプロイを補助する。

## ユースケース
* 「[BOOTH経済圏](https://inside.pixiv.blog/2020/07/21/160722)」におけるより便利で楽しい生活
* ヘッドレスサーバーとBOOTHとの橋渡し
  * ヘッドレスサーバー＝継続的デプロイ用のサーバーなど、デスクトップ環境が存在しないサーバー
* 継続的インテグレーションの成果物をBOOTHのサーバーへ送信する橋渡し
* ブラウザレス
* …

## このアプリケーションの使い方
### ダウンロード
1. [Releases](https://github.com/KisaragiEffective/kisaragi-booth-utility/releases)からダウンロードします。
2. 一番上にあるバージョンを見ます。
3. `<プラットフォーム>`は以下のとおりです。
    * Windows: x86_64-pc-windows-gnu
    * Linux: unknown-linux-musl
    * macOS: x86_64-apple-darwin
4. `<拡張子>`は次のとおりです。
    * Windows: `zip`
    * Linux: `tar.gz`または`tar.xz`
    * macOS: `zip`
5. `kisaragi-booth-utility_<バージョン>_<プラットフォーム>.<拡張子>`と`kisaragi-booth-utility_<バージョン>_<プラットフォーム>.<拡張子>.sha256sum`をダウンロードします。
6. (推奨) [ハッシュ値を検証](#ハッシュ値の検証)します。
7. `kisaragi-booth-utility_<バージョン>_<プラットフォーム>.<拡張子>`を展開します。
8. 使用を開始するためには、[コマンドライン](#コマンドライン)へ移動します。

#### ハッシュ値の検証
Windows:

1. <kbd>Win</kbd> + <kbd>R</kbd>キーを押して、`powershell`と入力し、<kbd>Enter</kbd>を押します。
2. 次のコマンドをコピーアンドペーストして<kbd>Enter</kbd>を押します。

```pwsh
$archive_file = "kisaragi-booth-utility_0.1.1_x86_64-pc-windows-gnu.zip"
$hash_file = $archive_file + ".sha256sum"
$actual_hash = if ($PSVersionTable.PSCompatibleVersions -contains [System.Version]::New(4, 0)) {
  $hash_obj = Get-FileHash $archive_file -Algorithm SHA256
  $hash_obj.Hash.ToLower() + " " + $(Split-Path $hash_obj.Path -leaf)
} else {
  # Get-FileHash is unsupported
  $hasher = [System.Security.Cryptography.SHA256]::Create()
  $io = New-Object System.IO.StreamReader $archive_file
  $hash_arr = $hasher.ComputeHash($io.BaseStream)
  $stream.Close()
  $hash = ""
  $hash_arr | %{ $hash += $_.ToString("x2") }
  $hash
}
$expected_hash = (type $hash_file) -join ""
if ($actual_hash -eq $expected_hash) {
  Write-Host "Hash OK"
} else {
  Write-Error "Hash Error: '$actual_hash' != '$expected_hash'"
}
```

3. `Hash OK`と表示された場合、検証が完了しています。

Linux/macOS:

1. お好みのPOSIX互換シェルを開きます。
2. 次のコードをコピーアンドペーストして実行します。macOSでは`sha256sum`を`gsha256sum`に変える必要があります。

```sh
#!/bin/sh
actual_hash=$(sha256sum kisaragi-booth-utility_0.1.1_x86_64-unknown-linux-musl.tar.gz)
expected_hash=$(cat kisaragi-booth-utility_0.1.1_x86_64-unknown-linux-musl.tar.gz.sha256sum)
if [ "$original_hash" -eq "$expected_hash" ]; then
    echo "Hash OK"
else
    echo "Hash Error: '$actual_hash' != '$expected_hash'" >&2
fi
```

3. `Hash OK`と表示された場合、検証が完了しています。

### コマンドライン
1. `cmd.exe`、`powershell.exe`、`/bin/sh`、`/bin/bash`、`/bin/zsh`などお好みの「シェル」を開きます。
2. 以下のコマンドでパスワードを取得します。
    * `<ブラウザ>`は`firefox`または`chromium`で置き換えてください。`chromium`を指定するべきブラウザは、Chrome、Edge (バージョン79以降)、Opera (バージョン15以降)、Vivaldi、その他Chromiumを採用しているブラウザです。
      * Internet Explorer、Edge (バージョン18以前)、Safariは手元に試せる環境を用意できないためサポートしません。
    * `<場所>`についてはクッキーが保存されているファイルを指定します。標準的な場所を以下に示します。この場所にない場合、Chromiumをベースとした他のブラウザを使っているか、あるいはプロファイルの場所やインストールする場所を変更されている可能性があります。前者については当該ブラウザのドキュメンテーションを参照してください。後者については恐れ入りますがサポートいたしかねます。
```text
kisaragi-booth-utility get-authorization-token --cookie-file <場所> --browser <ブラウザ>
```

<details><summary>参考：標準的なクッキーが保存されている場所</summary>

* Windows
  * Chrome:  `C:\Users\<ユーザー名>\AppData\Local\Google\Chrome\User Data\<プロファイル名>\Cookies`
  * Edge:    `C:\Users\<ユーザー名>\AppData\Local\Microsoft\Edge\User Data\<プロファイル名>\Cookies`
  * Opera:   `C:\Users\<ユーザー名>\AppData\Roaming\Opera\Opera\Cookies`
  * Vivaldi: `C:\Users\<ユーザー名>\AppData\Local\Vivaldi\User Data\<プロファイル名>\Cookies`
  * Chromium:`C:\Users\<ユーザー名>\AppData\Local\Chromium\User Data\<プロファイル名>\Cookies`
  * Firefox: `C:\Users\<ユーザー名>\AppData\Roaming\Mozilla\Firefox\<プロファイル名>\cookies.sqlite`?
* Linux
  * Chrome:  `~/.config/google-chrome/<プロファイル名>/Cookies`
  * Edge:    `~/.config/microsoft-edge-dev/<プロファイル名>/Cookies`
  * Opera:   `~/.config/opera/Cookies`
  * Vivaldi: `~/.config/vivaldi/<プロファイル名>/Cookies`
  * Chromium:`~/.config/chromium/<プロファイル名>/Cookies`
  * Firefox: `~/.mozilla/firefox/<プロファイル名>/cookies.sqlite`
* macOS
  * Chrome:  `~/Library/Application Support/Google/Chrome/<プロファイル名>/Cookies`
  * Edge:    不明
  * Opera:   不明
  * Vivaldi: 不明
  * Chromium:不明
  * Firefox: `~/Library/Application Support/Firefox/`

</details>

3. 「クッキー」が文字列として出力されるので、選択してコピーします。**この文字列はあなたのパスワードと同じ力を持ちます**。誰にも教えないようにしてください。
4. 以下のコマンドでアップロードします。
    * `<アイテムID>`はBOOTHのIDを指定します。例えば、URLが `https://booth.pm/ja/items/1234567` なら、指定するのは`1234567`です。
    * `<アップロードするファイルのパス>`はファイルのパスです。相対パスまたは絶対パスが指定できます。
    * `<クッキー>`は3でコピーした文字列で置き換えます。

```sh
kisaragi-booth-utility upload -i <アイテムID> -p <アップロードするファイルのパス> -t <トークン>
```

5. 参考に、使用例を示します。

```powershell
# PowerShell
kisaragi-booth-utility upload -i 1234567 -p ./利用規約.pdf -t this_is_dummy_token
```

```bat
@rem cmd.exe
kisaragi-booth-utility upload -i 1234567 -p ./利用規約.pdf -t this_is_dummy_token
```

```sh
#!/bin/sh
# Linux / macOS
kisaragi-booth-utility upload -i 1234567 -p ./利用規約.pdf -t this_is_dummy_token
```

6. サイズが表示されたなら成功です。お疲れ様でした。

### GitHub Actions
当面の間次の方法で代替できます。
1. [コマンドライン](#コマンドライン)の手順1から3を行います。
2. 出力された文字列を[暗号化されたシークレット](https://docs.github.com/ja/actions/security-guides/encrypted-secrets)に設定します。ここでは名前を`BOOTH_LOGIN_CREDENTIAL`にしたと仮定します。
3. 次のYAMLファイルを`.github/workflows`の直下に作成します。

<details><summary>ワークフローファイル</summary>

```yml
# --- CREDIT ---
# This file is distributed on https://github.com/KisaragiEffective/kisaragi-booth-utility.
# The original file is licensed under either of Apache License, Version 2.0 or MIT license at your option. 
# 

# Apache License, Version 2.0
# 
#  Copyright 2023 KisaragiEffective and Kisaragi Marine
# 
#  Licensed under the Apache License, Version 2.0 (the "License");
#  you may not use this file except in compliance with the License.
#  You may obtain a copy of the License at
#
#  https://www.apache.org/licenses/LICENSE-2.0
#
#  Unless required by applicable law or agreed to in writing, software
#  distributed under the License is distributed on an "AS IS" BASIS,
#  WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
#  See the License for the specific language governing permissions and
#  limitations under the License.

# MIT License
#  
#  Permission is hereby granted, free of charge, to any
#  person obtaining a copy of this software and associated
#  documentation files (the "Software"), to deal in the
#  Software without restriction, including without
#  limitation the rights to use, copy, modify, merge,
#  publish, distribute, sublicense, and/or sell copies of
#  the Software, and to permit persons to whom the Software
#  is furnished to do so, subject to the following
#  conditions:
#
#  The above copyright notice and this permission notice
#  shall be included in all copies or substantial portions
#  of the Software.
#
#  THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF
#  ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED
#  TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A
#  PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT
#  SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY
#  CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION
#  OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR
#  IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER
#  DEALINGS IN THE SOFTWARE.
# --- END OF CREDIT ---

name: Deploy to BOOTH
on:
  release:
    type:
      - created
jobs:
  upload:
    runs-on: ubuntu-latest
    steps:
      - uses: robinraju/release-downloader@v1.7
        name: Download compressed package
        with:
          repository: "KisaragiEffective/kisaragi-booth-utility"
          tag: "0.1.0+20220115231500"
          fileName: "kisaragi-booth-utility_0.1.0.20220115231500_x86_64-unknown-linux-musl.tar.gz"
      - uses: robinraju/release-downloader@v1.7
        name: Download hash of compressed package
        with:
          repository: "KisaragiEffective/kisaragi-booth-utility"
          tag: "0.1.0+20220115231500"
          fileName: "kisaragi-booth-utility_0.1.0.20220115231500_x86_64-unknown-linux-musl.tar.gz.sha256sum"
      - name: Validate hash
        run: |
          actual_hash=$(sha256sum kisaragi-booth-utility_0.1.0.20220115231500_x86_64-unknown-linux-musl.tar.gz)
          expected_hash=$(cat kisaragi-booth-utility_0.1.0.20220115231500_x86_64-unknown-linux-musl.tar.gz.sha256sum)
          if [ "$actual_hash" -ne "$expected_hash" ]; then
            echo "[E] different hashes: '$actual_hash' != '$expected_hash'"
            exit 1
          fi
      - name: Extract binary
        run: |
          tar -xvf kisaragi-booth-utility_0.1.0.20220115231500_x86_64-unknown-linux-musl.tar.gz
      - name: Deploy to BOOTH
        env:
          BOOTH_DEPLOY_TOKEN: ${{ secrets.BOOTH_DEPLOY_TOKEN }}
        run: |
          kisaragi-booth-utility -i 1234567 -p target/release/kisaragi-booth-utility -t "$BOOTH_DEPLOY_TOKEN"
```

</details>


## ピクシブ株式会社が定める規約との関連性
* ご利用にあたってはピクシブ株式会社が定める[サービス共通規約](https://policies.pixiv.net/)及び[BOOTHに対する個別規約](https://policies.pixiv.net/#booth)をお守りください。
  * サーバーに極端な負荷をかけるような使い方はおやめください。
  * このアプリケーションを使ってアップロードされた成果物の一切の権利は、引き続きそれを創作したユーザーに帰属します。
  * このアプリケーションの使用に関しては、各自がその一切の責任を負い、開発者は一切の責任を負いません。

## プライバシーポリシー
* このアプリケーションは、テレメトリー、トラッカー、その他それに類するユーザーの同意なく情報を送信する機構を持ちません。
* このアプリケーションは、利用者のリクエストがあった場合、次の情報をピクシブ株式会社のサーバーへ送信することがあります。
  * 利用者が使用する言語
* このアプリケーションは、HTTPの仕様上、次の情報をピクシブ株式会社のサーバーへ送信します。
  * IPアドレス
  * ポート番号
* このアプリケーションは、アカウントの認証が必要な操作を行う際、その認証情報をピクシブ株式会社のサーバーへ送信します。

## ライセンス
以下のファイルはすべてMITライセンスおよびApache License, Version 2.0のデュアルライセンスで提供します。
* `src`下のソースコード
* ソースコードをコンパイルすることによって生成されたバイナリ
* `.github`下のファイル
* `docs`下のファイル
* `README.md`
* `.misspell`
* `misspell.sh`

以下のファイルは[Creative Commons Zero](https://creativecommons.jp/sciencecommons/aboutcc0/)で提供します。
* `.gitignore`
* `Cargo.toml`
* `Cargo.lock`

## 開発体制
* Pull Requestを受け付けています。
  * コンパイルが通らない変更は受け付けることができません。
* バグ報告、機能改善、その他kisaragi-booth-utilityに関する議論はIssueで受け付けます。
  * Twitterやメールに送られても気づかない場合があります。
* 表記ブレをなくすため、ドキュメントを変更した際は`./misspell.sh`を必ず適用してください。
  * `./misspell.sh`が適用できない際は`.misspell`を見てください。
    * `#`が行の1文字目にある場合は飛ばしてください。
    * そうでない場合、ドキュメントの中にある左の文字列を1つ右の文字列に置き換えてください。

## ピクシブ株式会社の担当者様へ
このプロジェクトをご覧いただきましてありがとうございます。
担当者様からお問い合わせいただく際は、電子メールでのお問い合わせをお願いいたします。
件名に「`kisaragi-booth-utility`」と入れていただき、`kisarag.effective+contact[at]gmail.com`へお送りください。お手数ですが、スパム防止のため`[at]`はアットマークで置き換えていただくようお願いします。

†1: BOOTHはピクシブ株式会社の登録商標です。

[BOOTH-top]: https://booth.pm/
