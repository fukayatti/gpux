# gpux (GPUI eXtended) - React-like DX for GPUI

[GPUI](https://github.com/zed-industries/zed/tree/main/crates/gpui) フレームワーク上に、**「Reactライクな開発体験（JSX + Hooks）」と「Tailwind CSS のユーティリティクラス」** をもたらす実験的なマクロライブラリです。

`rstml` を活用した `view!` マクロの中で Tailwind クラスを用いた直感的な UI 構築を行いながら、Reactでお馴染みの Hooks（`use_state` や `use_effect` など）を利用できます。これらはすべてコンパイル時に、オーバーヘッドのない GPUI ネイティブのメソッドチェーンへと静的に変換されます。

---

## ⚠️ 免責事項
**これは実験的なプロトタイプ（アルファ版）です。**
GPUI の限界と、手続き型マクロによるコード生成の可能性を探る目的で作成されています。
現在は `gpui = "0.2.2"` に依存して動作確認を行っていますが、GPUI 本体に破壊的変更があった場合は動かなくなる可能性があります。

## 主な機能

- **Tailwind CSS パーサー**: コンパイル時に Tailwind のクラス（`flex`, `w-full`, `bg-blue-500` など）を GPUI のメソッド（`.flex()`, `.w_full()`, `.bg(...)` など）へ自動変換します。
- **Arbitrary Values (任意の値)**: Tailwind 独自のブラケット構文（`w-[150px]`, `p-[16px]`, `bg-[#123456]` など）に完全対応。
- **疑似クラス・状態修飾子**: `hover:`, `active:`, `focus:`, `group-hover:` などの状態に応じたスタイリングが可能。
- **レスポンシブデザイン**: 画面の幅（viewport）に応じてスタイルを切り替える `sm:`, `md:`, `lg:`, `xl:`, `2xl:` ブレイクポイントに対応。
- **アニメーション**: GPUI のアニメーションAPIを活用し、`animate-pulse`, `animate-bounce`, `animate-ping`, `animate-fade-in` などのアニメーションを再現。
- **Reactライクな Hooks エンジン**:
  - `use_state!`
  - `use_effect!`
  - `use_memo!`
  - `use_context!`
  - `use_future!` (Suspense のような非同期データのフェッチ・ローディング状態の管理)
  - `use_animation!` (独自のカスタムアニメーション用)
- **Portals**: `<Portal>` タグを使うことで、通常の DOM 階層から抜け出し、画面の最前面にモーダルやツールチップをオーバーレイ描画できます。

## プロジェクト構造

このワークスペースは以下の3つのクレート（ディレクトリ）から構成されています。
- `core/` (`gpux_macros`): JSX の構文解析（`rstml`）と GPUI 用の Rust コード生成を担う、手続き型マクロ（proc-macro）本体です。
- `src/` (`gpux`): コンポーネントの状態を管理する `Hooks` アリーナなどのランタイム実装を提供し、マクロを外部にエクスポートするラッパークレートです。
- `demo_app/`: 本ライブラリの各種機能がどのように動くかを検証できるサンプルアプリです。

## クイックスタート

### 1. コンポーネントの定義

関数に `#[gpui_component]` 属性をつけ、`view!` マクロを返すだけで React っぽく書けます。

```rust
use gpui::{App, IntoElement, px};
use gpux::{gpui_component, use_state, view};

#[gpui_component]
fn Counter(cx: &mut App, initial_value: i32) -> impl IntoElement {
    // React ライクな状態管理！
    let (count, set_count) = use_state!(initial_value);

    view! {
        <div class="flex flex-col items-center justify-center w-full h-full bg-slate-100 gap-4">
            <div class="text-[32px] font-bold text-gray-800">
                { format!("Count: {}", count) }
            </div>
            <button
                class="bg-blue-500 hover:bg-blue-600 active:bg-blue-700 text-white px-[16px] py-[8px] rounded-md cursor-pointer"
                on_click={move |_event, _window, cx| set_count(count + 1, cx)}
            >
                "Increment"
            </button>
        </div>
    }
}
```

### 2. アニメーションとレスポンシブ

ウィンドウ幅に応じて色が変わったり、アニメーションし続ける要素も簡単に作れます。

```rust
view! {
    <div class="w-[200px] h-[200px] bg-red-500 sm:bg-green-500 md:bg-blue-500 animate-bounce rounded-full">
        "レスポンシブでアニメーションもします！"
    </div>
}
```

### 3. モーダルと Portal

モーダルなどを構築する際は、`<Portal>` を使って標準のレイアウトツリーの制約を無視できます。

```rust
view! {
    <Portal>
        <div class="absolute top-0 left-0 w-full h-full flex items-center justify-center bg-black/50">
            <div class="bg-white p-[24px] rounded-lg shadow-lg">
                "全ての要素の最前面に描画されます！"
            </div>
        </div>
    </Portal>
}
```

## デモアプリの起動

ワークスペース内にある `demo_app` を実行することで、実際の動作を確認できます。
- `use_state` を使ったカウンター
- `w-[...px]` 等の任意値や Tailwind カラーのパース
- `<Portal>` を使ったモーダルウィンドウ
- `use_future!` による非同期データのローディングと描画
- GPUI ネイティブアニメーション（Pulse や Bounce など）

```bash
cargo run -p demo_app
```

## サポートされている機能と制限事項

**サポートされているもの:**
`flex`, `absolute` 等のレイアウトクラス、`items-center`, `justify-between` 等の Flexbox/Grid クラス、`w-full` 等のサイジング、`p-4` や `gap-2` などの余白クラスは、**ほぼ全てそのまま動作します**。GPUI のスタイルメソッドは、元々 Tailwind のクラス名を `snake_case` にした名前で定義されているため、マクロが自動的に対応するメソッドへフォールバック変換します。

**現在未対応（GPUI の仕様による制約）:**
- **`transition` & `duration`**: GPUI には「状態Aから状態Bへの変化を自動的に補間（Tween）する」CSSエンジンのような仕組みがないため、即座にスタイルが切り替わります（現時点では無視されます）。
- **複雑な Transform**: 回転（`rotate`）や任意の倍率によるスケール変更など、GPUI 標準の View では行列変換を手動で行わないと対応できないような機能はサポートされていません。
- **フィルター**: 高度な `backdrop-blur` や、複雑な `box-shadow` のマッピングは完全には一致しない場合があります。

## License
MIT
