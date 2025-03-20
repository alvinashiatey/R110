<script lang="ts">
  import { useColors } from "@store/useColors.svelte";

  interface Props {
    value: string[];
    name: string;
    required: boolean;
    disabled: boolean;
    max: number;
    valueChanged?: (value: string) => void;
  }

  let { value, name, required, disabled, max, valueChanged }: Props = $props();

  // State
  let selectedColors = $state(value || []);
  let previousMax = $state(max);

  // Watch for external value changes
  $effect(() => {
    selectedColors = value;
  });

  $effect(() => {
    if (max !== previousMax) {
      selectedColors.map(valueChanged!).filter(Boolean);
      previousMax = max;
    }
  });

  function toggleColor(hex: string): void {
    valueChanged?.(hex);

    const selectElement = document.querySelector<HTMLSelectElement>(
      `select[name="${name}"]`
    );
    selectElement?.dispatchEvent(new Event("change"));
  }

  // Helper function to check if a color should be disabled
  function isColorDisabled(hex: string): boolean {
    return selectedColors.length >= max && !selectedColors.includes(hex);
  }

  // Helper function to get the index number of a selected color
  function getSelectionNumber(hex: string): number {
    return selectedColors.indexOf(hex) + 1;
  }
</script>

<div class="colors">
  {#each useColors.colors as color}
    <button
      class="color"
      class:selected={selectedColors.includes(color.hex)}
      class:disabled={isColorDisabled(color.hex)}
      style="background-color: {color.hex}"
      onclick={() => toggleColor(color.hex)}
      disabled={isColorDisabled(color.hex)}
      type="button"
      aria-pressed={selectedColors.includes(color.hex)}
      aria-label={`Select ${color.name}`}
    >
      {#if selectedColors.includes(color.hex)}
        <span class="selection-number">{getSelectionNumber(color.hex)}</span>
      {/if}
      <span class="sr-only">{color.name}</span>
    </button>
  {/each}
</div>

<select
  {name}
  {required}
  {disabled}
  class="sr-only"
  multiple
  bind:value={selectedColors}
  onchange={(event) => {
    const selectElement = event.target as HTMLSelectElement;
    const selectedOptions = Array.from(selectElement.selectedOptions);
    selectedColors = selectedOptions.map((option) => option.value);
  }}
>
  {#each useColors.colors as color}
    <option value={color.hex}>{color.name}</option>
  {/each}
</select>

<style>
  .colors {
    display: flex;
    gap: 0.5rem;
  }

  .color {
    position: relative;
    display: flex;
    align-items: center;
    justify-content: center;
    width: 1.5rem;
    height: 1.5rem;
    border-radius: 50%;
    cursor: pointer;
    transition:
      transform 0.2s ease,
      border 0.2s ease;
    border: 2px solid transparent;
  }

  .color:hover:not(.disabled) {
    transform: scale(1.1);
  }

  .color.selected {
    border: 2px dashed #fefefe;
  }

  .color.disabled {
    opacity: 0.5;
    cursor: not-allowed;
  }

  .selection-number {
    position: absolute;
    top: 50%;
    left: 50%;
    transform: translate(-50%, -50%);
    color: white;
    font-size: 0.75rem;
    font-weight: bold;
    text-shadow: 0 0 2px black;
  }
</style>
