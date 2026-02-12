import { useRef, useEffect } from 'react'
import init, { Swarm } from "../../swarm-engine/pkg/swarm_engine"
import "./App.css"

function App() {
  const canvasRef = useRef<HTMLCanvasElement>(null);
  const swarmRef = useRef<Swarm | null>(null);

  const CANVAS_WIDTH = 800.0;
  const CANVAS_HEIGHT = 600.0;

  useEffect(() => {
    let animationId: number;
    let isMounted = true;

    async function setup() {
      await init();
      if (!isMounted) return;

      swarmRef.current = Swarm.new(100, CANVAS_WIDTH, CANVAS_HEIGHT);
      const ctx = canvasRef.current?.getContext("2d");

      const render = () => {
        if (!isMounted || !swarmRef.current || !ctx) return;

        swarmRef.current.tick();

        // the -> Unexplored" background not touched
        ctx.fillStyle = "#1a0f0a";
        ctx.fillRect(0, 0, 800, 600);

        // 2. Drawing the Mapped Areas
        ctx.fillStyle = "#4B2C20";
        const cols = CANVAS_WIDTH / 10;
        const rows = CANVAS_HEIGHT / 10;

        for (let x = 0; x < cols; x++) {
          for (let y = 0; y < rows; y++) {
            if (swarmRef.current.is_cell_mapped(x, y)) {
              ctx.fillRect(x * 10, y * 10, 10, 10);
            }
          }
        }

        // Drawing the rover loop
        ctx.fillStyle = "white";
        const count = swarmRef.current.count();
        for (let i = 0; i < count; i++) {
          const rx = swarmRef.current.get_rover_x(i);
          const ry = swarmRef.current.get_rover_y(i);
          ctx.fillRect(rx, ry, 3, 3);
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

export default App;