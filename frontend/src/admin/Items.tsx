import { FormEventHandler, useEffect, useState } from "react"
import { ADMIN_API_URL, API_URL } from "../etc/api_url";

interface Props {

}

export default function Items({  }: Props) {
  const [items, setItems] = useState<Array<Item>>([]);
  const [page, setPage] = useState<number>(1);

  async function fetchItems() {
    const itemsResult = await fetch(API_URL + "/items?page=" + page);
    const items = await itemsResult.json();
    setItems(items);
  }

  // Sends a request to the backend to update the item, then updates the frontend if it succeeds
  const editItem = async (item: Item): Promise<boolean> => {
    // Spread updated values into an UpdateItem object
    const updateItem: UpdateItem = {...item};
    const result = await fetch(ADMIN_API_URL + "/items?id=" + item.id, {
      method: "PUT",
      body: JSON.stringify(updateItem),
      headers: { "Content-Type": "application/json" }
    });

    if (result.ok) {
      // Update object in items with new values
      setItems(items.map(old_item => old_item.id === item.id ? item : old_item));
      return true;
    } else return false;
  }

  // Attempts to delete the user in the backend. If the deletion is successful, updates the users array
  const deleteItem = async (item_id: number) => {
    const result = await fetch(ADMIN_API_URL + "/items?id=" + item_id, { method: "DELETE" });
    if (result.ok) {
      setItems(items.filter(item => item.id !== item_id));
    }
  }

  const submitNewItem: FormEventHandler = async (e) => {
    e.preventDefault();
    const data = Object.fromEntries(new FormData(e.target as HTMLFormElement));
    const newItem: NewItem = {
      title: data.title.toString(),
      description: data.description.toString(),
      in_stock: Number(data.in_stock),
      price: Number(data.price),
      discounted_price: Number(data.discounted_price),
    }
    
    const result = await fetch(ADMIN_API_URL + "/items", {
      method: "POST",
      body: JSON.stringify(newItem),
      headers: { "Content-Type": "application/json" }
    });

    if (result.ok) {
      // Refetch items to include the new item
      fetchItems();
    }
  }

  // Fetch users once or on new page
  useEffect(() => {fetchItems()}, [page]);

  return (
    <>
      <h1 className="admin-title">Manage items</h1> 

      <div className="admin-section">
        <form onSubmit={submitNewItem} className="admin-create">
          <h3>Create new item</h3>
          <input type="text" name="title" placeholder="Title" required />
          <input type="text" name="description" placeholder="Description" />
          <input type="number" name="in_stock" placeholder="Amount in stock" required />
          <input type="number" name="price" placeholder="Price" required />
          <input type="number" name="discounted_price" placeholder="Discounted price" />
          <button type="submit">Submit</button>
        </form>
      </div>

      <div className="admin-section">
        <button onClick={() => setPage(Math.max(page - 1, 1))}>Previous page</button>
        <p>Current page: {page}</p>
        <button onClick={() => setPage(page + 1)}>Next page</button>
      </div>

      <table className="admin-table">
        <tr>
          <th>ID</th>
          <th>Title</th>
          <th>Description</th>
          <th>Rating</th>
          <th>In stock</th>
          <th>Price</th>
          <th>Discounted price</th>
          <th className="admin-table-manage">
            Actions
          </th>
        </tr>
        { items.map(item => <EditItemRow item={item} editItem={editItem} deleteItem={deleteItem} />) }
      </table>
    </>
  )
}

// Custom row component that can be edited and will use the editItem prop function to send back the updated item
interface RowProps {
  item: Item;
  editItem: (item: Item) => Promise<boolean>;
  deleteItem: (item_id: number) => void;
}
function EditItemRow({ item, editItem, deleteItem }: RowProps) {
  const [editActive, setEditActive] = useState<boolean>(false);
  // Clone item to allow "rollback" to previous data on cancel
  const itemData: Item = structuredClone(item);

  // If editActive, turn the data into text inputs and change the buttons to Cancel/Save
  if (editActive) {
    return (
      <tr key={item.id} className="admin-table-editing">
        <td>{item.id}</td>
        <td> <input type="text" onChange={(e) => itemData.title = e.target.value} defaultValue={item.title} /> </td>
        <td> <input type="text" onChange={(e) => itemData.description = e.target.value} defaultValue={item.description} /> </td>
        <td>{item.average_rating}</td>
        <td> <input type="number" onChange={(e) => itemData.in_stock = Number(e.target.value)} defaultValue={item.in_stock} /> </td>
        <td> <input type="number" onChange={(e) => itemData.price = Number(e.target.value)} defaultValue={item.price} /> </td>
        <td> <input type="number" onChange={(e) => itemData.discounted_price = Number(e.target.value)} defaultValue={item.discounted_price} /> </td>

        <td className="admin-table-manage">
          <button className="admin-table-button" onClick={() => editItem(itemData).then(success => success && setEditActive(false)) }>Save</button>
          <button className="admin-table-button" onClick={() => setEditActive(false)}>Cancel</button>
        </td>
      </tr>
    )
  } else {
    return (
      <tr key={item.id}>
        <td>{item.id}</td>
        <td>{item.title}</td>
        <td>{item.description}</td>
        <td>{item.average_rating}</td>
        <td>{item.in_stock}</td>
        <td>{item.price}</td>
        <td>{item.discounted_price ? item.discounted_price : ""}</td>
        <td className="admin-table-manage">
          <button className="admin-table-button" onClick={() => setEditActive(true)}>Edit</button>
          <button className="admin-table-button" onClick={() => confirm("Delete item " + item.title + "?") && deleteItem(item.id)}>Delete</button>
        </td>
      </tr>
    )
  }

}
