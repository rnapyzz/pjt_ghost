// src/routes/LoginPage.tsx

import { useState } from "react";
import { useNavigate, useLocation } from "react-router-dom";
import { useAuth } from "@/components/auth-provider"; // さっき作ったやつ
import { api } from "@/lib/api";

export const LoginPage = () => {
    const navigate = useNavigate();
    const location = useLocation();
    const { login } = useAuth(); // AuthContextからlogin関数をもらう

    // フォームの入力値管理
    const [email, setEmail] = useState("");
    const [password, setPassword] = useState("");
    const [error, setError] = useState("");

    // ログイン後に戻る場所（指定がなければダッシュボードへ）
    // ※ RequireAuthでリダイレクトされた場合、state.fromに元の場所が入っている
    const from = location.state?.from?.pathname || "/";

    const handleSubmit = async (e: React.FormEvent) => {
        e.preventDefault();
        setError("");

        try {
            // 1. バックエンドにログインリクエスト
            const response = await api.post("/login", {
                email,
                password,
            });

            // 2. トークンを取得 (Axumのレスポンス形式に合わせて調整)
            // { token: "..." } という形式で返ってくると想定
            const { token } = response.data;

            // 3. アプリ全体に「ログインしたよ！」と伝える
            login(token);

            // 4. 画面遷移
            navigate(from, { replace: true });

        } catch (err) {
            console.error(err);
            setError("ログインに失敗しました。メールアドレスかパスワードを確認してください。");
        }
    };

    return (
        <div className="flex items-center justify-center min-h-screen bg-background">
            <div className="w-full max-w-md p-8 space-y-6 bg-card rounded-xl shadow-lg border border-border">
                <h1 className="text-2xl font-bold text-center text-foreground">
                    ログイン
                </h1>

                {error && (
                    <div className="p-3 text-sm text-red-500 bg-red-100/10 border border-red-500/20 rounded-md">
                        {error}
                    </div>
                )}

                <form onSubmit={handleSubmit} className="space-y-4">
                    <div className="space-y-2">
                        <label className="text-sm font-medium text-foreground">
                            メールアドレス
                        </label>
                        <input
                            type="email"
                            className="flex h-10 w-full rounded-md border border-input bg-background px-3 py-2 text-sm ring-offset-background placeholder:text-muted-foreground focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring"
                            placeholder="user@example.com"
                            value={email}
                            onChange={(e) => setEmail(e.target.value)}
                            required
                        />
                    </div>

                    <div className="space-y-2">
                        <label className="text-sm font-medium text-foreground">
                            パスワード
                        </label>
                        <input
                            type="password"
                            className="flex h-10 w-full rounded-md border border-input bg-background px-3 py-2 text-sm ring-offset-background placeholder:text-muted-foreground focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring"
                            value={password}
                            onChange={(e) => setPassword(e.target.value)}
                            required
                        />
                    </div>

                    <button
                        type="submit"
                        className="w-full h-10 px-4 py-2 text-sm font-medium text-primary-foreground bg-primary rounded-md hover:bg-primary/90 transition-colors"
                    >
                        ログインする
                    </button>
                </form>

                {/* 開発用ヒント：あとで消す */}
                <div className="text-xs text-muted-foreground mt-4">
                    <p>※テスト用アカウントを持っていますか？</p>
                    <p>なければDBを見て確認するか、Rust側でテストデータを入れてください。</p>
                </div>
            </div>
        </div>
    );
};
