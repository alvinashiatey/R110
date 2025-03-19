export type Channels = "cyan" | "magenta" | "yellow" | "black";
export interface ProcessData {
  channel: string;
  image_path: string;
}
export interface ProcessedImages extends ProcessData {
  image_data: string | null;
}

export interface ProcessingCompletePayload {
  processed_images: ProcessedImages[];
  status: string;
}
