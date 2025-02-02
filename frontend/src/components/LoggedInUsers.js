import React, { useEffect, useState, useContext } from "react";
import { AuthContext } from "../context/AuthContext";

const LoggedInUsers = () => {
  const { token } = useContext(AuthContext);
  const [users, setUsers] = useState([]);

  useEffect(() => {
    fetch("http://127.0.0.1:8000/logged-in-users", {
      headers: { Authorization: `Bearer ${token}` },
    })
      .then((res) => res.json())
      .then(setUsers)
      .catch(console.error);
  }, [token]);

  return (
    <div>
      <h2>Eingeloggte Benutzer</h2>
      <ul>
        {users.map((email, index) => (
          <li key={index}>{email}</li>
        ))}
      </ul>
    </div>
  );
};

export default LoggedInUsers;
