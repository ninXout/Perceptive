<script lang="ts">
    import { onMount } from "svelte";
    import { open } from "@tauri-apps/api/dialog";
    import { invoke } from "@tauri-apps/api/tauri";

    let myModal: HTMLDivElement;
    let modalConfirm: HTMLButtonElement;
    let selectedFolder: boolean = true; // change to false before pushing

    const fileThing = async () => {
        const path = await open({
            multiple: false,
            directory: true,
        });
        if (typeof path == "string") {
            let agagag: string = await invoke("read_resources_folder", {
                path,
            });
            let guh = new Uint8ClampedArray(
                agagag.split(",").map(v => parseInt(v))
            );
            console.log(guh.length);
            selectedFolder = true;
        }
    };

    const finishSetup = async () => {
        if (selectedFolder === true) {
            myModal.style.display = "none";
            modalConfirm.classList.remove("disabled");
        }
    };

    onMount(async () => {
        myModal.style.display = "block";
    });
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
            bind:this={modalConfirm}
            class={selectedFolder ? "" : "disabled"}
            on:click={finishSetup}>Done</button
        >
    </div>
</div>

<button on:click={() => (myModal.style.display = "block")}
    >Create new pack</button
>
