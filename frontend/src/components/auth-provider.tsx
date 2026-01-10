import { createContext, useContext, useState, useEffect, type ReactNode } from "react";

// 認証コンテキストの型定義
type AuthContextType = {
    isAuthenticated: boolean;
    login: (token: string) => void;
    logout: () => void;
    isLoading: boolean; // 起動時のチェック中かどうか
};

const AuthContext = createContext<AuthContextType | undefined>(undefined);

export function AuthProvider({ children }: { children: ReactNode }) {
    const [isAuthenticated, setIsAuthenticated] = useState(false);
    const [isLoading, setIsLoading] = useState(true);

    // アプリ起動時に一度だけ実行：すでにトークン持ってる？
    useEffect(() => {
        const token = localStorage.getItem("token");
        if (token) {
            setIsAuthenticated(true);
        }
        setIsLoading(false);
    }, []);

    // ログイン処理: トークンを受け取って保存
    const login = (token: string) => {
        localStorage.setItem("token", token);
        setIsAuthenticated(true);
    };

    // ログアウト処理: トークンを破棄
    const logout = () => {
        localStorage.removeItem("token");
        setIsAuthenticated(false);
    };

    return (
        <AuthContext.Provider value={{ isAuthenticated, login, logout, isLoading }}>
            {children}
        </AuthContext.Provider>
    );
}

// 便利なカスタムフック
export const useAuth = () => {
    const context = useContext(AuthContext);
    if (context === undefined) {
        throw new Error("useAuth must be used within an AuthProvider");
    }
    return context;
};
