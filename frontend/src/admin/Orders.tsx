import { useEffect, useState } from "react"
import { ADMIN_API_URL } from "../etc/api_url";
import { Link, useNavigate, useParams } from "react-router";
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
        &nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;
        <button onClick={() => fetchOrders()}>Refresh</button>
      </div>

      <table className="admin-table">
        <tr>
          <th>ID</th>
          <th>Firstname</th>
          <th>Surname</th>
          <th>Username</th>
          <th>Email</th>
          <th>Total</th>
          <th>Completed</th>
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
            <td>{order.payment_completed ? "Yes":"No"}</td>
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

  const navigate = useNavigate();
  const [orderData, setOrderData] = useState<OrderWithUserDataAndItems | null>(null);
  const [error, setError] = useState<string>("");

  async function fetchOrderData() {
    const orderDataResult = await fetch(ADMIN_API_URL + "/order?id=" + orderId);

    // Go back if no order data is found (indicating an invalid order id)
    if (!orderDataResult.ok) {
      navigate("/admin/orders");
    }

    const orderData = await orderDataResult.json() as OrderWithUserDataAndItems;
    setOrderData(orderData);
  }

  // Cancel a pending order, i.e. release items and stop waiting for payment
  async function cancelOrder() {
    const confirmed = confirm(`Are you sure you want to cancel order #${orderId}? This will release all order items back into the stock.`);

    if (confirmed) {
      const result = await fetch(ADMIN_API_URL + "/order/cancel?id=" + orderId, {
        method: "DELETE",
      });

      if (result.ok) {
        navigate("/admin/orders");
      } else {
        setError(await result.text());
      }
    }
  }
  
  // Remove a finished order
  async function removeOrder() {
    const confirmed = confirm(`Are you sure you want to remove order #${orderId}? This will remove all data about this order from the database.`);

    if (confirmed) {
      const result = await fetch(ADMIN_API_URL + "/order/remove?id=" + orderId, {
        method: "DELETE",
      });

      if (result.ok) {
        navigate("/admin/orders");
      } else {
        setError(await result.text());
      }
    }
  }

  // Fetch order data once at render
  useEffect(() => {fetchOrderData()}, []);

  return (
    <>
      <Link to="/admin/orders">&lt; Return to order list</Link>
      <br /><br />
      <h1 className="admin-title">Manage order #{orderId}</h1>
      { error && <p className="admin-error">Error: {error}</p> }

      { orderData?.payment_completed ? (
        <button className="admin-button" onClick={() => removeOrder()}>Remove order</button>
      ) : (
        <button className="admin-button" onClick={() => cancelOrder()}>Cancel order</button>
      ) }

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
