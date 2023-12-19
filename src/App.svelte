<script lang="ts">
    // import * as wasm from "../wasm-lib/pkg/wasm_lib";
    // alert(wasm.test(5));
    
    import Menu from "./Menu.svelte";

    enum Page {
        HOME,
        MY_PACKS,
    }

    let page = Page.HOME;

    let bodyContents;
</script>

<div class="sidenav">
    <a on:click={() => (page = Page.HOME)}>Home</a>
    <a on:click={() => (page = Page.MY_PACKS)}>My Projects</a>
</div>

<body bind:this={bodyContents}>
    <h1>Perceptive Texture Pack Editor</h1>
    {#if page == Page.HOME}
        <Menu></Menu>
    {:else if page == Page.MY_PACKS}
        <div id="selection">
            <h2>My Packs</h2>
            <div data-category="packs">
                <div style="display: flex; min-height: 160px; width: 100%;">
                    <button>
                        <div style="'backdrop-filter: none; background: #0009'">
                            <span>
                                <h4>uhhh this one texture pack you have</h4>
                                <p></p>
                            </span>
                        </div>
                    </button>
                </div>
            </div>
        </div>
    {/if}
</body>
