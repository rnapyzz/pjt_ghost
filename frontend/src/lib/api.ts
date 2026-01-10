import axios from "axios";

// 共通のベース情報を設定したapiというクライアントを作っておく
export const api = axios.create({
    baseURL: "http://127.0.0.1:3000",
});

// リクエスト・インターセプター: 送信前の割り込み処理
// api.get()やapi.post()が呼ばれたときにサーバーに飛ぶ直前に実行される
api.interceptors.request.use((config) => {
    // LocalStorageからトークンを取得
    const token = localStorage.getItem("token");

    // トークンがあった場合Authorization ヘッダーにセットする
    if (token) {
        config.headers.Authorization = `Bearer ${token}`;
    }

    return config;
});
