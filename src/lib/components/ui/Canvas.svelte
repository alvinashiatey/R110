<script lang="ts">
  import { onMount } from "svelte";
  import type { ProcessedImages } from "../../types";
  import {
    drawImageScaled,
    prepareCMYKLayers,
    renderCMYKLayers,
  } from "../../utils/canvas";

  interface Props {
    image?: string;
    zoomLevel?: number;
    colors?: string[];
    processedImages?: ProcessedImages[];
  }

  let { image, zoomLevel, colors, processedImages }: Props = $props();
  let canvas: HTMLCanvasElement;
  let ctx: CanvasRenderingContext2D;

  let layers = $state<HTMLCanvasElement[]>([]);
  let loadedImage = $state<HTMLImageElement | null>(null);

  let isDragging = $state(false);
  let startX = $state(0);
  let startY = $state(0);
  let offsetX = $state(0);
  let offsetY = $state(0);
  let lastOffsetX = $state(0);
  let lastOffsetY = $state(0);

  onMount(() => {
    if (canvas) {
      ctx = canvas.getContext("2d")!;
    }
  });

  // Load base image
  $effect(() => {
    if (image) {
      const img = new Image();
      img.onload = () => {
        loadedImage = img;
      };
      img.src = image;
    } else {
      loadedImage = null;
    }
  });

  // Prepare CMYK layers
  $effect(() => {
    if (processedImages && processedImages.length) {
      const defaultColors = ["#00FFFF", "#FF00FF", "#FFFF00", "#000000"];
      const activeColors = colors && colors.length > 0 ? colors : defaultColors;

      prepareCMYKLayers(processedImages, activeColors).then((res) => {
        layers = res;
      });
    } else {
      layers = [];
    }
  });

  // Render loop
  $effect(() => {
    if (!ctx || !canvas) return;

    if (layers.length > 0) {
      renderCMYKLayers(layers, canvas, ctx, zoomLevel, {
        x: offsetX,
        y: offsetY,
      });
    } else if (loadedImage) {
      drawImageScaled(loadedImage, canvas, ctx, zoomLevel, {
        x: offsetX,
        y: offsetY,
      });
    }
  });

  function handleMouseDown(e: MouseEvent) {
    // Only allow dragging if zoomed in (optional, based on user request "passed a desired point")
    // For now, let's allow it always or maybe check zoomLevel > -30
    if ((zoomLevel ?? -30) > -30) {
      isDragging = true;
      startX = e.clientX;
      startY = e.clientY;
      lastOffsetX = offsetX;
      lastOffsetY = offsetY;
      canvas.style.cursor = "grabbing";
    }
  }

  function handleMouseMove(e: MouseEvent) {
    if (!isDragging) return;
    const dx = e.clientX - startX;
    const dy = e.clientY - startY;
    offsetX = lastOffsetX + dx;
    offsetY = lastOffsetY + dy;
  }

  function handleMouseUp() {
    isDragging = false;
    canvas.style.cursor = (zoomLevel ?? -30) > -30 ? "grab" : "default";
  }

  function handleMouseLeave() {
    isDragging = false;
    canvas.style.cursor = "default";
  }

  // Update cursor on zoom change
  $effect(() => {
    if (canvas) {
      canvas.style.cursor = (zoomLevel ?? -30) > -30 ? "grab" : "default";
    }
  });
</script>

<canvas
  id="canvas"
  bind:this={canvas}
  onmousedown={handleMouseDown}
  onmousemove={handleMouseMove}
  onmouseup={handleMouseUp}
  onmouseleave={handleMouseLeave}
></canvas>

<style>
  #canvas {
    width: 100%;
    height: 100%;
  }
</style>
