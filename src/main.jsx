import React from "react";
import ReactDOM from "react-dom/client";
import App from "./App";
import Nav from "./components/Nav";
import SideBar from "./components/SideBar.jsx"

ReactDOM.createRoot(document.getElementById("root")).render(
  <React.StrictMode>
    <main class="bg-black h-screen ">
      <SideBar />
      <App />

    </main>
  </React.StrictMode>
);
