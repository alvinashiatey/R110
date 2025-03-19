import { useStore, type AppResponse } from "../stores/useStore.svelte";
import { invoke } from "@tauri-apps/api/core";

let isSelectingImage = false;

async function convertAndSetImageData(
  base64String: string | undefined,
  imageType: string
) {
  if (!base64String) return;
  const imageDataUrl = `data:image/${imageType};base64,${base64String}`;
  console.log("Base64 String");
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

export async function selectImage() {
  if (isSelectingImage) return;
  isSelectingImage = true;

  try {
    const result = await invoke<AppResponse>("select_image");
    if (result) {
      const { image_path, image_type, image_name } = result;
      console.log(image_path, image_type);
      useStore.setImagePath(image_path);
      useStore.setImageName(image_name);

      const base64String = await handleImageProcessing();
      await convertAndSetImageData(base64String, image_type);
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
  console.log({
    colors,
    effect,
    filter,
  });

  const process_data = {
    colors: colors.length > 0 ? colors : null,
    effect: effect || null,
    filter: filter || null,
  };

  console.log("Processing data", process_data);

  const { image_type } = await invoke<AppResponse>("process_selected_image", {
    process_data,
  });
  const base64String = await handleImageProcessing();
  await convertAndSetImageData(base64String, image_type);
}
