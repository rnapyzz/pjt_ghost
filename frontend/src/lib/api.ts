import axios from "axios";

// 共通のベース情報を設定したapiというクライアントを作っておく
export const api = axios.create({
  baseURL: "http://localhost:3000",
});

// 開発用の固定ユーザーID
// TODO: ログイン機能実装後に削除する
const DEV_USER_ID = "79b111bb-e602-490b-ad0b-a86d8331ff5f";

// リクエスト・インターセプター: 送信前の割り込み処理
// api.get()やapi.post()が呼ばれたときにサーバーに飛ぶ直前に実行される
api.interceptors.request.use((config) => {
  if (DEV_USER_ID) {
    config.headers["x-user-id"] = DEV_USER_ID;
  }
  return config;
});
