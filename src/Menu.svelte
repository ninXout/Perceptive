<script lang="ts">
    import { onMount } from "svelte";
    import { open } from "@tauri-apps/api/dialog";
    import { invoke } from "@tauri-apps/api/tauri";

    let myModal: HTMLDivElement;

    const fileThing = async () => {
        const path = await open({
            multiple: false,
            directory: true,
        });
        if (typeof path == "string") {
            invoke("read_resources_folder", { path });
        }
    };
</script>

<div class="modal" bind:this={myModal}>
    <div class="modal-content">
        <span class="close"></span>
        <h2>Welcome to Perceptive!</h2>
        <h3>An all-in-one texture pack editor and splitter!</h3>
        <p>
            To begin, Place the Geometry Dash resource files in the Base
            Textures folder
        </p>
        <button on:click={fileThing}>Select Folder</button>
        <button
            id="modalConfirm"
            on:click={() => (myModal.style.display = "none")}>Done</button
        >
    </div>
</div>

<button on:click={() => (myModal.style.display = "block")}
    >Create new pack</button
>
