import { useState } from "react";
import flygplan from "../assets/flygplan.png"
import { BASE_URL } from "../etc/api_url";
import { AuthUser } from "./ProtectedRoute";
import { useNavigate } from "react-router";

interface Props {
  user: AuthUser | null;
  setUser: (user: User) => void;
};

export default function Login({ user, setUser }: Props) {
  const navigate = useNavigate();
  const [error, setError] = useState("");

  // console.log(user);
  if (user !== null) {
    navigate("/")
  }

  async function handleSubmit(e: any) {
    // By default, the browser will send the form data to the current URL,
    // and refresh the page. You can override that behavior by calling below
    e.preventDefault();

    // Read the form data
    const form = e.target;
    const formData = new FormData(form);

    // Pass formdata as fetch body 
    const formJson = Object.fromEntries(formData.entries());
    const result = await fetch(BASE_URL + "/auth/login", { headers: { "Content-Type": "application/json" }, method: "post", body: JSON.stringify(formJson) });
    
    if (!result.ok) {
      // Alert with error message
      setError(await result.text())
    } else {
      const user: User = await result.json();
      setUser(user);
      navigate("/");
    }
  }

  return (
    <div>
      <form method="post" onSubmit={handleSubmit}>
        <div>
          <h1>Login</h1>
          { error && (<p>{error}</p>) }
          <input type="text" name="username" placeholder="Username" />
          <br />
          <input type="password" name="password" placeholder="Password" />
          <br />
          <button name="Submit" type="submit">Login</button>
          <div>
            <a href="/register">Don't have a account? Register here!</a>
          </div>
        </div>
      </form>

      <br />
      <div>
        <a href="/">
          <img src={flygplan} className='logo' alt='flygplan' />
        </a>
      </div>
    </div>
  )
}
