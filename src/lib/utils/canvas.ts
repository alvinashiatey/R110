import type { ProcessedImages } from "@lib/types";
import { processColormap } from "@lib/actions/image";
import { useStore } from "@lib/stores/useStore.svelte";

const COLOR_ORDERS = {
  default: ["cyan", "yellow", "magenta", "black"],
  threeColor: ["black", "magenta", "yellow", "cyan"],
} as const;

const sortImagesByChannel = (images: ProcessedImages[]): ProcessedImages[] => {
  const orderKey =
    useStore.processState.maxColors < 3 ? "threeColor" : "default";

  return [...images].sort(
    (a, b) =>
      COLOR_ORDERS[orderKey].indexOf(
        a.channel as (typeof COLOR_ORDERS)[typeof orderKey][number]
      ) -
      COLOR_ORDERS[orderKey].indexOf(
        b.channel as (typeof COLOR_ORDERS)[typeof orderKey][number]
      )
  );
};

function calculateScaling(
  img: { width: number; height: number },
  canvas: { width: number; height: number },
  zoomLevel?: number,
  offset: { x: number; y: number } = { x: 0, y: 0 }
) {
  const baseScale = Math.min(
    canvas.width / img.width,
    canvas.height / img.height
  );
  const scale = baseScale * (1 + (zoomLevel ?? -30) / 100);
  const scaledWidth = img.width * scale;
  const scaledHeight = img.height * scale;
  const x = (canvas.width - scaledWidth) / 2 + offset.x;
  const y = (canvas.height - scaledHeight) / 2 + offset.y;

  return { scale, scaledWidth, scaledHeight, x, y };
}

const loadImage = (base64: string): Promise<HTMLImageElement> => {
  return new Promise((resolve, reject) => {
    const img = new Image();
    img.src = base64;
    img.onload = () => resolve(img);
    img.onerror = reject;
  });
};

export function drawImageScaled(
  img: HTMLImageElement,
  canvas: HTMLCanvasElement,
  ctx: CanvasRenderingContext2D,
  zoomLevel?: number,
  offset: { x: number; y: number } = { x: 0, y: 0 }
) {
  if (!canvas || !ctx) return;

  const displayWidth = canvas.clientWidth;
  const displayHeight = canvas.clientHeight;
  const dpr = window.devicePixelRatio || 1;
  canvas.width = displayWidth * dpr;
  canvas.height = displayHeight * dpr;
  ctx.scale(dpr, dpr);

  const { scaledWidth, scaledHeight, x, y } = calculateScaling(
    img,
    { width: displayWidth, height: displayHeight },
    zoomLevel,
    offset
  );

  ctx.clearRect(0, 0, canvas.width, canvas.height);
  ctx.drawImage(img, x, y, scaledWidth, scaledHeight);
}

const applyColorMap = async (
  img_path: string,
  hexColor: string
): Promise<HTMLCanvasElement> => {
  try {
    // If the color is white, return an empty canvas immediately
    if (hexColor === "#FFFFFF") {
      return document.createElement("canvas");
    }

    // Check if the processed image is already cached
    const cacheKey = `${img_path}-${hexColor}`;
    if (useStore.inColormapCache(cacheKey)) {
      return useStore.getFromColormapCache(cacheKey)!;
    }

    // Process the image and store it in cache
    const base64 = await processColormap(img_path, hexColor);
    const img = await loadImage(base64);
    const canvas = document.createElement("canvas");
    const ctx = canvas.getContext("2d")!;
    canvas.width = img.width;
    canvas.height = img.height;
    ctx.drawImage(img, 0, 0);

    useStore.setToColormapCache(cacheKey, canvas);
    return canvas;
  } catch (error) {
    console.error("Error applying color map:", error);
    return document.createElement("canvas");
  }
};

