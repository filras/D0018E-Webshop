import { Link, Route, Routes } from "react-router";

import "./Admin.css"

import Users from "./Users";

type Props = {

};

export default function AdminPanel({}: Props) {
  return (
    <div className="admin-container">
      <div className="admin-sidebar">
        <ul className="admin-links">
          <Link to="/admin/users" className="admin-link">Users</Link>
          <Link to="/admin/items" className="admin-link">Items</Link>
          <Link to="/admin/orders" className="admin-link">Orders</Link>
        </ul>
      </div>
      <div className="admin-content">
        <Routes>
          <Route index element={<p>AdminPanel</p>} />
          <Route path="users" element={<Users />} />
          <Route path="items" element={<p>Items</p>} />
          <Route path="orders" element={<p>Orders</p>} />
        </Routes>
      </div>
    </div>
  )
}
