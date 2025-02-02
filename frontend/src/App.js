import React from "react";
import { BrowserRouter as Router, Routes, Route } from "react-router-dom";
import { AuthProvider } from "./context/AuthContext";
import Login from "./components/Login";
import Register from "./components/Register";
import Users from "./components/Users";
import LoggedInUsers from "./components/LoggedInUsers";

function App() {
  return (
    <AuthProvider>
      <Router>
        <Routes>
          <Route path="/login" element={<Login />} />
          <Route path="/register" element={<Register />} />
          <Route path="/users" element={<Users />} />
          <Route path="/logged-in-users" element={<LoggedInUsers />} />
        </Routes>
      </Router>
    </AuthProvider>
  );
}

export default App;
