import "../index.css";

import { useState } from "react";

export default function Login() {
  const [userName, setUserName] = useState("");
  const [hostName, setHostName] = useState("");
  const [password, setPassword] = useState("");
  const [serverName, setServerName] = useState("");
  return (
<>
  <dev class="flex justify-center items-center h-screen bg-lime-400 " >
  <dev class="bg-white w-100 h-100 space-y-8 rounded-lg flex-col " >
    <form 
      onSubmit={(e) => {
            e.preventDefault();
      }}
    >
      <ul class="text-gray-600 font-bold text-xs">~ Username</ul>
      <input
          id="username-input"
          class="border-2 rounded-lg border-black "
          onChange={(e) => setUserName(e.currentTarget.value)}
          placeholder="Enter a username..."
      />
      <ul class="text-gray-600 font-bold text-xs"><li>Servername</li></ul>
      <input
        id="serverName"
        class="border-2 rounded-lg border-black "
        onChange={(e) => setServerName(e.currentTarget.value)}
        placeholder="Enter a server name..."
      />
      <ul class="text-gray-600 font-bold text-xs">~ Hostname</ul>
      <input
        id="hostName"
        class="border-2 rounded-lg border-black "
        onChange={(e) => setHostName(e.currentTarget.value)}
        placeholder="Enter a host name..."
      />
      <button type="submit">Greet</button>

    </form> 
  </dev>
  </dev>
</>
  ) 
}
