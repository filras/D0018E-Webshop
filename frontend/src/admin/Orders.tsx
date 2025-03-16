import { useEffect, useState } from "react"
import { ADMIN_API_URL } from "../etc/api_url";
import { Link, useParams } from "react-router";
import { CURRENCY } from "../etc/const";

interface Props {
  
}

export function Orders({ }: Props) {
  const [orders, setOrders] = useState<Array<OrderWithUserData>>([]);
  const [page, setPage] = useState<number>(1);

  async function fetchOrders() {
    const usersResult = await fetch(ADMIN_API_URL + "/orders?page=" + page);
    const users = await usersResult.json();
    setOrders(users);
  }

  // Fetch orders once
  useEffect(() => {fetchOrders()}, [page]);

  return (
    <>
      <h1 className="admin-title">Manage orders</h1>

      <div className="admin-section">
        <button onClick={() => setPage(Math.max(page - 1, 1))}>Previous page</button>
        <p>Current page: {page}</p>
        <button onClick={() => setPage(page + 1)}>Next page</button>
      </div>

      <table className="admin-table">
        <tr>
          <th>ID</th>
          <th>Firstname</th>
          <th>Surname</th>
          <th>Username</th>
          <th>Email</th>
          <th>Total</th>
          <th className="admin-table-manage">
            Actions
          </th>
        </tr>
        { orders.map(order => (
          <tr key={order.id}>
            <td>{order.id}</td>
            <td>{order.firstname}</td>
            <td>{order.surname}</td>
            <td>{order.username}</td>
            <td>{order.email}</td>
            <td>{order.total} {CURRENCY}</td>
            <td className="admin-table-manage">
              <Link className="admin-table-button" to={`/admin/orders/${order.id}`}>Manage</Link>
            </td>
          </tr>
        )) }
      </table>
    </>
  )
}

export function ManageOrder() {
  const orderId = Number(useParams().orderId);

  const [orderData, setOrderData] = useState<OrderWithUserDataAndItems | null>(null);

  async function fetchOrders() {
    const orderDataResult = await fetch(ADMIN_API_URL + "/order_data?id=" + orderId);
    const orderData = await orderDataResult.json() as OrderWithUserDataAndItems;
    setOrderData(orderData);
  }

  // Fetch orders once
  useEffect(() => {fetchOrders()}, []);

  return (
    <>
      <Link to="/admin/orders">Return to order list</Link>
      <h1>Manage order #{orderId}</h1>
      <div className="admin-order-grid">
        <div>
          <h2>Order items</h2>
          <table className="admin-table">
            <tr>
              <th>Name</th>
              <th>Price</th>
              <th>Amount</th>
              <th>Total</th>
            </tr>
            { orderData?.items.map(orderItem => (
              <tr>
                <td>{orderItem.name}</td>
                <td>{orderItem.discounted_price ? orderItem.discounted_price : orderItem.price} {CURRENCY}</td>
                <td>{orderItem.amount}</td>
                <td>{orderItem.total} {CURRENCY}</td>
              </tr>
            )) }
          </table>
        </div>
        <div>
          <h2>Order & shipping information</h2>
          <table className="admin-table">
            { orderData && Object.entries(orderData)
              .filter(pair => !["id", "items"].includes(pair[0])) // Ignore id and items fields here
              .map(entry => (
              <tr>
                <td>{entry[0]}</td>
                <td>{entry[1] !== null && String(entry[1])}</td>
              </tr>
            )) }
          </table>
        </div>
      </div>
    </>
  );
}
