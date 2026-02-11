import { useRef, useEffect } from 'react'
import init, { Swarm } from "../../swarm-engine/pkg/swarm_engine"
import "./App.css"

function App() {
  const canvasRef = useRef<HTMLCanvasElement>(null);
  const swarmRef = useRef<Swarm | null>(null);

  useEffect(() => {
    let animationId: number;
    let isMounted = true;

    async function setup() {
      await init();
      if (!isMounted) return;

      swarmRef.current = Swarm.new(100, 800.0, 600.0);
      const ctx = canvasRef.current?.getContext("2d");

      const render = () => {
        if (!isMounted || !swarmRef.current || !ctx) return;

        swarmRef.current.tick();

        ctx.fillStyle = "#4B2C20";
        ctx.fillRect(0, 0, 800, 600);

        ctx.fillStyle = "white";
        const count = swarmRef.current.count();

        // draw every single rover that is created
        for (let i = 0; i < count; i++) {
          const x = swarmRef.current.get_rover_x(i);
          const y = swarmRef.current.get_rover_y(i);
          ctx.fillRect(x, y, 4, 4);
        }

        animationId = requestAnimationFrame(render);
      };
      render();
    }

    setup();

    return () => {
      isMounted = false;
      cancelAnimationFrame(animationId);
      swarmRef.current = null;
    };
  }, []);

  return (
    <div id="main">
      <h1 className="heading">Ares Swarm Mission Control</h1>
      <canvas ref={canvasRef} width={800} height={600}></canvas>
    </div>
  );
}

export default App