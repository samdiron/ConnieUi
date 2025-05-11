import "../index.css";
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

  return (
<>
<div className="p-2 " >
  <div id="list" className="space-y-2 flex flex-col">
    <h1 className="text-amber-400 bg-black w-fit" >{userCpid}</h1>
     { media?.map(med => (
      <div>
        <p className="bg-amber-400 rounded-lg w-fit" >| {med.name} |</p>
        <p className="bg-amber-400 rounded-lg w-fit" >| {med.size} |</p>
        <p className="bg-amber-400 rounded-lg w-fit" >| {med.checksum} |</p>
      </div>
      ))
     }
  </div>
</div>
</>
  ) 
} 

export default MediaList 
