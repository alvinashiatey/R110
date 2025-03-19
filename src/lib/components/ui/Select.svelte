<script lang="ts">
  interface Option {
    label: string;
    value: string;
  }

  interface Props {
    options: Option[];
    value: string | null;
    placeholder: string;
    name: string;
    required: boolean;
    disabled: boolean;
    searchable?: boolean;
    anchor?: "top" | "bottom"; // New property
    myIcon?: any;
    valueChanged?: (value: string) => void;
  }

  let {
    options,
    value,
    placeholder,
    name,
    required,
    disabled,
    searchable,
    anchor = "top", // Default to 'top'
    myIcon,
    valueChanged,
  }: Props = $props();

  // State
  let isOpen = $state(false);
  let searchTerm = $state("");
  let highlightedIndex = $state(0);
  let selectedValue = $state(value);

  // Derived state
  let filteredOptions = $derived.by(() =>
    options.filter((option) =>
      option.label.toLowerCase().includes(searchTerm.toLowerCase())
    )
  );

  // Event handlers
  function toggleDropdown() {
    if (!disabled) {
      isOpen = !isOpen;
      if (isOpen) {
        searchTerm = "";
        highlightedIndex = options.findIndex(
          (opt) => opt.value === selectedValue
        );
        if (highlightedIndex === -1) {
          highlightedIndex = 0;
        }
      }
    }
  }

  function selectOption(option: Option) {
    selectedValue = option.value;
    isOpen = false;

    // Trigger both the custom event and the valueChanged callback
    dispatchEvent(new CustomEvent("change", { detail: option }));
    if (valueChanged) {
      valueChanged(option.value);
    }

    // Update the native select value
    const selectElement = document.querySelector(
      `select[name="${name}"]`
    ) as HTMLSelectElement;
    if (selectElement) {
      selectElement.value = option.value;
      selectElement.dispatchEvent(new Event("change"));
    }
  }

  function handleKeydown(event: KeyboardEvent) {
    if (disabled) return;

    switch (event.key) {
      case "Enter":
        event.preventDefault();
        if (isOpen && filteredOptions[highlightedIndex]) {
          selectOption(filteredOptions[highlightedIndex]);
        } else {
          toggleDropdown();
        }
        break;
      case "Escape":
        isOpen = false;
        break;
      case "ArrowDown":
        event.preventDefault();
        if (!isOpen) {
          isOpen = true;
        } else {
          highlightedIndex = Math.min(
            highlightedIndex + 1,
            filteredOptions.length - 1
          );
        }
        break;
      case "ArrowUp":
        event.preventDefault();
        if (!isOpen) {
          isOpen = true;
        } else {
          highlightedIndex = Math.max(highlightedIndex - 1, 0);
        }
        break;
    }
  }

  // Click outside directive
  function clickOutside(node: Element) {
    const handleClick = (event: MouseEvent) => {
      if (node !== event.target && !node.contains(event.target as Node)) {
        node.dispatchEvent(new CustomEvent("outclick"));
      }
    };

    document.addEventListener("mousedown", handleClick, true);

    return {
      destroy() {
        document.removeEventListener("mousedown", handleClick, true);
      },
    };
  }
</script>

<div
  class="dropdown-container"
  onkeypress={handleKeydown}
  use:clickOutside
  role="combobox"
  aria-haspopup="listbox"
  aria-expanded={isOpen}
  aria-controls="select-dropdown"
  aria-label={placeholder}
  tabindex="0"
>
  <button
    type="button"
    class="dropdown-button"
    onclick={toggleDropdown}
    {disabled}
    aria-labelledby="dropdown-label"
  >
    <span id="dropdown-label">
      {#if value === null}
        <span class="placeholder">{placeholder}</span>
      {:else}
        <span class={{ "icon-label": myIcon }}>
          {@render myIcon?.()}
          <span>{options.find((opt) => opt.value === value)?.label ?? ""}</span>
        </span>
      {/if}
    </span>
    <span class="dropdown-icon">
      <svg viewBox="0 0 20 20" fill="none" stroke="currentColor">
        <path
          d="M7 7l3 3 3-3"
          stroke-width="1.5"
          stroke-linecap="round"
          stroke-linejoin="round"
        />
      </svg>
    </span>
  </button>

  {#if isOpen}
    <div class="dropdown-menu {anchor}" role="listbox" id="dropdown-options">
      {#if searchable}
        <input
          type="text"
          class="search-input"
          placeholder="Search..."
          bind:value={searchTerm}
        />
      {/if}

      {#if filteredOptions.length === 0}
        <div class="no-options">No options found</div>
      {:else}
        {#each filteredOptions as option, index}
          <button
            class="dropdown-item {index === highlightedIndex
              ? 'highlighted'
              : ''}"
            role="option"
            aria-selected={value === option.value}
            onclick={() => selectOption(option)}
            id={`option-${option.value}`}
          >
            {option.label}
          </button>
        {/each}
      {/if}
    </div>
  {/if}
</div>

<!-- Hidden native select for form submission -->
<select
  onchange={(event: Event) => {
    const selectElement = event.target as HTMLSelectElement;
    valueChanged?.(selectElement.value);
  }}
  {name}
  {required}
  {disabled}
  class="sr-only"
  bind:value={selectedValue}
>
  <option value="" disabled selected={selectedValue === null}
    >{placeholder}</option
  >
  {#each options as option}
    <option value={option.value}>{option.label}</option>
  {/each}
</select>

<style>
  .dropdown-container {
    position: relative;
    width: 100%;
    font-family: Arial, sans-serif;
  }

  .dropdown-button {
    width: 100%;
    display: flex;
    justify-content: space-between;
    align-items: center;
    padding-inline: 0.5rem;
    background-color: transparent;
    border: 1px solid #ccc;
    border-radius: 6px;
    cursor: pointer;
    transition: all 0.2s ease;
  }

  .dropdown-button:focus {
    outline: none;
    border-color: #007bff;
    box-shadow: 0 0 4px rgba(0, 123, 255, 0.5);
  }

  .dropdown-button[disabled] {
    opacity: 0.5;
    cursor: not-allowed;
  }

  .placeholder {
    color: #888;
  }

  .icon-label {
    display: flex;
    gap: 0.5rem;
    align-items: center;
    justify-content: center;
  }

  .dropdown-icon {
    display: flex;
    align-items: center;
    pointer-events: none;
  }

  .dropdown-icon svg {
    width: 16px;
    height: 16px;
    color: #666;
  }

  .dropdown-menu {
    position: absolute;
    z-index: 10;
    width: fit-content;
    margin-top: 4px;
    background: white;
    border: 1px solid #ddd;
    border-radius: 6px;
    box-shadow: 0 4px 8px rgba(0, 0, 0, 0.1);
    max-height: 200px;
    overflow-y: auto;
  }

  .dropdown-menu.bottom {
    top: 100%;
  }

  .dropdown-menu.top {
    bottom: 100%;
  }

  .search-input {
    width: 100%;
    padding: 8px;
    border: none;
    border-bottom: 1px solid #ddd;
    outline: none;
    font-size: 14px;
  }

  .search-input:focus {
    border-bottom: 1px solid #007bff;
  }

  .dropdown-item {
    padding: 10px;
    cursor: pointer;
    transition: background 0.2s ease;
    width: 100%;
  }

  .dropdown-item:hover,
  .dropdown-item.highlighted {
    background-color: #f0f0f0;
  }

  .no-options {
    padding: 10px;
    color: #888;
    text-align: center;
  }
</style>
