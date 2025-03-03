import { Link } from 'react-router';
import { AuthUser } from '../auth/ProtectedRoute';

import flygplan from "../assets/flygplan.png"
import "./Navigation.css"

interface Props {
  user: AuthUser | null;
  loadingUser: boolean;
  performLogout: () => void;
};

export function Navigation({ user, loadingUser, performLogout }: Props) {
  return (
    <nav className="navbar">
      <div className="navbar-left">
        <Link to="/" className="navbar-home-link">
          <img src={flygplan} className='logo' alt='flygplan'/>
          <h1 className="navbar-title">PlaneShop</h1>
        </Link>
      </div>
      <div className="navbar-right">
        <ul className="navbar-links">
          {user?.is_admin && (<Link to="/admin">Admin</Link>)}
          <Link to="/shoppingcart">ShoppingCart</Link>
          { !loadingUser && !user && ( <>
            <Link to="/login">Login</Link>
            <Link to="/register">Register</Link>
          </> ) }
        </ul>
        <p>{loadingUser ? "Loading user..." : user?.username} {user !== null && <button onClick={performLogout}>Log out</button>} </p>
      </div>
    </nav>
  )
}