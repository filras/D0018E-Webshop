import { Component } from "react";
import flygplan from "./assets/flygplan.png"

type MyProps = {
  
};

type MyState = {
  username: string;
  userpass: string;
};

class Login extends Component<MyProps, MyState> {

  handleClick = (_event: MouseEvent): void => {
    alert('Button was clicked!');
};

  state: MyState = {
    username: "",
    userpass: "",
  };

  constructor(props: MyProps) {
    super(props);
    this.onPasswordInput = this.onPasswordInput.bind(this);
    this.onUsernameInput = this.onUsernameInput.bind(this);

  }

  onUsernameInput(event: React.ChangeEvent<HTMLInputElement>) {
    
    this.setState({
      username: event.target.value,
    });
  };

  onPasswordInput(event: React.ChangeEvent<HTMLInputElement>){
    this.setState({
      userpass: event.target.value,
    })
  }

  handleSubmit(e: any){
    e.preventDefault();

    // Read the form data
    const form = e.target;
    const formData = new FormData(form);
    
    // Pass formdata as fetch body 
    const formJson = Object.fromEntries(formData.entries());
    fetch('/auth/login', { headers: {"Content-Type": "application/json"}, method: "post", body: JSON.stringify(formJson) });

    
  }

  render() {
    return (
      <form method="post" onSubmit={this.handleSubmit}>
      <div>
        <h1>Login</h1>
        <input type="text" name="username" placeholder="username" onChange={this.onUsernameInput} />
        <input type="text" name="password" placeholder="password" onChange={this.onPasswordInput}/>
        <div>
          <a href="/">
          <img src={flygplan} className='logo' alt='flygplan'/>
          </a>
        </div>
        <div>
          <a href="/register">Don't have a account? Register here!</a>  
        </div>       
      </div>
      <br></br><br></br><br></br><br></br><br></br><br></br><br></br><br></br><br></br><br></br><br></br>  
      <button name="Submit" type="submit">Submit form</button>
      </form>
    )
  }

}

export default Login
