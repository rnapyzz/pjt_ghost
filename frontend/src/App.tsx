import { BrowserRouter, Route, Routes } from "react-router-dom";
import { DashboardLayout } from "./components/ui/layout/DashboardLayout";
import { Home } from "./pages/Home";
import { Signup } from "./pages/Singup";

function App() {
  return (
    <BrowserRouter>
      <Routes>
        <Route element={<DashboardLayout />}>
          <Route path="/" element={<Home />} />
          <Route path="/projects" element={<p>This is a projects page.</p>} />
          <Route path="/settings" element={<p>This is a setting page.</p>} />
        </Route>

        <Route path="/signup" element={<Signup />} />
        <Route path="/login" element={<p>Login</p>} />
      </Routes>
    </BrowserRouter>
  );
}

export default App;
