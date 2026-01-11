// src/routes/SignupPage.tsx

import { useState } from "react";
import { Link, useNavigate } from "react-router-dom";
import { api } from "@/lib/api";

export const SignupPage = () => {
  const navigate = useNavigate();

  // フォームの状態管理
  const [name, setName] = useState("");
  const [email, setEmail] = useState("");
  const [password, setPassword] = useState("");
  const [error, setError] = useState("");

  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault();
    setError("");

    try {
      // 1. サインアップAPIを叩く
      // バックエンドの仕様に合わせて payload を送る
      await api.post("/signup", {
        name,
        email,
        password,
      });

      // 2. 成功したらログイン画面へ遷移
      // ユーザーへのフィードバックとしてalertを出してもいいですが、
      // 本番ではトースト通知などが望ましいです。今回はシンプルに。
      alert("アカウント作成に成功しました！ログインしてください。");
      navigate("/login");
    } catch (err) {
      console.error(err);
      setError(
        "アカウント作成に失敗しました。すでに登録されているメールアドレスかもしれません。"
      );
    }
  };

  return (
    <div className="flex items-center justify-center min-h-screen bg-background">
      <div className="w-full max-w-md p-8 space-y-6 bg-card rounded-xl shadow-lg border border-border">
        <div className="text-center space-y-2">
          <h1 className="text-2xl font-bold text-foreground">
            Ghost; に参加する
          </h1>
          <p className="text-sm text-muted-foreground">
            新しいアカウントを作成して、プロジェクト管理を始めましょう
          </p>
        </div>

        {error && (
          <div className="p-3 text-sm text-red-500 bg-red-100/10 border border-red-500/20 rounded-md">
            {error}
          </div>
        )}

        <form onSubmit={handleSubmit} className="space-y-4">
          <div className="space-y-2">
            <label className="text-sm font-medium text-foreground">
              ユーザー名
            </label>
            <input
              type="text"
              className="flex h-10 w-full rounded-md border border-input bg-background px-3 py-2 text-sm ring-offset-background placeholder:text-muted-foreground focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring"
              placeholder="Your Name"
              value={name}
              onChange={(e) => setName(e.target.value)}
              required
            />
          </div>

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
            アカウント作成
          </button>
        </form>

        <div className="text-center text-sm">
          <span className="text-muted-foreground">
            すでにアカウントをお持ちですか？{" "}
          </span>
          <Link
            to="/login"
            className="text-primary hover:underline font-medium"
          >
            ログイン
          </Link>
        </div>
      </div>
    </div>
  );
};
