<script lang="ts">
    import { onMount } from "svelte";
    import type { ProcessedImages } from "../../types";
    import {
        drawImageScaled,
        processCMYKImagesDebounced,
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

    onMount(() => {
        if (canvas) {
            ctx = canvas.getContext("2d")!;
        }
    });

    $effect(() => {
        if (processedImages && processedImages.length) {
            const defaultColors = ["#00FFFF", "#FF00FF", "#FFFF00", "#000000"];
            const activeColors =
                colors && colors.length > 0 ? colors : defaultColors;

            processCMYKImagesDebounced(
                processedImages,
                activeColors,
                canvas,
                ctx,
                zoomLevel,
            );
        } else {
            if (ctx && image && zoomLevel !== undefined) {
                const img = new Image();
                img.onload = () => drawImageScaled(img, canvas, ctx, zoomLevel);
                img.src = image;
            }
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
