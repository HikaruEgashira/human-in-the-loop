## Human-in-the-Loop MCP (Model Context Protocol) Server

AI AssistantがDiscordやSlackを通じて人間に質問し、回答を得ることができる。

### 動作確認

MCPサーバーとしての動作確認（JSONRPC over stdio）：

```bash
source .env && echo '{"jsonrpc":"2.0","id":1,"method":"initialize","params":{"protocolVersion":"2024-11-05","capabilities":{"tools":{}},"clientInfo":{"name":"test","version":"1.0"}}}' | cargo run -- slack

source .env && echo '{"jsonrpc":"2.0","id":2,"method":"tools/list","params":{}}' | cargo run -- slack

source .env && echo '{"jsonrpc":"2.0","id":3,"method":"tools/call","params":{"name":"ask_human","arguments":{"question":"テスト質問です"}}}' | cargo run -- slack
```

**重要**: 各テスト実行で新しいSlackスレッドが作成されます。応答は必ず最新のスレッド（メンション付きメッセージ）に返信してください。

### 環境変数設定

`.env`ファイルを使用（`source .env`で読み込み）：

## 開発時の注意点

- 認証トークンは環境変数で管理、コードに直接書かない
- 非同期処理でエラーハンドリングを適切に行う
- タイムアウト処理を考慮したテスト作成
- MCP protocol仕様に準拠したtool定義
- 実装が完了したら最後に必ず動作確認を行なってください。