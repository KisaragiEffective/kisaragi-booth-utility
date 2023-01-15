# kisaragi-booth-utility
KisaragiEffectiveが開発した[booth][booth-top]™<sup>†1</sup>の利用を補助するための実行ファイル群です。

## はじめに
* このプロジェクトは株式会社ピクシブと何ら関係を持つものではなく、ピクシブ株式会社が主導及び開発するものではありません。

## このプロジェクトの目的
* [booth][booth-top]におけるContinuous Deploymentを行う。

## このアプリケーションの使い方
### コマンドライン
1. releasesからダウンロードします。
2. 以下のコマンドでパスワードを取得します。
```
kisaragi-booth-utility get-authorization-token --cookie-file <場所> --browser <ブラウザ>
```

`<ブラウザ>`は`firefox`または`chromium`で置き換えてください。`chromium`を指定するべきブラウザは、Chrome、Edge (バージョン79以降)、Opera (バージョン15以降)、Vivaldi、その他Chromiumを採用しているブラウザです。Internet Explorer、Edge (バージョン18以前)、Safariは手元に試せる環境を用意できないためサポートしません。また、`<場所>`についてはクッキーが保存されているファイルを指定します。標準的な場所を以下に示します。この場所にない場合、Chromiumをベースとした他のブラウザを使っているか、あるいはプロファイルの場所やインストールする場所を変更されている可能性があります。前者については当該ブラウザのドキュメンテーションを参照してください。後者については恐れ入りますがサポートいたしかねます。
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

3. トークンが文字列として出力されるので、選択してコピーします。**この文字列はあなたのパスワードと同じです**。誰にも明かさないようにしてください。
4. 以下のコマンドでアップロードします。

```sh
kisaragi-booth-utility upload -i <boothのアイテムのID> -p <アップロードするファイルのパス> -t <トークン>
```

### GitHub Actions
当面の間次の方法で代替できます。
1. コマンドラインの手順2と3を行います。
2. 出力された文字列を[暗号化されたシークレット](https://docs.github.com/ja/actions/security-guides/encrypted-secrets)に設定します。ここでは名前を`BOOTH_LOGIN_CREDENTIAL`にしたと仮定します。
3. 次のYAMLファイルを`.github/workflows`の直下に作成します。

```yml
# 適宜編集してください
name: Deploy to booth
on:
  release:
    type:
      - created
jobs:
  upload:
    runs-on: ubuntu-latest
    steps:
      -
        env:
          BOOTH_LOGIN_CREDENTIAL: ${{ secrets.BOOTH_LOGIN_CREDENTIAL }}
        run: |
          # TODO
          # wget 
          # kisaragi-booth-utility
```

## ピクシブ株式会社が定める規約との関連性
* ご利用にあたってはピクシブ株式会社が定める[サービス共通規約](https://policies.pixiv.net/)及び[boothに対する個別規約](https://policies.pixiv.net/#booth)をお守りください。
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
* このアプリケーションは、アカウントの認証が必要な操作を行う際、その情報をピクシブ株式会社のサーバーへ送信します。

## ピクシブ株式会社の担当者様へ
このプロジェクトをご覧いただきましてありがとうございます。
お問い合わせいただく際は、電子メールでのお問い合わせをお願いいたします。
件名に「`kisaragi-booth-utility`」と入れていただき、`kisarag.effective+contact[at]gmail.com`へお送りください。お手数ですが、スパム防止のため`[at]`はアットマークで置き換えていただくようお願いします。

†1: boothはピクシブ株式会社の登録商標です。

[booth-top]: https://booth.pm/
