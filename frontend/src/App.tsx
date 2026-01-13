import { Button } from "./components/ui/button";

function App() {
  return (
    <div className="flex h-screen w-full items-center justify-center bg-gray-100">
      <div className="text-center space-y-4">
        <h1 className="text-4xl font-bold text-blue-600 underline">
          Tailwind v4 + shadcn!
        </h1>
        <Button>click me</Button>
      </div>
    </div>
  );
}

export default App;
