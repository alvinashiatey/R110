interface R_COLOR {
  name: string;
  hex: string;
}
const RISOCOLORS: R_COLOR[] = [
  {
    name: "Black",
    hex: "#000000",
  },
  {
    name: "Yellow",
    hex: "#FFE800",
  },
  {
    name: "Green",
    hex: "#01A95D",
  },
  {
    name: "Red",
    hex: "#F15060",
  },
  {
    name: "Flat Gold",
    hex: "#BB8B41",
  },
  {
    name: "Florescent Pink",
    hex: "#FF48B0",
  },
  {
    name: "Blue",
    hex: "#0078BF",
  },
];

class ColorStore {
  colors = $state<R_COLOR[]>([]);
  async setColors(colors: R_COLOR[]) {
    this.colors = colors;
  }
  async addColor(color: R_COLOR) {
    this.colors = [...this.colors, color];
  }
  async getColorByName(name: string) {
    return this.colors.find((color) => color.name === name);
  }
}

export const useColors = new ColorStore();
useColors.setColors(RISOCOLORS);
