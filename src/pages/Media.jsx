import { invoke } from "@tauri-apps/api/core";


export default function Media() {
    let media = ["cunt"];
    async function print() {
        let rustMedia = await invoke("media")
        rustMedia.map((med) => (media.push(med)))
    }
    return (
        <>
        <button class="bg-black text-white w-fit active:bg-amber-200" onClick={print()}>print</button>
        <ul>
            {
                media.map((str, index) => (
                    <li key={index}>{str}</li>
                ))
            }
        </ul>
        </>
    )           

}