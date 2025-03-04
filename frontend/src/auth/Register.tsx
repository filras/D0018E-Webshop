import { useState } from "react";
import flygplan from "../assets/flygplan.png"
import { API_URL } from "../etc/api_url";
import { AuthUser } from "./ProtectedRoute";
import { useNavigate } from "react-router";

interface Props {
  user: AuthUser | null;
  setUser: (user: User) => void;
}

export default function Register({ setUser }: Props) {
  const navigate = useNavigate();
  const [error, setError] = useState("");

  async function handleSubmit(e: any) {
    e.preventDefault();

    // Read the form data
    const form = e.target;
    const formData = new FormData(form);

    // Create a post request to API to create an account
    const formJson = Object.fromEntries(formData.entries());
    const registerResult = await fetch(API_URL + "/account", {
      method: "POST",
      body: JSON.stringify(formJson),
      headers: new Headers({"content-type": "application/json"})
    })

    // If account creation succeeded, take the response user json and set user
    if (!registerResult.ok) {
      // Alert with error message
      setError(await registerResult.text())
    } else {
      const user: User = await registerResult.json();
      setUser(user);
      navigate("/");
    }
  }

  return (
    <div>
      <h1>Register Account</h1>
      { error && (<p>{error}</p>) }
      <form method="post" onSubmit={handleSubmit}> 
        <input type="text" name="email" placeholder="email" />
        <br />
        <input type="password" name="password" placeholder="password" />
        <br />
        <input type="text" name="firstname" placeholder="firstname" />
        <br />
        <input type="text" name="surname" placeholder="surname" />
        <br />
        <button type="submit">Create account</button>

        <br /><br />

        <a href="/">
          <img src={flygplan} className='logo' alt='flygplan'/>
        </a>
        
      </form>
      <a href="/login">Already have and account? Log in instead</a>  
    </div>
  )
}
