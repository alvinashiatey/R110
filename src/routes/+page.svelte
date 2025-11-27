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
        colors={useStore.activeColors || undefined}
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

    @media (prefers-color-scheme: dark) {
        :root {
            color: #f6f6f6;
            background-color: #2f2f2f;
        }
    }
</style>
