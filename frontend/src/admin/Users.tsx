import { useEffect, useState } from "react"
import { ADMIN_API_URL } from "../etc/api_url";

interface Props {
  user_id: number;
}

export default function Users({ user_id }: Props) {
  const [users, setUsers] = useState<Array<User>>([]);

  async function fetchUsers() {
    const usersResult = await fetch(ADMIN_API_URL + "/users");
    const users = await usersResult.json();
    setUsers(users);
  }

  // Sends a request to the backend to update the user, then updates the frontend if it succeeds
  const editUser = async (user: User): Promise<boolean> => {
    // Spread updated values into an UpdateUser object
    const updateUser: UpdateUser = {...user};
    const result = await fetch(ADMIN_API_URL + "/users?id=" + user.id, {
      method: "PUT",
      body: JSON.stringify(updateUser),
      headers: { "Content-Type": "application/json" }
    });

    if (result.ok) {
      // Update object in users with new values
      setUsers(users.map(old_user => old_user.id === user.id ? user : old_user));
      return true;
    } else return false;
  }

  // Attempts to delete the user in the backend. If the deletion is successful, updates the users array
  const deleteUser = async (user_id: number) => {
    const result = await fetch(ADMIN_API_URL + "/users?id=" + user_id, { method: "DELETE" });
    if (result.ok) {
      setUsers(users.filter(user => user.id !== user_id));
    }
  }

  // Fetch users once
  useEffect(() => {fetchUsers()}, []);

  return (
    <>
      <h1 className="admin-title">Manage users</h1>
      <table className="admin-table">
        <tr>
          <th>ID</th>
          <th>Role</th>
          <th>Username</th>
          <th>Firstname</th>
          <th>Surname</th>
          <th>Email</th>
          <th className="admin-table-manage">
            Actions
          </th>
        </tr>
        { users.map(user => <EditUserRow user={user} own_id={user_id} editUser={editUser} deleteUser={deleteUser} />) }
      </table>
    </>
  )
}

// Custom row component that can be edited and will use the editUser prop function to send back the updated user
interface RowProps {
  user: User;
  own_id: number; // The ID of the admin user (to prevent self deletes)
  editUser: (user: User) => Promise<boolean>;
  deleteUser: (user_id: number) => void;
}
function EditUserRow({ user, own_id, editUser, deleteUser }: RowProps) {
  const [editActive, setEditActive] = useState<boolean>(false);
  const userData = user;

  // If editActive, turn the data into text inputs and change the buttons to Cancel/Save
  if (editActive) {
    return (
      <tr key={user.id} className="admin-table-editing">
        <td>{user.id}</td>
        <td>{user.role}</td>
        <td> <input type="text" onChange={(e) => userData.username = e.target.value} defaultValue={user.username} /> </td>
        <td> <input type="text" onChange={(e) => userData.firstname = e.target.value} defaultValue={user.firstname} /> </td>
        <td> <input type="text" onChange={(e) => userData.surname = e.target.value} defaultValue={user.surname} /> </td>
        <td> <input type="text" onChange={(e) => userData.email = e.target.value} defaultValue={user.email} /> </td>
        <td className="admin-table-manage">
          <button className="admin-table-button" onClick={() => editUser(userData).then(success => success && setEditActive(false)) }>Save</button>
          <button className="admin-table-button" onClick={() => setEditActive(false)}>Cancel</button>
        </td>
      </tr>
    )
  } else {
    return (
      <tr key={user.id}>
        <td>{user.id}</td>
        <td>{user.role}</td>
        <td>{user.username}</td>
        <td>{user.firstname}</td>
        <td>{user.surname}</td>
        <td>{user.email}</td>
        <td className="admin-table-manage">
          <button className="admin-table-button" onClick={() => setEditActive(true)}>Edit</button>
          {user.id !== own_id && <button className="admin-table-button" onClick={() => confirm("Delete user " + user.username + "?") && deleteUser(user.id)}>Delete</button>}
        </td>
      </tr>
    )
  }

}