const mergeImages = async (
  canvases: HTMLCanvasElement[],
  finalCanvas: HTMLCanvasElement,
  finalCtx: CanvasRenderingContext2D,
  zoomLevel?: number,
  offset: { x: number; y: number } = { x: 0, y: 0 }
): Promise<HTMLCanvasElement> => {
  const width = canvases[0].width;
  const height = canvases[0].height;

  // Set canvas properties for rendering
  const displayWidth = finalCanvas.clientWidth;
  const displayHeight = finalCanvas.clientHeight;
  const dpr = window.devicePixelRatio || 1;
  finalCanvas.width = displayWidth * dpr;
  finalCanvas.height = displayHeight * dpr;
  finalCtx.scale(dpr, dpr);

  finalCtx.clearRect(0, 0, finalCanvas.width, finalCanvas.height);

  const { scaledWidth, scaledHeight, x, y } = calculateScaling(
    { width, height },
    { width: displayWidth, height: displayHeight },
    zoomLevel,
    offset
  );

  // Start with the first image using normal mode
  finalCtx.globalCompositeOperation = "source-over";
  finalCtx.drawImage(canvases[0], x, y, scaledWidth, scaledHeight);

  // Multiply blend the rest
  for (let i = 1; i < canvases.length; i++) {
    finalCtx.globalCompositeOperation = "multiply";
    finalCtx.drawImage(canvases[i], x, y, scaledWidth, scaledHeight);
  }

  return finalCanvas;
};

export const prepareCMYKLayers = async (
  images: ProcessedImages[],
  hexColors: string[]
): Promise<HTMLCanvasElement[]> => {
  if (images.length !== 4) {
    throw new Error("Expected exactly 4 images (C, M, Y, K).");
  }

  // Sort images by channel order, determined by hexColors length
  const sortedImages = sortImagesByChannel(images);

  // Trim `#FFFFFF` padding and ensure we have at least 4 colors
  let assignedColors = hexColors.slice(0, 4);

  if (assignedColors.length === 0) {
    assignedColors = ["#00FFFF", "#FFFF00", "#FF00FF", "#000000"];
  }

  // Process images in the determined order
  return await Promise.all(
    sortedImages.map((image, index) =>
      applyColorMap(image.image_path, assignedColors[index] || "#FFFFFF")
    )
  );
};

export const renderCMYKLayers = async (
  layers: HTMLCanvasElement[],
  canvas: HTMLCanvasElement,
  ctx: CanvasRenderingContext2D,
  zoomLevel?: number,
  offset: { x: number; y: number } = { x: 0, y: 0 }
) => {
  if (!layers.length) return;
  await mergeImages(layers, canvas, ctx, zoomLevel, offset);
};

const processCMYKImages = async (
  images: ProcessedImages[],
  hexColors: string[],
  canvas: HTMLCanvasElement,
  ctx: CanvasRenderingContext2D,
  zoomLevel?: number
) => {
  const layers = await prepareCMYKLayers(images, hexColors);
  await renderCMYKLayers(layers, canvas, ctx, zoomLevel);
};

function debounce<T extends (...args: any[]) => void>(
  func: T,
  delay: number
): (...args: Parameters<T>) => void {
  let timeoutId: ReturnType<typeof setTimeout>;
  return function (...args: Parameters<T>) {
    clearTimeout(timeoutId);
    timeoutId = setTimeout(() => func(...args), delay);
  };
}

export const processCMYKImagesDebounced = debounce(processCMYKImages, 100);

/**
 * Creates a full-resolution composed image from the CMYK layers
 * This is used for saving the image at original quality regardless of zoom
 */
export const createFullResolutionComposedImage = (
  layers: HTMLCanvasElement[]
): HTMLCanvasElement | null => {
  if (!layers.length || !layers[0].width || !layers[0].height) return null;

  const width = layers[0].width;
  const height = layers[0].height;

  const outputCanvas = document.createElement("canvas");
  outputCanvas.width = width;
  outputCanvas.height = height;
  const ctx = outputCanvas.getContext("2d")!;

  // Start with white background
  ctx.fillStyle = "#FFFFFF";
  ctx.fillRect(0, 0, width, height);

  // Start with the first image using normal mode
  ctx.globalCompositeOperation = "source-over";
  ctx.drawImage(layers[0], 0, 0);

  // Multiply blend the rest
  for (let i = 1; i < layers.length; i++) {
    ctx.globalCompositeOperation = "multiply";
    ctx.drawImage(layers[i], 0, 0);
  }

  return outputCanvas;
};
