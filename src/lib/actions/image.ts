import { useStore, type AppResponse } from "../stores/useStore.svelte";
import { invoke } from "@tauri-apps/api/core";
import type { Channels, ProcessedImages } from "../types";

let isSelectingImage = false;

async function convertAndSetImageData(
  base64String: string | undefined,
  imageType: string
) {
  if (!base64String) return;
  const imageDataUrl = `data:image/${imageType};base64,${base64String}`;
  useStore.setImageData(imageDataUrl);
}

async function handleImageProcessing(): Promise<string | undefined> {
  try {
    return await invoke<string>("read_image");
  } catch (e) {
    console.error("Error processing image:", e);
    return undefined;
  }
}

async function handleProcessedImagesRead(): Promise<
  [string, Channels][] | undefined
> {
  try {
    return await invoke<[string, Channels][]>("read_processed_images");
  } catch (error) {
    console.error("Error reading processed images:", error);
    return undefined;
  }
}

export async function selectImage() {
  if (isSelectingImage) return;
  isSelectingImage = true;

  try {
    const result = await invoke<AppResponse>("select_image");
    if (result) {
      const { image_path, image_type, image_name } = result;
      useStore.setImagePath(image_path);
      useStore.setImageName(image_name);
      const base64String = await handleImageProcessing();
      await convertAndSetImageData(base64String, image_type);
      useStore.setProcessedImages([]);
      useStore.resetProcessState();
      useStore.resetColormapCache();
    }
  } catch (e) {
    console.error("Frontend Error: ", e);
  } finally {
    isSelectingImage = false;
  }
}

export async function submitProcessData() {
  if (!useStore.imagePath) {
    console.error("No image selected");
    return;
  }

  const { colors, effect, filter } = useStore.processState;

  const process_data = {
    colors: colors.length > 0 ? colors : null,
    effect: effect || null,
    filter: filter || null,
  };

  const { image_type, processed_images } = await invoke<AppResponse>(
    "process_selected_image",
    {
      process_data,
    }
  );
  if (processed_images?.length) {
    const images = await handleProcessedImagesRead();
    if (images) {
      const processedImages = processed_images
        .map((image, index) => {
          if (image.channel === images[index][1]) {
            return {
              ...image,
              image_data: `data:image/${image_type};base64,${images[index][0]}`,
            } as ProcessedImages;
          }
          return undefined;
        })
        .filter((item): item is ProcessedImages => item !== undefined);
      useStore.setProcessedImages(processedImages);
    }
  } else {
    console.error("No processed images found");
    useStore.setProcessedImages([]);
  }
}

export async function processColormap(
  imagePath: string,
  hexColor: string
): Promise<string> {
  try {
    return await invoke<string>("process_colormap", {
      image_path: imagePath,
      hex_color: hexColor,
    });
  } catch (error) {
    console.error("Error processing colormap:", error);
    return "";
  }
}
