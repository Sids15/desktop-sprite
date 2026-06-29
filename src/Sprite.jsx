import { useState, useEffect } from "react";

// Renders one frame of the sprite sheet using CSS background-position stepping.
// A frame is defined by a rectangle: (x, y) top-left of frame 0, w x h size,
// stepping right by w for each subsequent frame index.
export default function Sprite({ spriteSheet, x, y, w, h, frame, scale = 1, flip = false }) {
  const [sheetDims, setSheetDims] = useState(null);

  useEffect(() => {
    if (!spriteSheet) return;
    const img = new Image();
    img.onload = () => {
      setSheetDims({ w: img.naturalWidth, h: img.naturalHeight });
    };
    img.src = spriteSheet;
  }, [spriteSheet]);

  return (
    <div
      className="sprite"
      style={{
        width: `${w}px`,
        height: `${h}px`,
        backgroundImage: `url(${spriteSheet})`,
        backgroundRepeat: "no-repeat",
        backgroundSize: sheetDims ? `${sheetDims.w}px ${sheetDims.h}px` : "auto",
        // Offset to the current frame's top-left on the sheet.
        backgroundPosition: `-${x + frame * w}px -${y}px`,
        transform: `scale(${flip ? -scale : scale}, ${scale})`,
        transformOrigin: "center",
        imageRendering: "pixelated",
      }}
    />
  );
}

