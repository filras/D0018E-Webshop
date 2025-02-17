import { Component } from "react";

type MyProps = {
  
};

type MyState = {
  username: string;
};

class Login extends Component<MyProps, MyState> {

  state: MyState = {
    username: "",
  };

  constructor(props: MyProps) {
    super(props);
    
    this.onUsernameInput = this.onUsernameInput.bind(this);
  }

  onUsernameInput(event: React.ChangeEvent<HTMLInputElement>) {
    // event.preventDefault();
    this.setState({
      username: event.target.value,
    });
  };

  render() {
    return (
      <div>
        <h1>Login</h1>
        <input type="text" name="Username" placeholder="Username" onChange={this.onUsernameInput} />
        <input type="text" name="Password" placeholder="Password" />
        <a href="/">Back to homepage</a>
        <p>{this.state.username}</p>
      </div>
    )
  }

}

export default Login
