document.addEventListener('DOMContentLoaded', function() {
  const baseHttp = window.location.origin;

  const canvas = document.getElementById("drawingCanvas");
  if (!canvas) {
    return;
  }

  const ctx = canvas.getContext("2d");
  const baseWs = baseHttp.replace('http', 'ws');
  const arenaId = window.location.pathname.split("/").at(-1);
  const socket = new ReconnectingWebSocket(`${baseWs}/${arenaId}/ws`);

  const colorPalette = [
    "black",    // 0: Black
    "red",      // 1: Red
    "blue",     // 2: Blue
    "green",    // 3: Green
    "yellow",   // 4: Yellow
    "orange",   // 5: Orange
    "purple",   // 6: Purple
    "pink",     // 7: Pink
    "brown",    // 8: Brown
    "gray"      // 9: Gray
  ];

  let currentColor = colorPalette[0]; // Default color
  ctx.strokeStyle = currentColor;

  document.addEventListener("keydown", (e) => {
    if (e.key >= '0' && e.key <= '9') {
      const colorIndex = parseInt(e.key, 10);
      currentColor = colorPalette[colorIndex];
      ctx.strokeStyle = currentColor;
    }
  });

  let isDrawing = false;
  let lineData = [];

  // Handle incoming WebSocket messages
  socket.addEventListener("message", (event) => {
    const { event_type, data } = JSON.parse(event.data);

    // Update the entire canvas with the new state
    if (event_type === "full_state") {
      lineData = data;
      redrawAllLines();
    }

    // Handle the incremental update (new line)
    if (event_type === "line_update") {
      lineData.push(data);
      redrawAllLines();
    }
  });

  const clearButton = document.getElementById("clear-button");
  clearButton.onclick = (e) => { clearDrawing(); };

  function clearDrawing() {
    isDrawing = false;
    lineData = [];
    sendFullState();
  }

  function startDrawing(e) {
    isDrawing = true;
    const { offsetX, offsetY } = e;
    lineData.push([{ x: offsetX, y: offsetY }]);
  }

  function draw(e) {
    if (!isDrawing) return;
    const { offsetX, offsetY } = e;
    const lastLine = lineData[lineData.length - 1];
    lastLine.push({ x: offsetX, y: offsetY });
    drawLine(lastLine);
  }

  function stopDrawing() {
    if (!isDrawing) return;
    isDrawing = false;

    // Simplify the last line
    const lastLine = lineData[lineData.length - 1];
    const simplifiedLine = simplifyLine(lastLine);
    lineData[lineData.length - 1] = simplifiedLine;

    sendLineUpdate();
  }

  function drawLine(pts) {
    if (pts.length < 2) return;

    ctx.beginPath();
    ctx.moveTo(pts[0].x, pts[0].y);

    for (let i = 1; i < pts.length - 1; i++) {
      const midPointX = (pts[i].x + pts[i + 1].x) / 2;
      const midPointY = (pts[i].y + pts[i + 1].y) / 2;
      ctx.quadraticCurveTo(pts[i].x, pts[i].y, midPointX, midPointY);
    }

    // Draw the last segment
    ctx.lineTo(pts[pts.length - 1].x, pts[pts.length - 1].y);
    ctx.stroke();
  }

  function redrawAllLines() {
    ctx.clearRect(0, 0, canvas.width, canvas.height);
    for (const line of lineData) {
      drawLine(line);
    }
  }

  function sendFullState() {
    socket.send(JSON.stringify({
      event_type: "full_state",
      data: lineData
    }));
  }

  function sendLineUpdate() {
    const newLine = lineData[lineData.length - 1];
    socket.send(JSON.stringify({
      event_type: "line_update",
      data: [newLine]
    }));
  }

  function getTouchPosition(e) {
    const rect = canvas.getBoundingClientRect();
    const touch = e.touches[0];
    const x = touch.clientX - rect.left;
    const y = touch.clientY - rect.top;
    return {
      x: (x / rect.width) * 800,
      y: (y / rect.height) * 800,
    };
  }

  canvas.addEventListener("mousedown", startDrawing);
  canvas.addEventListener("mousemove", draw);
  canvas.addEventListener("mouseup", stopDrawing);
  canvas.addEventListener("mouseout", stopDrawing);

  canvas.addEventListener("touchstart", (e) => {
    e.preventDefault();
    const pos = getTouchPosition(e);
    startDrawing({ offsetX: pos.x, offsetY: pos.y });
  });

  canvas.addEventListener("touchmove", (e) => {
    e.preventDefault();
    const pos = getTouchPosition(e);
    draw({ offsetX: pos.x, offsetY: pos.y });
  });

  canvas.addEventListener("touchend", (e) => {
    e.preventDefault();
    stopDrawing();
  });

  function simplifyLine(points, tolerance = 3) {
    if (points.length < 3) return points;

    const sqTolerance = tolerance * tolerance;

    function getSqDist(p1, p2) {
      const dx = p1.x - p2.x;
      const dy = p1.y - p2.y;
      return dx * dx + dy * dy;
    }

    function getSqSegDist(p, p1, p2) {
      let x = p1.x;
      let y = p1.y;
      let dx = p2.x - p1.x;
      let dy = p2.y - p1.y;

      if (dx !== 0 || dy !== 0) {
        const t = ((p.x - p1.x) * dx + (p.y - p1.y) * dy) / (dx * dx + dy * dy);
        if (t > 1) {
          x = p2.x;
          y = p2.y;
        } else if (t > 0) {
          x += dx * t;
          y += dy * t;
        }
      }

      dx = p.x - x;
      dy = p.y - y;
      return dx * dx + dy * dy;
    }

    function simplifyRecursive(points, first, last, sqTolerance, simplified) {
      let maxSqDist = sqTolerance;
      let index = 0;

      for (let i = first + 1; i < last; i++) {
        const sqDist = getSqSegDist(points[i], points[first], points[last]);
        if (sqDist > maxSqDist) {
          index = i;
          maxSqDist = sqDist;
        }
      }

      if (maxSqDist > sqTolerance) {
        if (index - first > 1) simplifyRecursive(points, first, index, sqTolerance, simplified);
        simplified.push(points[index]);
        if (last - index > 1) simplifyRecursive(points, index, last, sqTolerance, simplified);
      }
    }

    const simplified = [points[0]];
    simplifyRecursive(points, 0, points.length - 1, sqTolerance, simplified);
    simplified.push(points[points.length - 1]);
    return simplified;
  }
});
