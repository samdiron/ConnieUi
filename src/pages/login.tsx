import "../index.css";


function Login() {
  localStorage.theme = "dark";
  document.documentElement.classList.toggle(
    "dark",
    localStorage.theme === "dark" ||
      (!("theme" in localStorage) && window.matchMedia("(prefers-color-scheme: dark)").matches),
  );

 return (
  <>
    <div className="flex justify-center items-center h-screen w-screen bg-black ">
        <div className="bg-amber-400 w-75 h-100 border-2 border-white flex-col" >
          <form>
            <label for="username"> Username:</label>
            <input className="bg-white m-3 mt-3 w-65 " id="username" place-holder="name" type="text" />
          </form>
        </div>
    </div>
      
  </>    
) 
}


export default Login
