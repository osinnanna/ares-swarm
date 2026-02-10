import { useRef, useEffect } from 'react'
import init, { Rover } from "../../swarm-engine/pkg/swarm_engine"
import "./App.css"

function App() {
  const canvasRef = useRef<HTMLCanvasElement>(null);
  const roverRef = useRef<Rover>(null);

  useEffect(() => {
    async function setup() {
      await init();
      roverRef.current = Rover.new(100, 100);

      const ctx = canvasRef.current?.getContext("2d");

      const render = () => {
        if (!roverRef.current || !ctx) return;
        
        roverRef.current.tick();

        ctx.fillStyle = "#4B2C20";
        ctx.fillRect(0, 0, 800, 600);

        ctx.fillStyle = "white";
        ctx.fillRect(roverRef.current.x, roverRef.current.y, 10, 10);

        requestAnimationFrame(render);
      }
      render();
    }
    setup();
  }, [])
  return (
    <>
      <div id="main">
        <h1 className="heading">Ares Swarm Mission Control</h1>
        <canvas ref={canvasRef} width={800} height={600}></canvas>
      </div>
    </>
  )
}

export default App
