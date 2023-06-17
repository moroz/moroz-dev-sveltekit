<script lang="ts">
	import type { BasicPostData } from "@api/interfaces";
	import BlogEntry from "./BlogEntry.svelte";

	export let posts: BasicPostData[];
	export let totalPages: number;
</script>

<div class="container">
	<h2>Blog</h2>
	{#each posts as post}
		<BlogEntry {post} />
	{/each}
	{totalPages}
</div>
