import { Component } from "react";
import flygplan from "./assets/flygplan.png"
type MyProps = {
  
};

type MyState = {
  username: string;
  userpass: string;
  firstname: string;
  lastname: string;
  email: string;
  role: string;
  address?: string;
  zip?: string;
  co?: string;
  country?: string;
};

class Register extends Component<MyProps, MyState> {

  state: MyState = {
    username: "",
    userpass: "",
    firstname:"",
    lastname: "",
    email: "",
    role: "",
    address: "",
    zip: "",
    co: "",
    country: "",
  };

  constructor(props: MyProps) {
    super(props);
    this.onPasswordInput = this.onPasswordInput.bind(this);
    this.onUsernameInput = this.onUsernameInput.bind(this);
    this.onFirstnameInput = this.onFirstnameInput.bind(this);
    this.onLastnameInput = this.onLastnameInput.bind(this);
    this.onEmailInput = this.onEmailInput.bind(this);
    this.onZipCodeInput = this.onZipCodeInput.bind(this);
    this.onCoInput = this.onCoInput.bind(this);
    this.onCountryInput = this.onCountryInput.bind(this);
    this.onAddressInput = this.onAddressInput.bind(this);
    this.onRoleInput = this.onRoleInput.bind(this);
    
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

  onFirstnameInput(event: React.ChangeEvent<HTMLInputElement>){
    this.setState({
      firstname: event.target.value,
    })
  }

  onLastnameInput(event: React.ChangeEvent<HTMLInputElement>){
    this.setState({
      lastname: event.target.value,
    })
  }

  onEmailInput(event: React.ChangeEvent<HTMLInputElement>){
    this.setState({
      email: event.target.value,
    })
  }

  onRoleInput(event: React.ChangeEvent<HTMLInputElement>){
    this.setState({
      role: event.target.value,
    })
  }

  onAddressInput(event: React.ChangeEvent<HTMLInputElement>){
    this.setState({
      address: event.target.value,
    })
  }

  onZipCodeInput(event: React.ChangeEvent<HTMLInputElement>){
    this.setState({
      zip: event.target.value,
    })
  }

  onCoInput(event: React.ChangeEvent<HTMLInputElement>){
    this.setState({
      co: event.target.value,
    })
  }

  onCountryInput(event: React.ChangeEvent<HTMLInputElement>){
    this.setState({
      country: event.target.value,
    })
  }

  handleSubmit(e: any){
    e.preventDefault();

    // Read the form data
    const form = e.target;
    const formData = new FormData(form);

    const formJson = Object.fromEntries(formData.entries());
    console.log(formJson); //Errorhandling, remove later
  }


  render() {
    return (
      <form method="post" onSubmit={this.handleSubmit}> 
      <div>
        <h1>Register Account</h1>
        <input type="text" name="Username" placeholder="Username" onChange={this.onUsernameInput} />
        <br></br>
        <input type="text" name="Password" placeholder="Password" onChange={this.onPasswordInput}/>
        <br></br>
        <input type="text" name="Firstname" placeholder="Firstname" onChange={this.onFirstnameInput}/>
        <br></br>
        <input type="text" name="Email" placeholder="Email" onChange={this.onEmailInput}/>
        <br></br>
        <input type="text" name="Role" placeholder="Role" onChange={this.onRoleInput}/>
        <br></br>
        <input type="text" name="address" placeholder="address" onChange={this.onAddressInput}/>
        <br></br>
        <input type="text" name="Zip" placeholder="Zip" onChange={this.onZipCodeInput}/>
        <br></br>
        <input type="text" name="CO" placeholder="CO" onChange={this.onCoInput}/>
        <br></br>
        <input type="text" name="Country" placeholder="Country" onChange={this.onCountryInput}/>
        <br></br>
        <div>
        <a href="/">
        <img src={flygplan} className='logo' alt='flygplan'/>
        </a>
        </div>
        <div>
        <a href="/login">Already have and account? Click me!</a>  
        </div>
        <br></br>
        <button type="submit">Submit form</button>
      </div>
      </form>
    )
  }

}

export default Register