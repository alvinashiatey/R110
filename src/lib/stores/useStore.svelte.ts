import { config } from "@utils/config";
import type { ProcessData, ProcessedImages } from "@lib/types";

// Add these enum definitions at the top
export enum ImageEffect {
  Original = "Original",
  Dither = "Dither",
  HalfTone = "HalfTone",
  Threshold = "Threshold",
  Posterize = "Posterize",
}

export enum ImageFilter {
  Grayscale = "Grayscale",
  Sepia = "Sepia",
  Invert = "Invert",
  Pixelate = "Pixelate",
  Brighten = "Brighten",
  Darken = "Darken",
  Contrast = "Contrast",
  Blur = "Blur",
  Sharpen = "Sharpen",
}

interface ProcessState {
  isProcessing?: boolean;
  isProcessed?: boolean;
  colors: string[];
  effect: ImageEffect;
  filter: ImageFilter | null;
  maxColors: number;
  shouldFilter: boolean;
}

interface ExportState {
  exportType: number;
}

class AppState {
  imagePath = $state<string | null>(null);
  imageData = $state<string | null>(null);
  imageName = $state<string | null>(null);
  processedImages = $state<ProcessedImages[]>([]);
  colormapCache = $state<Map<string, HTMLCanvasElement>>(new Map());

  processState = $state<ProcessState>({
    shouldFilter: false,
    isProcessing: false,
    isProcessed: false,
    colors: [],
    effect: ImageEffect.Original,
    filter: null,
    maxColors: config.maxColors,
  });
  exportState = $state<ExportState>({
    exportType: 1,
  });

  setImagePath = async (path: string) => {
    this.imagePath = path;
  };

  setImageData = async (data: string) => {
    this.imageData = data;
  };

  setImageName = async (name: string) => {
    const extension = name.split(".").pop();
    const baseName = name.substring(0, name.lastIndexOf("."));
    if (baseName.length > 10) {
      const firstPart = baseName.substring(0, 5);
      const lastPart = baseName.substring(baseName.length - 5);
      name = `${firstPart}...${lastPart}${extension ? "." + extension : ""}`;
    }
    this.imageName = name;
  };

  setProcessState = async (state: ProcessState) => {
    this.processState = state;
  };

  resetProcessState = async () => {
    this.processState = {
      shouldFilter: false,
      isProcessing: false,
      isProcessed: false,
      colors: [],
      effect: ImageEffect.Original,
      filter: null,
      maxColors: config.maxColors,
    };
  };

  setToColormapCache = async (key: string, canvas: HTMLCanvasElement) => {
    this.colormapCache.set(key, canvas);
  };

  getFromColormapCache = (key: string) => {
    return this.colormapCache.get(key);
  };

  inColormapCache = (key: string) => {
    return this.colormapCache.has(key);
  };

  resetColormapCache = async () => {
    this.colormapCache = new Map();
  };

  setExportState = (state: ExportState) => {
    this.exportState = state;
  };

  setExportType = async (exportType: string) => {
    const type = Number(exportType);
    if (!Number.isNaN(type)) this.exportState.exportType = type;
  };

  setProcessedImages = async (processedImages: ProcessedImages[]) => {
    this.processedImages = processedImages;
  };

  toggleFilter = async () => {
    this.processState.shouldFilter = !this.processState.shouldFilter;
    if (!this.processState.shouldFilter) {
      this.processState.filter = null;
    } else {
      const filterValues = Object.values(ImageFilter);
      const randomIndex = Math.floor(Math.random() * filterValues.length);
      this.processState.filter = filterValues[randomIndex];
    }
    console.log(this.processState.shouldFilter, this.processState.filter);
  };

  setMaxColors = (maxColors: string): void => {
    const max = Number(maxColors);
    if (!Number.isNaN(max)) {
      this.processState.maxColors = max;
    }
  };

  setEffect = async (effect: string) => {
    this.processState.effect = effect as ImageEffect;
  };

  setFilter = (filter: string) => {
    this.processState.filter = filter as ImageFilter;
  };

  addColor = async (color: string) => {
    const colorIndex = this.processState.colors.indexOf(color);
    if (colorIndex !== -1) {
      this.processState.colors = this.processState.colors.filter(
        (c) => c !== color
      );
    } else if (this.processState.colors.length < this.processState.maxColors) {
      this.processState.colors = [...this.processState.colors, color];
    }
  };
}

export interface AppResponse {
  processed_images?: ProcessData[];
  image_path: string;
  image_type: string;
  image_name: string;
}

export const useStore = new AppState();
