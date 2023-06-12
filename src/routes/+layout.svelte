<script lang="ts">
	import "@css/globals.sass";
	import Header from "@components/layout/Header.svelte";
	import Footer from "@components/layout/Footer.svelte";
</script>

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
		flex: 1;
	}
</style>
