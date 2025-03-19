<script lang="ts">
  import { onMount } from "svelte";

  interface Props {
    image?: string;
    zoomLevel?: number;
  }

  let { image, zoomLevel }: Props = $props();
  let canvas: HTMLCanvasElement;
  let ctx: CanvasRenderingContext2D;

  function drawImageScaled(img: HTMLImageElement) {
    if (!canvas || !ctx) return;

    // Get canvas display size from CSS
    const displayWidth = canvas.clientWidth;
    const displayHeight = canvas.clientHeight;

    // Get device pixel ratio for high-res screens (Retina, etc.)
    const dpr = window.devicePixelRatio || 1;

    // Set actual canvas size based on DPR
    canvas.width = displayWidth * dpr;
    canvas.height = displayHeight * dpr;

    // Scale the context so drawings aren't blurry
    ctx.scale(dpr, dpr);

    // Calculate aspect ratio scaling
    const baseScale = Math.min(
      displayWidth / img.width,
      displayHeight / img.height
    );
    const scale = baseScale * (1 + (zoomLevel ?? -30) / 100);
    const scaledWidth = img.width * scale;
    const scaledHeight = img.height * scale;

    // Center the image
    const x = (displayWidth - scaledWidth) / 2;
    const y = (displayHeight - scaledHeight) / 2;

    // Clear and draw
    ctx.clearRect(0, 0, canvas.width, canvas.height);
    ctx.drawImage(img, x, y, scaledWidth, scaledHeight);
  }

  onMount(() => {
    if (canvas) {
      ctx = canvas.getContext("2d")!;
    }
  });

  $effect(() => {
    if (ctx && image && zoomLevel !== undefined) {
      const img = new Image();
      img.onload = () => drawImageScaled(img);
      img.src = image;
    }
  });
</script>

<canvas id="canvas" bind:this={canvas}></canvas>

<style>
  #canvas {
    width: 100%;
    height: 100%;
  }
</style>
