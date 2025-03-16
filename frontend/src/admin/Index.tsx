import { Link, Route, Routes } from "react-router";

import "./Admin.css"

import Users from "./Users";
import Items from "./Items";
import { Orders, ManageOrder } from "./Orders";

type Props = {
  user_id: number
};

export default function AdminPanel({ user_id }: Props) {
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
          <Route index element={<h3>Select a function on the panel to the left to manage the shop</h3>} />
          <Route path="users" element={<Users user_id={user_id} />} />
          <Route path="items" element={<Items />} />
          <Route path="orders">
            <Route index element={<Orders />} />
            <Route path=":orderId" element={<ManageOrder />} />
          </Route>
        </Routes>
      </div>
    </div>
  )
}
