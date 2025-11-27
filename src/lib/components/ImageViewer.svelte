<script lang="ts">
  import { PlusCircle, MinusCircle, FloppyDisk } from "phosphor-svelte";
  import Canvas from "@ui/Canvas.svelte";
  import type { ProcessedImages } from "@lib/types";
  import { saveComposedImage } from "@lib/actions/image";

  interface Props {
    image?: string;
    imageName?: string;
    colors?: string[];
    processedImages?: ProcessedImages[];
  }
  const ZOOM_MIN = -30;
  const ZOOM_MAX = 300;

  let { image, imageName, colors, processedImages }: Props = $props();
  let zoomLevel = $state(ZOOM_MIN);
  let canvasElement: HTMLCanvasElement | null = $state(null);
  let getFullResImage: (() => HTMLCanvasElement | null) | null = $state(null);

  function zoomIn() {
    if (zoomLevel < ZOOM_MAX) zoomLevel += 20;
  }

  function zoomOut() {
    if (zoomLevel > ZOOM_MIN) zoomLevel -= 20;
  }

  function handleSave() {
    // Use full resolution image if available, otherwise fall back to display canvas
    const fullResCanvas = getFullResImage?.();
    if (fullResCanvas) {
      saveComposedImage(fullResCanvas);
    } else if (canvasElement) {
      saveComposedImage(canvasElement);
    }
  }

  function setCanvasRef(el: HTMLCanvasElement) {
    canvasElement = el;
  }

  function setFullResolutionGetter(fn: () => HTMLCanvasElement | null) {
    getFullResImage = fn;
  }

  // Check if we have a composed image to save (processed images with colors)
  let canSave = $derived(processedImages && processedImages.length > 0);
</script>

<div class="image-viewer">
  <div class="image-container">
    <div class="image">
      <Canvas
        {image}
        {zoomLevel}
        {colors}
        {processedImages}
        canvasRef={setCanvasRef}
        getFullResolutionImage={setFullResolutionGetter}
      />
    </div>

    <div class="image-detail-panel">
      <div class="panel-wrapper">
        <div class="info">
          <p>
            {image ? `Image: ${imageName}` : "No image selected"}
          </p>
        </div>
        <div class="buttons">
          {#if canSave}
            <button onclick={handleSave} title="Save composed image">
              <FloppyDisk size="1.25rem" />
            </button>
          {/if}
          <button onclick={zoomOut} disabled={zoomLevel === ZOOM_MIN}>
            <MinusCircle size="1.25rem" />
          </button>
          <button onclick={zoomIn} disabled={zoomLevel === ZOOM_MAX}>
            <PlusCircle size="1.25rem" />
          </button>
        </div>
      </div>
    </div>
  </div>
</div>

<style>
  .image-viewer {
    position: absolute;
    top: 0;
    left: 0;
    width: 100%;
    height: var(--top-height);
    background-color: #fefefe;
  }
  .image-container {
    position: relative;
    display: flex;
    justify-content: center;
    align-items: center;
    width: 100%;
    height: 100%;
    background-color: tomato;
  }

  .image-container .image {
    width: 100%;
    height: 100%;
    background-color: #f0f0f0;
  }

  .image-detail-panel {
    position: absolute;
    bottom: 0;
    left: 0;
    width: 100%;
    background-color: transparent;
  }

  .panel-wrapper {
    display: flex;
    justify-content: space-between;
    align-items: center;
    padding: 1rem;
    width: 100%;
  }

  .info {
    opacity: 0.05;
    transition: opacity 0.25s cubic-bezier(0.445, 0.05, 0.55, 0.95);
  }

  .info:hover {
    opacity: 1;
  }

  .info p {
    margin: 0;
    font-size: 0.8rem;
    color: #0f0f0f;
    background-color: rgba(240, 240, 240, 0.5);
    border-radius: 0.5rem;
    padding-inline: 0.5rem;
    backdrop-filter: blur(5px);
    cursor: default;
  }

  .buttons {
    display: grid;
    grid-auto-flow: column;
    align-items: center;
    justify-content: center;
    gap: 0.25rem;
    background-color: rgba(240, 240, 240, 0.5);
    border-radius: 0.5rem;
    padding: 0.25rem;
    backdrop-filter: blur(5px);
  }

  .buttons button {
    border: none;
    background-color: transparent;
    display: grid;
    place-items: center;
    cursor: pointer;
  }

  .image {
    width: 70%;
    height: 60%;
    background-color: #f0f0f0;
  }
</style>
