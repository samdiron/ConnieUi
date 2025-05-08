import "../index.css";
// import { useState } from "react";
import { invoke } from "@tauri-apps/api/core";
import React from 'react';
import { Media } from "../strcts/MediaStruct"; 

const userCpid: String = "c9aba43b-fcd4-4f65-87ee-8612a9e69767";




async function addMore() {
  console.log("cunt");
  let media: Media[] = await invoke<Media[]>("list_media", {userCpid});
  console.log(media[0].name)
};



function MediaList() {


  // let msg: String[] = ["cunt", "bitch"];

  return (
<>
<div>
  <button className="bg-blue-500" onClick={addMore} >button</button>
  <ul id="list">
    <h1>{userCpid}</h1>
    {/* {msg.map(m => { */}
    {/*   return ( */}
    {/*   <li >{m}</li> */}
    {/* ) */}
    
  </ul>
</div>
</>
  ) 
} 

export default MediaList 
