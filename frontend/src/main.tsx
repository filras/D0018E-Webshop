import { createRoot } from 'react-dom/client'
import './index.css'
import Homepage from './App.tsx'

// Auth
import Login from './auth/Login.tsx';
import Register from "./auth/Register.tsx"

import ShoppingCart from "./ShoppingCart.tsx"

// Admin panel
import AdminPanel from "./admin/Index.tsx";

import { BrowserRouter, Routes, Route } from "react-router";
import { useEffect, useState } from 'react';
import { AuthUser, ProtectedRoute } from './auth/ProtectedRoute.tsx';
import { API_URL, BASE_URL } from './etc/api_url.ts';
import Navigation from './navbar';

// Create app component with auth hooks
const  App = () => {
  const [user, setUser] = useState<AuthUser | null>(null);
  const [loading, setLoading] = useState<boolean>(true);

  const loadUser = async () => {
    const userRequest = await fetch(API_URL + "/account");
    if (userRequest.ok) {
      const user: User = await userRequest.json();
      setUser({
        username: user.username,
        firstname: user.firstname,
        is_admin: user.role === "admin",
      });
    }
    setLoading(false);
  }

  // Set user from login/register
  const handleLogin = (user: User) => {
    setUser({
      username: user.username,
      firstname: user.firstname,
      is_admin: user.role === "admin",
    });
  }

  const performLogout = async () => {
    await fetch(BASE_URL + "/auth/logout");
    setUser(null);
  }
  
  // Load user once at start of session
  useEffect(() => {loadUser();}, [])

  return (
    <>
      <Navigation user={user} loadingUser={loading} performLogout={performLogout} />
      <div id="page-content">
        <Routes>
          <Route path="/" element={<Homepage />} />
          <Route path="/login" element={
            <ProtectedRoute user={user} requireUnauthed>
              <Login user={user} setUser={handleLogin} />
            </ProtectedRoute>
          }/>
          <Route path="/register" element={
            <ProtectedRoute user={user} requireUnauthed>
              <Register user={user} setUser={handleLogin}/>
            </ProtectedRoute>
          }/>
          <Route path="/shoppingcart" element={<ShoppingCart/>}/>
          <Route path="/admin/*" element={
            <ProtectedRoute user={user} requireAdmin>
              <AdminPanel />
            </ProtectedRoute>
          }>
            <Route path="users" />
          </Route>
        </Routes>
      </div>
    </>
  )
}

// Render app
createRoot(document.getElementById('root')!).render(
  <BrowserRouter>
    <App />
  </BrowserRouter>
);
