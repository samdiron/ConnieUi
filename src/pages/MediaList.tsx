import "../index.css";
// import { useState } from "react";
import { invoke } from "@tauri-apps/api/core";
import React from 'react';
import { useState, useEffect } from 'react';
import { Media } from "../strcts/MediaStruct"; 

const userCpid: String = "c9aba43b-fcd4-4f65-87ee-8612a9e69767";




async function addMore() {
  console.log("cunt");
  let media: Media[] = await invoke<Media[]>("list_media", {userCpid});
  media.map(med=> console.log(med.name))
 
}



function MediaList() {
  const [media, setMedia] = useState(null);

  useEffect(()=> {
    const fetchMedia = async() => {
      const med: Media[] = await invoke<Media[]>("list_media", {userCpid});
      setMedia(med);
    }
    fetchMedia();
  }, []);
  // let media: Media[] = await invoke<Media[]>("list_media", {userCpid});
  // let msg: String[] = ["cunt", "bitch"];

  return (
<>
<div>
  <button className="bg-blue-500" onClick={addMore} >button</button>
  <ul id="list" className="space-y-2 flex flex-col">
    <h1>{userCpid}</h1>
     { media?.map(med => (
      <li className="bg-amber-400 rounded-lg " >| {med.name} | </li>
      ))
     }
  </ul>
</div>
</>
  ) 
} 

export default MediaList 
