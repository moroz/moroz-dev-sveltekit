<script lang="ts">
	import "@css/globals.sass";
	import Header from "@components/layout/Header.svelte";
</script>

<div class="layout">
	<Header />
	<slot />
</div>

<style lang="scss">
	@import "@css/variables";

	.layout {
		min-height: 100vh;
	}
</style>
