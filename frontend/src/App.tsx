import { BrowserRouter, Route, Routes } from "react-router-dom";
import { DashboardLayout } from "./components/ui/layout/DashboardLayout";
import { Home } from "./pages/Home";
import { Signup } from "./pages/Singup";
import { Login } from "./pages/Login";
import { ProtectedRoute } from "./features/auth/components/ProtectedRoute";
import { Themes } from "./pages/Themes";
import { Services } from "./pages/Services";

function App() {
  return (
    <BrowserRouter>
      <Routes>
        {/* 保護ルート */}
        <Route element={<ProtectedRoute />}>
          <Route element={<DashboardLayout />}>
            <Route path="/" element={<Home />} />
            <Route path="/themes" element={<Themes />} />
            <Route path="/services" element={<Services />} />
            <Route path="/projects" element={<p>This is a projects page.</p>} />
            <Route path="/settings" element={<></>} />
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
