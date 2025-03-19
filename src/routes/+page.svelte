<script lang="ts">
  import ImageViewer from "@lib/components/ImageViewer.svelte";
  import ToolBox from "$lib/components/ToolBox.svelte";
  import type { ProcessingCompletePayload } from "@lib/types";
  import { useStore } from "@store/useStore.svelte";
  import { listen } from "@tauri-apps/api/event";

  listen<ProcessingCompletePayload>("processing-complete", (event) => {
    useStore.setProcessedImages(event.payload.processed_images);
  });
</script>

<main class="container">
  <div class="title-bar" data-tauri-drag-region></div>

  <ImageViewer
    image={useStore.imageData || undefined}
    imageName={useStore.imageName || undefined}
    colors={useStore.processState.colors || undefined}
    processedImages={useStore.processedImages || undefined}
  />
  <ToolBox />
</main>

<style>
  :root {
    font-family: Arial, sans-serif;
    font-size: 16px;
    line-height: 24px;
    font-weight: 400;

    color: #0f0f0f;
    background-color: #fefefe;

    font-synthesis: none;
    text-rendering: optimizeLegibility;
    -webkit-font-smoothing: antialiased;
    -moz-osx-font-smoothing: grayscale;
    -webkit-text-size-adjust: 100%;
  }

  .title-bar {
    position: absolute;
    top: 0;
    left: 0;
    z-index: 1000;
    width: 100%;
    height: 2em;
    background-color: transparent;
  }

  .container {
    margin: 0;
    display: flex;
    flex-direction: column;
    justify-content: center;
    text-align: center;
  }

  input,
  button {
    border-radius: 8px;
    border: 1px solid transparent;
    padding: 0.6em 1.2em;
    font-size: 1em;
    font-weight: 500;
    font-family: inherit;
    color: #0f0f0f;
    background-color: #ffffff;
    transition: border-color 0.25s;
    box-shadow: 0 2px 2px rgba(0, 0, 0, 0.2);
  }

  button {
    cursor: pointer;
  }

  button:hover {
    border-color: #396cd8;
  }
  button:active {
    border-color: #396cd8;
    background-color: #e8e8e8;
  }

  input,
  button {
    outline: none;
  }

  #greet-input {
    margin-right: 5px;
  }

  @media (prefers-color-scheme: dark) {
    :root {
      color: #f6f6f6;
      background-color: #2f2f2f;
    }

    a:hover {
      color: #24c8db;
    }

    input,
    button {
      color: #ffffff;
      background-color: #0f0f0f98;
    }
    button:active {
      background-color: #0f0f0f69;
    }
  }
</style>
