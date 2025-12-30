import { useEffect, useState } from "react";

function App() {
  const [message, setMessage] = useState("");

  useEffect(() => {
    fetch("http://localhost:3000/")
      .then((res) => res.text())
      .then((data) => setMessage(data))
      .catch((err) => {
        console.error(err);
        setMessage("Error fetchin data");
      });
  }, []);

  return (
    <div className="min-h-screen bg-slate-900 flex flex-col items-center justfy-center text-white">
      <h1 className="text-4xl font-semibold mb-4">my dev training</h1>
      <div className="p-6 bg-slate-800 rounded-lg shadow-xl border border-slate-700">
        <p className="text-gray-400 text-sm mb-2">message from backend: </p>
        <p className="text-2xl font-mono text-green-400">{message}</p>
      </div>
    </div>
  );
}

export default App;
