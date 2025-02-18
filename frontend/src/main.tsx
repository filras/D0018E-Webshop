import { createRoot } from 'react-dom/client'
import './index.css'
import App from './App.tsx'
import Login from './Login.tsx';
import Register from "./Register.tsx"

import { BrowserRouter, Routes, Route } from "react-router";

createRoot(document.getElementById('root')!).render(
  <BrowserRouter>
    <Routes>
      <Route path="/" element={<App />} />
      <Route path="/login" element={<Login />} />
      <Route path='/register' element={<Register/>}/>
    </Routes>
  </BrowserRouter>
);
