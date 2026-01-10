// src/App.tsx

import { BrowserRouter, Routes, Route, Navigate, useLocation } from "react-router-dom";
import { QueryClient, QueryClientProvider } from "@tanstack/react-query";
import { ThemeProvider } from "@/components/theme-provider";
import { AppLayout } from "@/components/layouts/AppLayout";
import { AuthProvider, useAuth } from "@/components/auth-provider"; // さっき作ったやつ

// 各ページコンポーネント
import { DashboardPage } from "./routes/DashboardPage";
import { ProjectPage } from "./routes/ProjectsPage";
import { ProjectDetailPage } from "./routes/ProjectDetailPage";
import { JobDetailPage } from "./routes/JobDetailPage";
import { LoginPage } from "./routes/LoginPage"; // ★あとで作る
import type { JSX } from "react";

const queryClient = new QueryClient();

// ガード用コンポーネント: ログインしてなければログイン画面へ飛ばす
function RequireAuth({ children }: { children: JSX.Element }) {
    const { isAuthenticated, isLoading } = useAuth();
    const location = useLocation();

    if (isLoading) {
        return <div>Loading...</div>; // またはローディングスピナー
    }

    if (!isAuthenticated) {
        // ログイン画面へリダイレクト（元の場所をstateで渡しておくと親切）
        return <Navigate to="/login" state={{ from: location }} replace />;
    }

    return children;
}

function App() {
    return (
        <QueryClientProvider client={queryClient}>
            <ThemeProvider defaultTheme="dark" storageKey="vite-ui-theme">
                {/* ★ ここでAuthProviderで包む！ */}
                <AuthProvider>
                    <BrowserRouter>
                        <Routes>
                            {/* ▼▼▼ 公開ルート (ログイン画面など) ▼▼▼ */}
                            <Route path="/login" element={<LoginPage />} />

                            {/* ▼▼▼ 保護されたルート (ログイン必須) ▼▼▼ */}
                            <Route
                                element={
                                    <RequireAuth>
                                        <AppLayout />
                                    </RequireAuth>
                                }
                            >
                                <Route path="/" element={<DashboardPage />} />
                                <Route path="/projects" element={<ProjectPage />} />
                                <Route
                                    path="/projects/:projectId"
                                    element={<ProjectDetailPage />}
                                />
                                <Route
                                    path="/projects/:projectId/jobs/:jobId"
                                    element={<JobDetailPage />}
                                />
                            </Route>
                        </Routes>
                    </BrowserRouter>
                </AuthProvider>
            </ThemeProvider>
        </QueryClientProvider>
    );
}

export default App;
