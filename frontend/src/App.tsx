import { BrowserRouter, Route, Routes } from "react-router-dom";
import { DashboardLayout } from "./components/ui/layout/DashboardLayout";
import { Home } from "./pages/Home";
import { Signup } from "./pages/Singup";
import { Login } from "./pages/Login";
import { ProtectedRoute } from "./components/auth/ProtectedRoute";

function App() {
  return (
    <BrowserRouter>
      <Routes>
        {/* 保護ルート */}
        <Route element={<ProtectedRoute />}>
          <Route element={<DashboardLayout />}>
            <Route path="/" element={<Home />} />
            <Route path="/projects" element={<p>This is a projects page.</p>} />
            <Route path="/settings" element={<p>This is a setting page.</p>} />
          </Route>
        </Route>

        {/* 公開ルート */}
        <Route path="/signup" element={<Signup />} />
        <Route path="/login" element={<Login />} />
      </Routes>
    </BrowserRouter>
  );
}

export default App;
