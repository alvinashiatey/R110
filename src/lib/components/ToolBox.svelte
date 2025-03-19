<script lang="ts">
  import { selectImage, submitProcessData } from "@lib/actions/image";
  import Select from "@ui/Select.svelte";
  import Colors from "@ui/Colors.svelte";
  import { useStore, ImageFilter, ImageEffect } from "@store/useStore.svelte";
  import {
    FileArrowUp,
    Swatches,
    MagicWand,
    Sparkle,
    Files,
  } from "phosphor-svelte";

  let isProcessActive = $state(false);

  $effect(() => {
    isProcessActive =
      (useStore.processState.colors.length > 0 ||
        useStore.processState.effect !== ImageEffect.Original ||
        useStore.processState.filter) &&
      useStore.imageData !== null
        ? true
        : false;
  });

  const effectOptions = Object.entries(ImageEffect).map(([label, value]) => ({
    label,
    value,
  }));
</script>

<div class="toolbox">
  <section class="tools">
    <article class="bar">
      <button class="upload btn" id="upload-btn" onclick={() => selectImage()}>
        <FileArrowUp size="1.25rem" />
        <p>Upload image</p>
      </button>
      <div class="clr-mix btn">
        <Select
          options={[
            { label: "1", value: "1" },
            { label: "2", value: "2" },
            { label: "3", value: "3" },
            { label: "4", value: "4" },
          ]}
          value={useStore.processState.maxColors.toString()}
          placeholder="Number of colors"
          name="mix"
          required={true}
          disabled={false}
          valueChanged={(v) => useStore.setMaxColors(v)}
        >
          {#snippet myIcon()}
            <Swatches size="1.25rem" />
          {/snippet}
        </Select>
      </div>
    </article>

    <article>
      <Colors
        value={useStore.processState.colors}
        name="colors"
        required={true}
        disabled={false}
        max={useStore.processState.maxColors}
        valueChanged={(v) => useStore.addColor(v)}
      />
    </article>

    <article class="bar">
      <div class="effects btn">
        <Select
          options={effectOptions}
          value={useStore.processState.effect}
          placeholder="Select effect"
          name="effect"
          required={true}
          disabled={false}
          valueChanged={(v) => useStore.setEffect(v)}
        >
          {#snippet myIcon()}
            <MagicWand size="1.25rem" />
          {/snippet}
        </Select>
      </div>

      <div class="filter">
        <button
          class:selected={useStore.processState.shouldFilter}
          id="filter-btn"
          onclick={() => useStore.toggleFilter()}
        >
          <Sparkle size="1.25rem" />
        </button>
      </div>

      <div class="export-type btn">
        <Select
          options={[
            { label: "PDF", value: "0" },
            { label: "PNG", value: "1" },
          ]}
          value={useStore.exportState.exportType.toString()}
          placeholder="Select export"
          name="export"
          required={true}
          disabled={false}
          valueChanged={(v) => useStore.setExportType(v)}
        >
          {#snippet myIcon()}
            <Files size="1.25rem" />
          {/snippet}
        </Select>
      </div>
    </article>
  </section>
  <section class="btns">
    <button
      id="process"
      class:active={isProcessActive}
      onclick={() => submitProcessData()}>Process</button
    >
    <button id="export"> Export </button>
  </section>
</div>

<style>
  .toolbox {
    position: absolute;
    bottom: 0;
    left: 0;
    height: var(--bottom-height);
    width: 100%;
  }

  .tools {
    padding-inline: 1rem;
    display: flex;
    flex-direction: column;
    justify-content: space-around;
    height: 75%;
    padding-block: 0.5rem;
  }

  .btns {
    display: flex;
    justify-content: space-around;
    align-items: center;
    height: 25%;
    border-top: 2px solid #000000;
  }

  .btns button {
    width: 100%;
    height: 100%;
    font-weight: bold;
  }

  .btns button:first-child {
    border-right: 2px solid #000000;
    color: tomato;
  }

  .btn {
    display: flex;
    align-items: center;
    gap: 0.5rem;
    cursor: pointer;
  }

  .btn p {
    margin: 0;
    padding: 0;
    text-wrap: nowrap;
  }

  #filter-btn {
    height: 100%;
    cursor: pointer;
    transition: all 0.2s cubic-bezier(0.55, 0.055, 0.675, 0.19);
  }

  #filter-btn.selected {
    color: rebeccapurple;
    scale: 1.2;
    filter: invert(1);
  }

  .bar {
    display: flex;
    justify-content: space-between;
    /* display: grid; */
    /* grid-template-columns: repeat(3, minmax(8ch, 1fr)); */
  }

  article.bar:last-of-type > * {
    flex: 1;
    width: calc(100% / 3);
  }

  .btns button.active {
    background-color: tomato;
    color: white;
    cursor: pointer;
  }
  .btns button.active:hover {
    /* darken background */
    filter: brightness(0.9);
  }
</style>
