import React from "react";
import ReactDOM from "react-dom/client";
import App from "./App.jsx";
import "./main.css";

document.addEventListener("DOMContentLoaded", () => {
	const platformClass = `platform-${window.electronAPI.platform}`;
	document.body.classList.add(platformClass);
});

// Alternative way without duplicate tree for debug
ReactDOM.createRoot(document.getElementById("root")).render(<App />);

/*
ReactDOM.createRoot(document.getElementById("root")).render(
	<React.StrictMode>
		<App />
	</React.StrictMode>
);
*/
