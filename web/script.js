document.addEventListener('DOMContentLoaded', function() {
  const canvas = document.getElementById("drawingCanvas");
  const ctx = canvas.getContext("2d");
  const baseUrl = window.location.origin;
  const arenaId = window.location.pathname.split("/")[2];
  const socket = new WebSocket(`ws://${baseUrl}/arena/${arenaId}/ws`);

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

  function clear() {
    isDrawing = false;
    lineData = [];
    redrawAllLines();
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
    sendLineUpdate();
  }

  function drawLine(pts) {
    if (pts.length < 2) return;
    ctx.beginPath();
    ctx.moveTo(pts[0].x, pts[0].y);
    for (let i = 1; i < pts.length; i++) {
      ctx.lineTo(pts[i].x, pts[i].y);
    }
    ctx.stroke();
  }

  function redrawAllLines() {
    ctx.clearRect(0, 0, canvas.width, canvas.height);
    for (const line of lineData) {
      drawLine(line);
    }
  }

  // Currently unused
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
    return {
      x: touch.clientX - rect.left,
      y: touch.clientY - rect.top
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
});
