import { Link } from 'react-router';
import { AuthUser } from '../auth/ProtectedRoute';
import cart from "../assets/shoppingcart.png"
import logIn from "../assets/LogInn.png"
import flygplan from "../assets/flygplan.png"
import reg from "../assets/TKL.png"
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
          <img src={flygplan} className='navbar-logo' alt='flygplan'/>
          <h1 className="navbar-title">PlaneShop</h1>
        </Link>
      </div>
      <div className="navbar-right">
        <ul className="navbar-links">
          {user?.is_admin && (<Link to="/admin">Admin</Link>)}
          <Link to="/shoppingcart">
          <img src={cart} className='navbar-pics' alt='shoppingcart' />
          </Link>
          { !loadingUser && !user && ( <>
            <Link to="/login">
            <img src={logIn} className='navbar-pics' alt='logIn'>
            </img>
            </Link>
            <Link to="/register">
            <img src={reg} className='navbar-pics' alt='reg'>
            </img>
            </Link>
          </> ) }
        </ul>
        <p>{loadingUser ? "Loading user..." : user?.username} {user !== null && <button onClick={performLogout}>Log out</button>} </p>
      </div>
    </nav>
  )
}