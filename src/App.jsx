import { useState } from "react";
import { invoke } from "@tauri-apps/api/core";
import "./App.css";

function App() {
  const [formData, setFormData] = useState({
    email: "",
    password: ""
  });

  async function handleSubmission(e) {
    e.preventDefault();

    try {
      const response = await invoke("create_account", {
        data: formData
      });

      console.log("Server response:", response);
      alert("Account created successfully!");
    } catch (err) {
      console.error("Error:", err);
      alert("Failed to create account.");
    }
  }

  function handleInputChange(e) {
    const { name, value } = e.target;
    setFormData(prev => ({
      ...prev,
      [name]: value
    }));
  }

  async function requestAccounts() {
    let res = await invoke("get_accounts");

    if (res) {
      console.log(res)
    }
    else {
      console.log("Failure to Request")
    }
  }

  return (
  <>
    <button onClick={requestAccounts}>Request Accounts</button>
    <form className="form" onSubmit={handleSubmission}>
      <label htmlFor="email">Email:</label>
      <input
        type="text"
        name="email"
        placeholder="email@gmail.com"
        value={formData.email}
        onChange={handleInputChange}
      />

      <label htmlFor="password">Password:</label>
      <input
        type="password"
        name="password"
        placeholder="skibidi1234"
        value={formData.password}
        onChange={handleInputChange}
      />

      <input type="submit" value="Create Account" />
    </form>
  </>
  );
}

export default App;
