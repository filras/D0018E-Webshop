import { Link } from 'react-router';
import { AuthUser } from '../auth/ProtectedRoute';

import "./Navigation.css"

interface Props {
  user: AuthUser | null;
  loadingUser: boolean;
  performLogout: () => void;
};

export function Navigation({ user, loadingUser, performLogout }: Props) {
  return (
    <div className="navbar">
      <div className="navbar-left">
        <h1 className="navbar-title"><Link to="/">PlaneShop</Link></h1>
      </div>
      <div className="navbar-right">
        <ul className="navbar-links">
          <Link to="/admin">Admin</Link>
          <Link to="/shoppingcart">ShoppingCart</Link>
          { !loadingUser && !user && ( <>
            <Link to="/login">Login</Link>
            <Link to="/register">Register</Link>
          </> ) }
        </ul>
        <p>{loadingUser ? "Loading user..." : user?.username} {user !== null && <button onClick={performLogout}>Log out</button>} </p>
      </div>
    </div>
  )
}