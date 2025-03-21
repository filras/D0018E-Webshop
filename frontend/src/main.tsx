import { createRoot } from 'react-dom/client'
import './index.css'
import Homepage from './App.tsx'

// Products 
import ProductPage from './product/Product.tsx';

// Auth
import Login from './auth/Login.tsx';
import Register from "./auth/Register.tsx"

// Shoppingcart /checkout /complete
import ShoppingCart from "./ShoppingCart.tsx"
import Checkout from './checkout.tsx';
import Complete from "./complete.tsx";

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
        ...user,
        user_id: user.id,
        is_admin: user.role === "admin",
      });
    }
    setLoading(false);
  }
  
  // Set user from login/register
  const handleLogin = (user: User) => {
    setUser({
      ...user,
      user_id: user.id,
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
          <Route index element={<Homepage user={user} loadingUser={loading}  />} />
          <Route path="/item/:itemId" element={<ProductPage user={user} />} />
          <Route path="/login" element={
            <ProtectedRoute user={user} userLoading={loading} requireUnauthed>
              <Login user={user} setUser={handleLogin} />
            </ProtectedRoute>
          }/>
          <Route path="/register" element={
            <ProtectedRoute user={user} userLoading={loading} requireUnauthed>
              <Register user={user} setUser={handleLogin}/>
            </ProtectedRoute>
          }/>
          <Route path="/shoppingcart" element={<ShoppingCart/>}/>
          <Route path='/checkout' element={<Checkout/>}></Route>
          <Route path='/complete' element={<Complete/>}></Route>
          <Route path="/admin/*" element={
            <ProtectedRoute user={user} userLoading={loading} requireAdmin>
              <AdminPanel user_id={user?.user_id || 0 /* User will always exist inside ProtectedRoutes, so this is safe */} />
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
