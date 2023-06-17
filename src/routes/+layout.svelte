<script lang="ts">
	import { page } from "$app/stores";
	import "@css/globals.sass";
	import Header from "@components/layout/Header.svelte";
	import Footer from "@components/layout/Footer.svelte";

	const DEFAULT_TITLE = "moroz.dev";
</script>

<svelte:head>
	<title>{$page.data.title ? `${$page.data.title} | ` : ""}{DEFAULT_TITLE}</title>
</svelte:head>
<div class="layout">
	<Header />
	<main>
		<slot />
	</main>
	<Footer />
</div>

<style lang="scss">
	@import "@css/variables";

	.layout {
		min-height: 100vh;
		display: flex;
		flex-direction: column;
	}

	main {
		padding-top: $header-height;
		flex: 1;
	}
</style>
