export default function Users() {
  return (
    <>
      <h1 className="admin-title">Manage users</h1>
      <table className="admin-table">
        <tr>
          <th>ID</th>
          <th>Username</th>
          <th>Firstname</th>
          <th>Surname</th>
          <th>Email</th>
          <div className="admin-table-manage">
            Edit user
          </div>
        </tr>
      </table>
    </>
  )
}